use std::borrow::Cow;
use std::fs::File;
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use russh::keys::*;
use russh::*;
use russh_sftp::client::SftpSession;
use russh_sftp::protocol::OpenFlags;
use std::fs;
use std::str;
use tokio::io::AsyncWriteExt;
use tokio::net::ToSocketAddrs;
use tokio::runtime::Handle;

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
    pub fn connect<P: AsRef<Path>, A: ToSocketAddrs>(
        key_path: P,
        user: impl Into<String> + Copy,
        addrs: A,
        timeout: Option<u64>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        tokio::task::block_in_place(|| Handle::current().block_on(Self::_connect(key_path, user, addrs, timeout)))
    }

    async fn _connect<P: AsRef<Path>, A: ToSocketAddrs>(
        key_path: P,
        user: impl Into<String> + Copy,
        addrs: A,
        timeout: Option<u64>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let key_pair = load_secret_key(key_path, None)?;
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
        let sh = SshClient {};
        let mut session = client::connect(config, addrs, sh).await?;
        let auth_res = session
            .authenticate_publickey(
                user.into(),
                PrivateKeyWithHashAlg::new(
                    Arc::new(key_pair),
                    session.best_supported_rsa_hash().await.unwrap().flatten(),
                ),
            )
            .await?;

        if !auth_res.success() {
            Err("ошибка подключения по ssh")?
        }

        Ok(Self {
            session,
            user: user.into(),
            is_listen: timeout.is_none(),
        })
    }

    pub fn call(&self, command: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        tokio::task::block_in_place(|| Handle::current().block_on(self._call(command)))
    }

    async fn _call(&self, command: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
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
                        Err(_) => Err("не удалось обработать данные ssh соединения")?,
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
                Err("произошла ошибка при выполнении команды")?
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
                            Err(_) => Err("не удалось обработать данные ssh соединения")?,
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
                Err("произошла ошибка при выполнении команды")?
            }
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
            Err("файл не содержит данных")?
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
            .await
            .unwrap();
        // Write data
        let mut progress = 0;
        let chunk = file.metadata()?.size() / 100;
        for data in fs::read(path)?.chunks(chunk as usize) {
            state(progress);
            sftp_file.write_all(data).await.unwrap();
            progress += 1;
        }
        Ok(format!("/home/{}/{}", self.user, file_name))
    }

    pub async fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.session.disconnect(Disconnect::ByApplication, "", "ru_RU").await?;
        Ok(())
    }
}
