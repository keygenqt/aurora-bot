use std::borrow::Cow;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use russh::keys::*;
use russh::*;
use std::str;
use tokio::net::ToSocketAddrs;

struct SshClient {}

impl client::Handler for SshClient {
    type Error = russh::Error;

    async fn check_server_key(
        &mut self,
        _server_public_key: &ssh_key::PublicKey,
    ) -> Result<bool, Self::Error> {
        Ok(true)
    }
}

pub struct SshSession {
    session: client::Handle<SshClient>,
}

impl SshSession {
    pub async fn connect<P: AsRef<Path>, A: ToSocketAddrs>(
        key_path: P,
        user: impl Into<String>,
        addrs: A,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let key_pair = load_secret_key(key_path, None)?;
        let config = client::Config {
            inactivity_timeout: Some(Duration::from_secs(5)),
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
                user,
                PrivateKeyWithHashAlg::new(
                    Arc::new(key_pair),
                    session.best_supported_rsa_hash().await.unwrap().flatten(),
                ),
            )
            .await?;

        if !auth_res.success() {
            Err("ошибка подключения по ssh")?
        }

        Ok(Self { session })
    }

    pub async fn call(&self, command: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
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
        if code.is_none() {
            Err("произошла ошибка при выполнении команды")?
        }
        Ok(response)
    }

    pub async fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.session
            .disconnect(Disconnect::ByApplication, "", "ru_RU")
            .await?;
        Ok(())
    }
}
