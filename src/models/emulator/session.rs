use std::path::PathBuf;

use crate::{service::ssh::client::SshSession, utils::methods};

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
    pub async fn new(
        session_type: EmulatorSessionType,
        key: &String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let host = "localhost";
        let user = if session_type == EmulatorSessionType::Root {
            "root"
        } else {
            "defaultuser"
        };
        let port = 2223;
        let session = SshSession::connect(PathBuf::from(key), user, (host, port)).await?;
        let output = session.call("cat /etc/os-release").await?;
        let lines = match output.first() {
            Some(s) => s
                .split("\n")
                .map(|e| e.to_string())
                .collect::<Vec<String>>(),
            None => Err("ошибка при получении данных")?,
        };
        let os_name = match methods::config_get_string(&lines, "PRETTY_NAME", "=") {
            Ok(s) => s,
            Err(error) => Err(error)?,
        };
        let os_version = match methods::config_get_string(&lines, "VERSION_ID", "=") {
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

    pub async fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.session.close().await?;
        Ok(())
    }
}
