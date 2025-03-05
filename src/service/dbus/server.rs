use std::sync::Arc;

use dbus::Message;
use dbus::Path;
use dbus::channel::MatchingReceiver;
use dbus::channel::Sender;
use dbus::message::MatchRule;
use dbus::nonblock::SyncConnection;
use dbus_crossroads::Crossroads;
use dbus_crossroads::IfaceBuilder;
use dbus_tokio::connection;
use futures::future;
use tokio::runtime::Handle;

use crate::models::client::app_info::incoming::AppInfoIncoming;
use crate::models::client::emulator_close::incoming::EmulatorCloseIncoming;
use crate::models::client::emulator_info::incoming::EmulatorInfoIncoming;
use crate::models::client::emulator_open::incoming::EmulatorOpenIncoming;
use crate::models::client::emulator_package_install::incoming::EmulatorPackageInstallIncoming;
use crate::models::client::emulator_package_run::incoming::EmulatorPackageRunIncoming;
use crate::models::client::emulator_package_uninstall::incoming::EmulatorPackageUninstallIncoming;
use crate::models::client::emulator_record_start::incoming::EmulatorRecordStartIncoming;
use crate::models::client::emulator_record_stop::incoming::EmulatorRecordStopIncoming;
use crate::models::client::emulator_screenshot::incoming::EmulatorScreenshotIncoming;
use crate::models::client::emulator_sync::incoming::EmulatorSyncIncoming;
use crate::models::client::emulator_terminal::incoming::EmulatorTerminalIncoming;
use crate::models::client::emulator_upload::incoming::EmulatorUploadIncoming;
use crate::models::client::flutter_available::incoming::FlutterAvailableIncoming;
use crate::models::client::flutter_download::incoming::FlutterDownloadIncoming;
use crate::models::client::flutter_info::incoming::FlutterInfoIncoming;
use crate::models::client::flutter_sync::incoming::FlutterSyncIncoming;
use crate::models::client::flutter_terminal::incoming::FlutterTerminalIncoming;
use crate::models::client::psdk_available::incoming::PsdkAvailableIncoming;
use crate::models::client::psdk_download::incoming::PsdkDownloadIncoming;
use crate::models::client::psdk_info::incoming::PsdkInfoIncoming;
use crate::models::client::psdk_sync::incoming::PsdkSyncIncoming;
use crate::models::client::psdk_terminal::incoming::PsdkTerminalIncoming;
use crate::models::client::sdk_available::incoming::SdkAvailableIncoming;
use crate::models::client::sdk_download::incoming::SdkDownloadIncoming;
use crate::models::client::sdk_info::incoming::SdkInfoIncoming;
use crate::models::client::sdk_sync::incoming::SdkSyncIncoming;
use crate::models::client::sdk_tools::incoming::SdkToolsIncoming;
use crate::tools::constants;
use crate::tools::macros::print_success;
use crate::tools::single;

// gdbus call --timeout=99999 --session --dest com.keygenqt.aurora_bot --object-path /api --method com.keygenqt.aurora_bot.{KEY}
// gdbus monitor --session --dest com.keygenqt.aurora_bot --object-path /api com.keygenqt.aurora_bot.listen

pub struct IfaceData {}

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
        let signal_state = cr.register(constants::DBUS_NAME, |builder| {
            /////////////////
            // Signals
            ServerDbus::add_signal("listen", builder);

            /////////////////
            // App
            AppInfoIncoming::dbus_method_run(builder);

            /////////////////
            // Emulator
            EmulatorCloseIncoming::dbus_method_run(builder);
            EmulatorCloseIncoming::dbus_method_run_by_id(builder);

            EmulatorInfoIncoming::dbus_method_run(builder);
            EmulatorInfoIncoming::dbus_method_run_by_id(builder);

            EmulatorOpenIncoming::dbus_method_run(builder);
            EmulatorOpenIncoming::dbus_method_run_by_id(builder);
            EmulatorOpenIncoming::dbus_method_run_vnc(builder);
            EmulatorOpenIncoming::dbus_method_run_vnc_by_id(builder);

            EmulatorPackageInstallIncoming::dbus_method_run_path(builder);
            EmulatorPackageInstallIncoming::dbus_method_run_path_by_id(builder);

            EmulatorPackageRunIncoming::dbus_method_run(builder);
            EmulatorPackageRunIncoming::dbus_method_run_by_id(builder);
            EmulatorPackageRunIncoming::dbus_method_run_by_package(builder);
            EmulatorPackageRunIncoming::dbus_method_run_by_id_package(builder);

            EmulatorPackageUninstallIncoming::dbus_method_run(builder);
            EmulatorPackageUninstallIncoming::dbus_method_run_by_id(builder);
            EmulatorPackageUninstallIncoming::dbus_method_run_by_package(builder);
            EmulatorPackageUninstallIncoming::dbus_method_run_by_id_package(builder);

            EmulatorRecordStartIncoming::dbus_method_run(builder);
            EmulatorRecordStartIncoming::dbus_method_run_by_id(builder);

            EmulatorRecordStopIncoming::dbus_method_run(builder);
            EmulatorRecordStopIncoming::dbus_method_run_by_id(builder);

            EmulatorScreenshotIncoming::dbus_method_run(builder);
            EmulatorScreenshotIncoming::dbus_method_run_by_id(builder);

            EmulatorSyncIncoming::dbus_method_run(builder);

            EmulatorTerminalIncoming::dbus_method_run(builder);
            EmulatorTerminalIncoming::dbus_method_run_by_id(builder);

            EmulatorUploadIncoming::dbus_method_run_path(builder);
            EmulatorUploadIncoming::dbus_method_run_path_by_id(builder);
            EmulatorUploadIncoming::dbus_method_run_url(builder);
            EmulatorUploadIncoming::dbus_method_run_url_by_id(builder);

            /////////////////
            // Flutter
            FlutterAvailableIncoming::dbus_method_run(builder);
            FlutterAvailableIncoming::dbus_method_run_by_id(builder);

            FlutterDownloadIncoming::dbus_method_run(builder);
            FlutterDownloadIncoming::dbus_method_run_by_id(builder);

            FlutterInfoIncoming::dbus_method_run(builder);
            FlutterInfoIncoming::dbus_method_run_by_id(builder);

            FlutterSyncIncoming::dbus_method_run(builder);

            FlutterTerminalIncoming::dbus_method_run(builder);
            FlutterTerminalIncoming::dbus_method_run_by_id(builder);

            /////////////////
            // Psdk
            PsdkAvailableIncoming::dbus_method_run(builder);
            PsdkAvailableIncoming::dbus_method_run_by_id(builder);

            PsdkDownloadIncoming::dbus_method_run(builder);
            PsdkDownloadIncoming::dbus_method_run_by_id(builder);

            PsdkInfoIncoming::dbus_method_run(builder);
            PsdkInfoIncoming::dbus_method_run_by_id(builder);

            PsdkSyncIncoming::dbus_method_run(builder);

            PsdkTerminalIncoming::dbus_method_run(builder);
            PsdkTerminalIncoming::dbus_method_run_by_id(builder);

            /////////////////
            // Sdk
            SdkAvailableIncoming::dbus_method_run(builder);
            SdkAvailableIncoming::dbus_method_run_by_id(builder);

            SdkDownloadIncoming::dbus_method_run(builder);
            SdkDownloadIncoming::dbus_method_run_by_id(builder);

            SdkInfoIncoming::dbus_method_run(builder);
            SdkInfoIncoming::dbus_method_run_by_id(builder);

            SdkSyncIncoming::dbus_method_run(builder);

            SdkToolsIncoming::dbus_method_run(builder);
            SdkToolsIncoming::dbus_method_run_by_id(builder);
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

    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        tokio::task::block_in_place(|| {
            Handle::current().block_on(single::get_dbus().connection.request_name(
                constants::DBUS_NAME,
                false,
                true,
                false,
            ))
        })?;
        print_success!("Сервис D-Bus запущен!");
        tokio::task::block_in_place(|| Handle::current().block_on(future::pending::<()>()));
        unreachable!()
    }

    pub fn send(outgoing: String) {
        let path: Path<'static> = format!("{}", "/api").into();
        let msg = Message::signal(&path, &constants::DBUS_NAME.into(), &"listen".into()).append1(outgoing);
        let _ = single::get_dbus().connection.send(msg);
    }

    fn add_signal(name: &str, builder: &mut IfaceBuilder<IfaceData>) {
        builder.signal::<(String,), _>(String::from(name), ("sender",));
    }
}
