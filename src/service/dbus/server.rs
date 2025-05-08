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
use serde::Serialize;
use tokio::runtime::Handle;

use crate::feature::app_auth_login::incoming::AppAuthLoginIncoming;
use crate::feature::app_auth_logout::incoming::AppAuthLogoutIncoming;
use crate::feature::app_info::incoming::AppInfoIncoming;
use crate::feature::app_open_dir::incoming::AppOpenDirIncoming;
use crate::feature::app_open_file::incoming::AppOpenFileIncoming;
use crate::feature::demo_app_info::incoming::DemoAppInfoIncoming;
use crate::feature::device_info::incoming::DeviceInfoIncoming;
use crate::feature::device_package_install::incoming::DevicePackageInstallIncoming;
use crate::feature::device_package_run::incoming::DevicePackageRunIncoming;
use crate::feature::device_package_uninstall::incoming::DevicePackageUninstallIncoming;
use crate::feature::device_screenshot::incoming::DeviceScreenshotIncoming;
use crate::feature::device_sync::incoming::DeviceSyncIncoming;
use crate::feature::device_terminal::incoming::DeviceTerminalIncoming;
use crate::feature::device_upload::incoming::DeviceUploadIncoming;
use crate::feature::emulator_close::incoming::EmulatorCloseIncoming;
use crate::feature::emulator_info::incoming::EmulatorInfoIncoming;
use crate::feature::emulator_open::incoming::EmulatorOpenIncoming;
use crate::feature::emulator_package_install::incoming::EmulatorPackageInstallIncoming;
use crate::feature::emulator_package_run::incoming::EmulatorPackageRunIncoming;
use crate::feature::emulator_package_uninstall::incoming::EmulatorPackageUninstallIncoming;
use crate::feature::emulator_record_start::incoming::EmulatorRecordStartIncoming;
use crate::feature::emulator_record_stop::incoming::EmulatorRecordStopIncoming;
use crate::feature::emulator_screenshot::incoming::EmulatorScreenshotIncoming;
use crate::feature::emulator_sync::incoming::EmulatorSyncIncoming;
use crate::feature::emulator_terminal::incoming::EmulatorTerminalIncoming;
use crate::feature::emulator_upload::incoming::EmulatorUploadIncoming;
use crate::feature::flutter_available::incoming::FlutterAvailableIncoming;
use crate::feature::flutter_download::incoming::FlutterDownloadIncoming;
use crate::feature::flutter_info::incoming::FlutterInfoIncoming;
use crate::feature::flutter_install::incoming::FlutterInstallIncoming;
use crate::feature::flutter_project_format::incoming::FlutterProjectFormatIncoming;
use crate::feature::flutter_project_report::incoming::FlutterProjectReportIncoming;
use crate::feature::flutter_sync::incoming::FlutterSyncIncoming;
use crate::feature::flutter_terminal::incoming::FlutterTerminalIncoming;
use crate::feature::flutter_uninstall::incoming::FlutterUninstallIncoming;
use crate::feature::psdk_available::incoming::PsdkAvailableIncoming;
use crate::feature::psdk_download::incoming::PsdkDownloadIncoming;
use crate::feature::psdk_info::incoming::PsdkInfoIncoming;
use crate::feature::psdk_install::incoming::PsdkInstallIncoming;
use crate::feature::psdk_package_sign::incoming::PsdkPackageSignIncoming;
use crate::feature::psdk_sync::incoming::PsdkSyncIncoming;
use crate::feature::psdk_target_package_find::incoming::PsdkTargetPackageFindIncoming;
use crate::feature::psdk_target_package_install::incoming::PsdkTargetPackageInstallIncoming;
use crate::feature::psdk_target_package_uninstall::incoming::PsdkTargetPackageUninstallIncoming;
use crate::feature::psdk_terminal::incoming::PsdkTerminalIncoming;
use crate::feature::psdk_uninstall::incoming::PsdkUninstallIncoming;
use crate::feature::sdk_available::incoming::SdkAvailableIncoming;
use crate::feature::sdk_download::incoming::SdkDownloadIncoming;
use crate::feature::sdk_ide_close::incoming::SdkIdeCloseIncoming;
use crate::feature::sdk_ide_open::incoming::SdkIdeOpenIncoming;
use crate::feature::sdk_info::incoming::SdkInfoIncoming;
use crate::feature::sdk_install::incoming::SdkInstallIncoming;
use crate::feature::sdk_project_format::incoming::SdkProjectFormatIncoming;
use crate::feature::sdk_sync::incoming::SdkSyncIncoming;
use crate::feature::sdk_tools::incoming::SdkToolsIncoming;
use crate::feature::sdk_uninstall::incoming::SdkUninstallIncoming;
use crate::service::dbus::methods;
use crate::tools::constants;
use crate::tools::macros::print_success;
use crate::tools::single;

// gdbus call --timeout=99999 --session --dest com.keygenqt.aurora_bot --object-path /api --method com.keygenqt.aurora_bot.{KEY}
// gdbus monitor --session --dest com.keygenqt.aurora_bot --object-path /api com.keygenqt.aurora_bot.listen

/// Common state client
#[derive(Serialize, Clone)]
pub enum DbusOnly {
    FaqSearch,
    CanYouCPlusPlusDoThat,
}

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
            AppAuthLoginIncoming::dbus_method_run(builder);
            AppAuthLogoutIncoming::dbus_method_run(builder);
            AppInfoIncoming::dbus_method_run(builder);
            AppOpenDirIncoming::dbus_method_run(builder);
            AppOpenFileIncoming::dbus_method_run(builder);

            /////////////////
            // Demo App
            DemoAppInfoIncoming::dbus_method_run(builder);
            DemoAppInfoIncoming::dbus_method_run_by_id(builder);

            /////////////////
            // Device
            DeviceInfoIncoming::dbus_method_run(builder);
            DeviceInfoIncoming::dbus_method_run_by_id(builder);

            DevicePackageInstallIncoming::dbus_method_run_path(builder);
            DevicePackageInstallIncoming::dbus_method_run_urls(builder);
            DevicePackageInstallIncoming::dbus_method_run_path_by_id(builder);
            DevicePackageInstallIncoming::dbus_method_run_urls_by_id(builder);

            DevicePackageRunIncoming::dbus_method_run(builder);
            DevicePackageRunIncoming::dbus_method_run_by_id(builder);
            DevicePackageRunIncoming::dbus_method_run_package(builder);
            DevicePackageRunIncoming::dbus_method_run_package_by_id(builder);

            DevicePackageUninstallIncoming::dbus_method_run(builder);
            DevicePackageUninstallIncoming::dbus_method_run_by_id(builder);
            DevicePackageUninstallIncoming::dbus_method_run_package(builder);
            DevicePackageUninstallIncoming::dbus_method_run_package_by_id(builder);

            DeviceScreenshotIncoming::dbus_method_run(builder);
            DeviceScreenshotIncoming::dbus_method_run_by_id(builder);

            DeviceSyncIncoming::dbus_method_run(builder);

            DeviceTerminalIncoming::dbus_method_run(builder);
            DeviceTerminalIncoming::dbus_method_run_by_id(builder);

            DeviceUploadIncoming::dbus_method_run_path(builder);
            DeviceUploadIncoming::dbus_method_run_path_by_id(builder);
            DeviceUploadIncoming::dbus_method_run_url(builder);
            DeviceUploadIncoming::dbus_method_run_url_by_id(builder);

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
            EmulatorPackageInstallIncoming::dbus_method_run_url(builder);
            EmulatorPackageInstallIncoming::dbus_method_run_url_by_id(builder);

            EmulatorPackageRunIncoming::dbus_method_run(builder);
            EmulatorPackageRunIncoming::dbus_method_run_by_id(builder);
            EmulatorPackageRunIncoming::dbus_method_run_package(builder);
            EmulatorPackageRunIncoming::dbus_method_run_package_by_id(builder);

            EmulatorPackageUninstallIncoming::dbus_method_run(builder);
            EmulatorPackageUninstallIncoming::dbus_method_run_by_id(builder);
            EmulatorPackageUninstallIncoming::dbus_method_run_package(builder);
            EmulatorPackageUninstallIncoming::dbus_method_run_package_by_id(builder);

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

            FlutterInstallIncoming::dbus_method_run(builder);
            FlutterInstallIncoming::dbus_method_run_by_id(builder);

            FlutterProjectFormatIncoming::dbus_method_run(builder);
            FlutterProjectFormatIncoming::dbus_method_run_by_id(builder);

            FlutterProjectReportIncoming::dbus_method_run_path(builder);
            FlutterProjectReportIncoming::dbus_method_run_path_by_id(builder);
            FlutterProjectReportIncoming::dbus_method_run_url(builder);
            FlutterProjectReportIncoming::dbus_method_run_url_by_id(builder);

            FlutterSyncIncoming::dbus_method_run(builder);

            FlutterTerminalIncoming::dbus_method_run(builder);
            FlutterTerminalIncoming::dbus_method_run_by_id(builder);

            FlutterUninstallIncoming::dbus_method_run(builder);
            FlutterUninstallIncoming::dbus_method_run_by_id(builder);

            /////////////////
            // Psdk
            PsdkAvailableIncoming::dbus_method_run(builder);
            PsdkAvailableIncoming::dbus_method_run_by_id(builder);

            PsdkPackageSignIncoming::dbus_method_run_path(builder);
            PsdkPackageSignIncoming::dbus_method_run_path_by_id(builder);

            PsdkTargetPackageFindIncoming::dbus_method_run_package(builder);
            PsdkTargetPackageFindIncoming::dbus_method_run_package_by_id(builder);
            PsdkTargetPackageFindIncoming::dbus_method_run_package_target(builder);
            PsdkTargetPackageFindIncoming::dbus_method_run_package_target_by_id(builder);

            PsdkTargetPackageInstallIncoming::dbus_method_run_path(builder);
            PsdkTargetPackageInstallIncoming::dbus_method_run_path_by_id(builder);

            PsdkTargetPackageUninstallIncoming::dbus_method_run_package(builder);
            PsdkTargetPackageUninstallIncoming::dbus_method_run_package_by_id(builder);
            PsdkTargetPackageUninstallIncoming::dbus_method_run_package_target(builder);
            PsdkTargetPackageUninstallIncoming::dbus_method_run_package_target_by_id(builder);

            PsdkDownloadIncoming::dbus_method_run(builder);
            PsdkDownloadIncoming::dbus_method_run_by_id(builder);

            PsdkInfoIncoming::dbus_method_run(builder);
            PsdkInfoIncoming::dbus_method_run_by_id(builder);

            PsdkInstallIncoming::dbus_method_run(builder);
            PsdkInstallIncoming::dbus_method_run_by_id(builder);

            PsdkSyncIncoming::dbus_method_run(builder);

            PsdkTerminalIncoming::dbus_method_run(builder);
            PsdkTerminalIncoming::dbus_method_run_by_id(builder);

            PsdkUninstallIncoming::dbus_method_run(builder);
            PsdkUninstallIncoming::dbus_method_run_by_id(builder);

            /////////////////
            // Sdk
            SdkAvailableIncoming::dbus_method_run(builder);
            SdkAvailableIncoming::dbus_method_run_by_id(builder);

            SdkDownloadIncoming::dbus_method_run(builder);
            SdkDownloadIncoming::dbus_method_run_by_id(builder);

            SdkIdeCloseIncoming::dbus_method_run(builder);
            SdkIdeCloseIncoming::dbus_method_run_by_id(builder);

            SdkIdeOpenIncoming::dbus_method_run(builder);
            SdkIdeOpenIncoming::dbus_method_run_by_id(builder);

            SdkInfoIncoming::dbus_method_run(builder);
            SdkInfoIncoming::dbus_method_run_by_id(builder);

            SdkInstallIncoming::dbus_method_run(builder);
            SdkInstallIncoming::dbus_method_run_by_id(builder);

            SdkProjectFormatIncoming::dbus_method_run_path(builder);
            SdkProjectFormatIncoming::dbus_method_run_path_by_id(builder);

            SdkSyncIncoming::dbus_method_run(builder);

            SdkToolsIncoming::dbus_method_run(builder);
            SdkToolsIncoming::dbus_method_run_by_id(builder);

            SdkUninstallIncoming::dbus_method_run(builder);
            SdkUninstallIncoming::dbus_method_run_by_id(builder);

            /////////////////
            // Methods only for D-Bus
            methods::OnlyDbusMethods::search(builder);
            methods::OnlyDbusMethods::fun_can_you_c_plus_plus_do_that(builder);
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
