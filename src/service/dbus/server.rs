use colored::Colorize;
use std::sync::Arc;

use dbus::channel::{MatchingReceiver, Sender};
use dbus::message::MatchRule;
use dbus::nonblock::SyncConnection;
use dbus::{Message, Path};
use dbus_crossroads::{Crossroads, IfaceBuilder};
use dbus_tokio::connection;
use futures::future;

use crate::app::api::enums::SendType;
use crate::utils::single::get_dbus;
use crate::{
    app::api::{
        convert::convert_outgoing, handler::handler_incoming, incoming::models::Incoming,
        outgoing::models::Outgoing,
    },
    utils::constants::DBUS_NAME,
};

// gdbus call --session --dest com.keygenqt.aurora_bot --object-path /api --method
// - com.keygenqt.aurora_bot.api_info
// - com.keygenqt.aurora_bot.app_info
// - com.keygenqt.aurora_bot.emulator_start

// gdbus monitor --session --dest com.keygenqt.aurora_bot --object-path /api
// - com.keygenqt.aurora_bot.listen

struct IfaceData {}

pub struct ServerDbus {
    pub connection: Arc<SyncConnection>,
}

impl ServerDbus {
    /// Create instance
    pub fn new() -> ServerDbus {
        let mut cr = Crossroads::new();
        let (resource, connection) = connection::new_session_sync().unwrap();

        // Init tokio
        cr.set_async_support(Some((
            connection.clone(),
            Box::new(|x| {
                tokio::spawn(x);
            }),
        )));

        // Init api
        let signal_state = cr.register(DBUS_NAME, |builder| {
            // Signals
            ServerDbus::add_signal(
                "listen",
                builder
            );
            // Methods
            ServerDbus::add_method(
                "api_info",
                Incoming::api_info(),
                builder
            );
            ServerDbus::add_method(
                "app_info",
                Incoming::app_info(),
                builder
            );
            ServerDbus::add_method(
                "emulator_start",
                Incoming::emulator_start(),
                builder
            );
        });

        // Add api
        cr.insert("/api", &[signal_state], IfaceData {});

        // Init listen methods
        connection.start_receive(
            MatchRule::new_method_call(),
            Box::new(move |msg: dbus::Message, conn| {
                cr.handle_message(msg, conn).unwrap();
                true
            }),
        );
        let _handle = tokio::spawn(async {
            let err = resource.await;
            panic!("Lost connection to D-Bus: {}", err);
        });

        return ServerDbus { connection };
    }

    pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
        get_dbus()
            .unwrap()
            .connection
            .request_name(DBUS_NAME, false, true, false)
            .await?;
        println!("{}", "Сервис D-Bus успешно запущен!".green().bold());
        future::pending::<()>().await;
        unreachable!()
    }

    pub fn send(outgoing: &Outgoing) {
        let outgoing = convert_outgoing(&outgoing);
        if outgoing.is_ok() {
            let path: Path<'static> = format!("{}", "/api").into();
            let msg = Message::signal(&path, &DBUS_NAME.into(), &"listen".into())
                .append1(outgoing.unwrap());
            let _ = get_dbus().unwrap().connection.send(msg);
        }
    }

    fn add_signal(
        name: &str,
        builder: &mut IfaceBuilder<IfaceData>
    ) {
        builder.signal::<(String,), _>(String::from(name), ("sender",));
    }

    fn add_method(
        name: &str,
        incoming: Incoming,
        builder: &mut IfaceBuilder<IfaceData>
    ) {
        builder.method_with_cr_async(
            String::from(name),
            (),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, _: ()| {
                let value = incoming.clone();
                async move {
                    let result = match handler_incoming(&value, SendType::Dbus).await {
                        Ok(outgoing) => Ok((convert_outgoing(&outgoing).unwrap(),)),
                        Err(_) => Ok((convert_outgoing(&Outgoing::error(
                            "An error occurred while executing".into(),
                        ))
                        .unwrap(),)),
                    };
                    ctx.reply(result)
                }
            },
        );
    }
}
