use std::borrow::Cow;
use std::fs::File;
use std::io::Write;
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use russh::keys::*;
use russh::*;
use russh_sftp::client::SftpSession;
use russh_sftp::protocol::OpenFlags;
use std::fs;
use std::str;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::ToSocketAddrs;
use tokio::runtime::Handle;

use crate::tools::macros::tr;

struct SshClient {}

impl client::Handler for SshClient {
    type Error = russh::Error;

    async fn check_server_key(&mut self, _server_public_key: &ssh_key::PublicKey) -> Result<bool, Self::Error> {
        Ok(true)
    }
}

pub struct SshSession {
    session: client::Handle<SshClient>,
    user: String,
    is_listen: bool,
}

impl SshSession {
    pub fn connect_key(
        key_path: &PathBuf,
        user: &String,
        host: &String,
        port: u16,
        timeout: Option<u64>,
        connect_timeout: Option<u64>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        tokio::task::block_in_place(|| {
            Handle::current().block_on(Self::_connect(
                Some(key_path.clone()),
                None,
                user.clone(),
                (host.clone(), port),
                timeout,
                connect_timeout,
            ))
        })
    }

    pub fn connect_pass(
        password: &String,
        user: &String,
        host: &String,
        port: u16,
        timeout: Option<u64>,
        connect_timeout: Option<u64>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        tokio::task::block_in_place(|| {
            Handle::current().block_on(Self::_connect(
                None,
                Some(password.clone()),
                user.clone(),
                (host.clone(), port),
                timeout,
                connect_timeout,
            ))
        })
    }

    async fn _connect<T: ToSocketAddrs>(
        key_path: Option<PathBuf>,
        password: Option<String>,
        user: String,
        addrs: T,
        timeout: Option<u64>,
        connect_timeout: Option<u64>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let config = client::Config {
            inactivity_timeout: if let Some(timeout) = timeout {
                Some(Duration::from_secs(timeout))
            } else {
                None
            },
            preferred: Preferred {
                kex: Cow::Owned(vec![
                    russh::kex::CURVE25519_PRE_RFC_8731,
                    russh::kex::EXTENSION_SUPPORT_AS_CLIENT,
                ]),
                ..Default::default()
            },
            ..<_>::default()
        };
        let config = Arc::new(config);
        let sh: SshClient = SshClient {};
        let connect_timeout = match connect_timeout {
            Some(value) => value,
            None => 600, // 10m
        };
        let result =
            tokio::time::timeout(Duration::from_secs(connect_timeout), client::connect(config, addrs, sh)).await?;
        if result.is_err() {
            Err("не удалось соединиться")?;
        }
        let mut session = result.unwrap();
        let auth_res = if let Some(key_path) = key_path {
            let key_pair = load_secret_key(key_path, None)?;
            session
                .authenticate_publickey(
                    user.clone(),
                    PrivateKeyWithHashAlg::new(
                        Arc::new(key_pair),
                        session.best_supported_rsa_hash().await.unwrap().flatten(),
                    ),
                )
                .await?
        } else {
            session.authenticate_password(user.clone(), password.unwrap()).await?
        };
        if !auth_res.success() {
            Err(tr!("ошибка подключения по ssh"))?
        }
        Ok(Self {
            session,
            user,
            is_listen: timeout.is_none(),
        })
    }

    pub fn call(&self, command: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        tokio::task::block_in_place(|| Handle::current().block_on(self._call(command)))
    }

    pub async fn _call(&self, command: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut code = None;
        let mut response: Vec<String> = vec![];
        let mut channel = self.session.channel_open_session().await?;
        channel.exec(true, command).await?;
        loop {
            let Some(msg) = channel.wait().await else {
                break;
            };
            match msg {
                ChannelMsg::Data { ref data } => {
                    match str::from_utf8(data.as_ref()) {
                        Ok(out_line) => response.push(out_line.into()),
                        Err(_) => Err(tr!("не удалось обработать данные ssh соединения"))?,
                    };
                }
                ChannelMsg::ExitStatus { exit_status } => {
                    code = Some(exit_status);
                }
                _ => {}
            }
        }
        if let Some(code) = code {
            if code == 1 {
                Err(tr!("произошла ошибка при выполнении команды"))?
            }
        }
        Ok(response)
    }

    pub fn run(&self, command: &str) -> Result<(), Box<dyn std::error::Error>> {
        tokio::task::block_in_place(|| Handle::current().block_on(self._run(command)))
    }

    async fn _run(&self, command: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut code = None;
        let mut channel = self.session.channel_open_session().await?;
        channel.exec(true, command).await?;
        loop {
            let Some(msg) = channel.wait().await else {
                break;
            };
            match msg {
                ChannelMsg::Data { ref data } => {
                    if self.is_listen {
                        match str::from_utf8(data.as_ref()) {
                            Ok(out_line) => {
                                if !out_line.is_empty() {
                                    println!("{}", out_line.trim_matches('\n'))
                                }
                            }
                            Err(_) => Err(tr!("не удалось обработать данные ssh соединения"))?,
                        };
                    }
                }
                ChannelMsg::ExitStatus { exit_status } => {
                    code = Some(exit_status);
                }
                _ => {}
            }
        }
        if let Some(code) = code {
            if code == 1 {
                Err(tr!("произошла ошибка при выполнении команды"))?
            }
        }
        Ok(())
    }

    pub async fn download(&self, path_local: &PathBuf, path_remote: &String) -> Result<(), Box<dyn std::error::Error>> {
        // Get connect
        let channel = self.session.channel_open_session().await?;
        channel.request_subsystem(true, "sftp").await.unwrap();
        let sftp = SftpSession::new(channel.into_stream()).await.unwrap();
        // let path_remote = "/home/defaultuser/Downloads/Screenshot_1745909213076.png".to_string();
        let mut sftp_file = sftp.open_with_flags(path_remote.clone(), OpenFlags::READ).await?;
        // Create file
        let mut file = File::create(path_local)?;
        // Download data
        let mut buffer = [0u8; 4096];
        loop {
            let n = sftp_file.read(&mut buffer).await?;
            if n == 0 {
                break;
            }
            file.write_all(&buffer[..n])?;
        }
        Ok(())
    }

    pub async fn upload<F: Fn(i32) + Send + Copy + Sync + 'static>(
        &self,
        path: &PathBuf,
        state: F,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Get name
        let file_name = match path.file_name() {
            Some(name) => format!("Downloads/{}", name.to_string_lossy()),
            None => Err("error load file name")?,
        };
        // Get data
        let file = File::open(path)?;
        let size = file.metadata()?.size();
        if size == 0 {
            Err(tr!("файл не содержит данных"))?
        }
        // Get connect
        let channel = self.session.channel_open_session().await?;
        channel.request_subsystem(true, "sftp").await.unwrap();
        let sftp = SftpSession::new(channel.into_stream()).await.unwrap();
        // Open file
        let mut sftp_file = sftp
            .open_with_flags(
                file_name.clone(),
                OpenFlags::CREATE | OpenFlags::TRUNCATE | OpenFlags::WRITE | OpenFlags::READ,
            )
            .await?;
        // Write data
        let mut progress = 0;
        let size = file.metadata()?.size();
        if size < 100 {
            state(progress);
            let data = fs::read(path)?;
            sftp_file.write_all(&data).await.unwrap();
        } else {
            let chunk = file.metadata()?.size() / 100;
            for data in fs::read(path)?.chunks(chunk as usize) {
                if progress < 100 {
                    state(progress);
                }
                sftp_file.write_all(data).await.unwrap();
                progress += 1;
            }
        }
        state(100);
        Ok(format!("/home/{}/{}", self.user, file_name))
    }

    pub async fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.session.disconnect(Disconnect::ByApplication, "", "ru_RU").await?;
        Ok(())
    }
}
