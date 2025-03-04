use std::path::PathBuf;

use tokio::runtime::Handle;

use crate::service::ssh::client::SshSession;
use crate::tools::utils;

#[derive(PartialEq)]
pub enum EmulatorSessionType {
    Root,
    User,
}

#[allow(dead_code)]
pub struct EmulatorSession {
    pub host: String,
    pub user: String,
    pub port: u16,
    pub os_name: String,
    pub os_version: String,
    session: SshSession,
}

impl EmulatorSession {
    pub fn new(session_type: EmulatorSessionType, key: &String) -> Result<Self, Box<dyn std::error::Error>> {
        let host = "localhost";
        let user = if session_type == EmulatorSessionType::Root {
            "root"
        } else {
            "defaultuser"
        };
        let port = 2223;
        let session = SshSession::connect(PathBuf::from(key), user, (host, port))?;
        let output = session.call("cat /etc/os-release")?;
        let lines = match output.first() {
            Some(s) => s.split("\n").map(|e| e.to_string()).collect::<Vec<String>>(),
            None => Err("ошибка при получении данных")?,
        };
        let os_name = match utils::config_get_string(&lines, "PRETTY_NAME", "=") {
            Ok(s) => s,
            Err(error) => Err(error)?,
        };
        let os_version = match utils::config_get_string(&lines, "VERSION_ID", "=") {
            Ok(s) => s,
            Err(error) => Err(error)?,
        };
        Ok(EmulatorSession {
            host: host.to_string(),
            user: user.to_string(),
            port,
            os_name,
            os_version,
            session,
        })
    }

    pub fn file_upload<F: Fn(i32) + Send + Copy + Sync + 'static>(
        &self,
        path: &PathBuf,
        state: F,
    ) -> Result<(), Box<dyn std::error::Error>> {
        tokio::task::block_in_place(|| Handle::current().block_on(self.session.upload(path, state)))?;
        Ok(())
    }

    pub fn get_install_packages(&self) -> Vec<String> {
        match self.session.call("find /usr/bin -name '*.*.*'") {
            Ok(value) => if let Some(line) = value.first() {
                line.split("\n").filter(|e| !e.is_empty() && !e.contains("perl")).map(|e| e.replace("/usr/bin/", "")).collect()
            } else {
                vec![]
            },
            Err(_) => vec![],
        }
    }

    pub fn run_package(&self, package: String) -> Result<(), Box<dyn std::error::Error>> {
        self.session.run(&format!("invoker --type=qt5 {package}"))?;
        Ok(())
    }

    pub fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        tokio::task::block_in_place(|| Handle::current().block_on(self.session.close()))?;
        Ok(())
    }
}
