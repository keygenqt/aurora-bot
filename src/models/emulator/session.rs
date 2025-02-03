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
        vm_folder: &String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let host = "localhost";
        let user = if session_type == EmulatorSessionType::Root {
            "root"
        } else {
            "defaultuser"
        };
        let port = 2223;
        // Connect
        let session = SshSession::connect(
            PathBuf::from(format!("{}/vmshare/ssh/private_keys/sdk", vm_folder)),
            user,
            (host, port),
        )
        .await?;
        // Get info emulator
        let output = session.call("cat /etc/os-release").await?;
        let key_name = "PRETTY_NAME";
        let key_version = "VERSION_ID";
        let params = methods::config_vec_filter_keys(output, [key_name, key_version])?;
        let os_name = methods::config_get_string(&params, key_name, "=")?;
        let os_version = methods::config_get_string(&params, key_version, "=")?;
        // Create emulator session
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
