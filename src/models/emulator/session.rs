use std::path::PathBuf;

use tokio::runtime::Handle;

use crate::service::ssh::client::SshSession;
use crate::tools::utils;

#[allow(dead_code)]
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

    pub fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        tokio::task::block_in_place(|| Handle::current().block_on(self.session.close()))?;
        Ok(())
    }
}
