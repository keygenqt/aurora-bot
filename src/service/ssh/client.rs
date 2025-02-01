use std::borrow::Cow;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use russh::keys::*;
use russh::*;
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

// @todo
// https://github.com/Eugeny/russh/blob/main/russh/examples/client_exec_simple.rs
impl SshSession {
    pub async fn connect<P: AsRef<Path>, A: ToSocketAddrs>(
        key_path: P,
        user: impl Into<String>,
        addrs: A,
    ) -> Result<Self> {
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
            anyhow::bail!("Authentication failed: {auth_res:?}");
        }

        Ok(Self { session })
    }

    pub async fn close(&mut self) -> Result<()> {
        self.session
            .disconnect(Disconnect::ByApplication, "", "English")
            .await?;
        Ok(())
    }
}
