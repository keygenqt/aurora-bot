use std::path::PathBuf;

use regex::Regex;
use tokio::runtime::Handle;

use crate::service::ssh::client::SshSession;
use crate::tools::macros::tr;
use crate::tools::utils;

#[derive(PartialEq)]
pub enum SessionModelType {
    Root,
    User,
}

#[allow(dead_code)]
pub struct SessionModel {
    pub host: String,
    pub user: String,
    pub port: u16,
    pub os_name: String,
    pub os_version: String,
    pub arch: String,
    session: SshSession,
    session_listen: SshSession,
}

impl SessionModel {
    pub fn new_key(
        session_type: SessionModelType,
        path: &String,
        host: &String,
        port: u16,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Self::new(session_type, Some(path.clone()), None, host.clone(), port)
    }

    pub fn new_pass(
        session_type: SessionModelType,
        pass: &String,
        host: &String,
        port: u16,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Self::new(session_type, None, Some(pass.clone()), host.clone(), port)
    }

    fn new(
        session_type: SessionModelType,
        path: Option<String>,
        pass: Option<String>,
        host: String,
        port: u16,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let user = if session_type == SessionModelType::Root {
            "root".to_string()
        } else {
            "defaultuser".to_string()
        };
        let sessions = if let Some(path) = path {
            let session = SshSession::connect_key(&PathBuf::from(&path), &user, &host, port, Some(3))?;
            let session_listen = SshSession::connect_key(&PathBuf::from(&path), &user, &host, port, None)?;
            (session, session_listen)
        } else {
            let pass = pass.unwrap();
            let session = SshSession::connect_pass(&pass, &user, &host, port, Some(3))?;
            let session_listen = SshSession::connect_pass(&pass, &user, &host, port, None)?;
            (session, session_listen)
        };
        let output = sessions.0.call("cat /etc/os-release")?;
        let lines = match output.first() {
            Some(s) => s.split("\n").map(|e| e.to_string()).collect::<Vec<String>>(),
            None => Err(tr!("ошибка при получении данных"))?,
        };
        let os_name = match utils::config_get_string(&lines, "PRETTY_NAME", "=") {
            Ok(s) => s,
            Err(error) => Err(error)?,
        };
        let os_version = match utils::config_get_string(&lines, "VERSION_ID", "=") {
            Ok(s) => s,
            Err(error) => Err(error)?,
        };
        let arch = match sessions.0.call("cat /etc/rpm/platform")?.first() {
            Some(value) => match value.split("-").next() {
                Some(value) => value.to_string(),
                None => "undefined".to_string(),
            },
            None => "undefined".to_string(),
        };
        Ok(SessionModel {
            host: host.to_string(),
            user: user.to_string(),
            port,
            os_name,
            os_version,
            arch,
            session: sessions.0,
            session_listen: sessions.1,
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
                        Err(tr!("ничего не найдено"))?
                    }
                    Ok(packages)
                } else {
                    Err(tr!("не удалось получить пакеты"))?
                }
            }
            Err(_) => Err(tr!("при запросе пакетов возникла ошибка"))?,
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
            Err(_) => Err(tr!("произошла ошибка при установке пакета"))?,
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
            Err(_) => Err(tr!("произошла ошибка при удалении пакета"))?,
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
