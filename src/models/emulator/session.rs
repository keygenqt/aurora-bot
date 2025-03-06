use std::path::PathBuf;

use regex::Regex;
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
    session_listen: SshSession,
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
        let session_listen = SshSession::connect(PathBuf::from(key), user, (host, port), None)?;
        let session = SshSession::connect(PathBuf::from(key), user, (host, port), Some(3))?;
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
            session_listen,
        })
    }

    pub fn file_upload<F: Fn(i32) + Send + Copy + Sync + 'static>(
        &self,
        path: &PathBuf,
        state: F,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let path_remote = tokio::task::block_in_place(|| Handle::current().block_on(self.session.upload(path, state)))?;
        Ok(path_remote)
    }

    pub fn get_install_packages(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let re = Regex::new(r"/usr/bin/[A-z]+.[A-z]+.[A-z]+")?;
        match self.session.call("find /usr/bin -name '*.*.*'") {
            Ok(value) => {
                if let Some(line) = value.first() {
                    let packages: Vec<String> = line
                        .split("\n")
                        .filter(|e| {
                            if e.is_empty() {
                                return false;
                            }
                            match re.captures(e) {
                                Some(_) => true,
                                None => false,
                            }
                        })
                        .map(|e| e.replace("/usr/bin/", ""))
                        .collect();
                    if packages.len() == 0 {
                        Err("ничего не найдено")?
                    }
                    Ok(packages)
                } else {
                    Err("не удалось получить пакеты")?
                }
            }
            Err(_) => Err("при запросе пакетов возникла ошибка")?,
        }
    }

    pub fn install_package(
        &self,
        path_remote: String,
        package_name: Option<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Try remove
        if let Some(package_name) = package_name {
            let _ = self.remove_package(package_name, false);
        }
        // Install
        let prompt = "\"{}\"";
        let command = format!(
            "gdbus call --system --dest ru.omp.APM --object-path /ru/omp/APM --method ru.omp.APM.Install \"{path_remote}\" {}",
            prompt
        );
        match self.session.call(&command) {
            Ok(_) => Ok(()),
            Err(_) => Err("произошла ошибка при установке пакета")?,
        }
    }

    pub fn remove_package(&self, package_name: String, is_remove_data: bool) -> Result<(), Box<dyn std::error::Error>> {
        let prompt = if is_remove_data {
            "\"{}\""
        } else {
            "\"{'KeepUserData': <true>}\""
        };
        let command = format!(
            "gdbus call --system --dest ru.omp.APM --object-path /ru/omp/APM --method ru.omp.APM.Remove \"{package_name}\" {}",
            prompt
        );
        match self.session.call(&command) {
            Ok(_) => Ok(()),
            Err(_) => Err("произошла ошибка при удалении пакета")?,
        }
    }

    pub fn run_package(&self, package: String) -> Result<(), Box<dyn std::error::Error>> {
        self.session.run(&format!("invoker --type=qt5 {package}"))?;
        Ok(())
    }

    pub fn run_package_listen(&self, package: String) -> Result<(), Box<dyn std::error::Error>> {
        self.session_listen.run(&format!("invoker --type=qt5 {package}"))?;
        Ok(())
    }

    pub fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        tokio::task::block_in_place(|| Handle::current().block_on(self.session.close()))?;
        Ok(())
    }
}
