use colored::Colorize;
use std::sync::Arc;

use dbus::channel::{MatchingReceiver, Sender};
use dbus::message::MatchRule;
use dbus::nonblock::SyncConnection;
use dbus::{Message, Path};
use dbus_crossroads::{Crossroads, IfaceBuilder};
use dbus_tokio::connection;
use futures::future;

use crate::app::api::enums::CommandType;
use crate::utils::single::get_dbus;
use crate::{
    app::api::{
        convert::convert_outgoing, handler::handler_incoming, incoming::models::Incoming,
        outgoing::models::Outgoing,
    },
    utils::constants::DBUS_NAME,
};

// gdbus call --session --dest com.keygenqt.aurora_bot --object-path /api --method com.keygenqt.aurora_bot.app_info
// gdbus call --session --dest com.keygenqt.aurora_bot --object-path /api --method com.keygenqt.aurora_bot.emulator_start
// gdbus monitor --session --dest com.keygenqt.aurora_bot --object-path /api com.keygenqt.aurora_bot.listen

struct IfaceData {}

fn add_signal_state(builder: &mut IfaceBuilder<IfaceData>) {
    builder.signal::<(String,), _>("listen", ("sender",));
}

fn add_method_app_info(builder: &mut IfaceBuilder<IfaceData>) {
    builder.method_with_cr_async(
        "app_info",
        (),
        ("result",),
        |mut ctx: dbus_crossroads::Context, _, _: ()| {
            let incoming = Incoming::app_info();
            async move {
                let result = match handler_incoming(&incoming, CommandType::Dbus, None).await {
                    Ok(outgoing) => Ok((convert_outgoing(&outgoing).unwrap(),)),
                    Err(_) => Ok((String::from(""),)),
                };
                ctx.reply(result)
            }
        },
    );
}

fn add_method_emulator_start(builder: &mut IfaceBuilder<IfaceData>) {
    builder.method_with_cr_async(
        "emulator_start",
        (),
        ("result",),
        |mut ctx: dbus_crossroads::Context, _, _: ()| {
            let incoming = Incoming::emulator_start();
            async move {
                let result = match handler_incoming(&incoming, CommandType::Dbus, None).await {
                    Ok(outgoing) => Ok((convert_outgoing(&outgoing).unwrap(),)),
                    Err(_) => Ok((String::from(""),)),
                };
                ctx.reply(result)
            }
        },
    );
}

pub struct ClientDbus {
    pub connection: Arc<SyncConnection>,
}

impl ClientDbus {
    /// Create instance
    pub fn new() -> ClientDbus {
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
            add_signal_state(builder);
            add_method_app_info(builder);
            add_method_emulator_start(builder);
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

        return ClientDbus { connection };
    }

    pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
        get_dbus()
            .unwrap()
            .connection
            .request_name(DBUS_NAME, false, true, false)
            .await?;
        println!("{}", "Сервис D-Bust успешно запущен!".green().bold());
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
}
