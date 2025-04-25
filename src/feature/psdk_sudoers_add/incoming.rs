use dbus_crossroads::IfaceBuilder;
use serde::Deserialize;
use serde::Serialize;

use crate::feature::ClientMethodsKey;
use crate::feature::incoming::TraitIncoming;
use crate::feature::outgoing::OutgoingType;
use crate::feature::outgoing::TraitOutgoing;
use crate::feature::selector::selects::select_psdk_installed::PsdkInstalledModelSelect;
use crate::feature::state_message::outgoing::StateMessageOutgoing;
use crate::models::psdk_installed::model::PsdkInstalledModel;
use crate::service::dbus::server::IfaceData;
use crate::tools::macros::print_debug;
use crate::tools::macros::tr;

#[derive(Serialize, Deserialize, Clone)]
pub struct PsdkSudoersAddIncoming {}

impl PsdkSudoersAddIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::PsdkSudoersAdd)
            .unwrap()
            .to_string()
    }

    pub fn new() -> Box<PsdkSudoersAddIncoming> {
        print_debug!("> {}: new()", Self::name());
        Box::new(Self {})
    }

    pub fn dbus_method_run(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            Self::name(),
            (),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (): ()| async move {
                let outgoing = Self::new().run(OutgoingType::Dbus);
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }

    fn run(models: Vec<PsdkInstalledModel>) -> Result<Box<dyn TraitOutgoing>, Box<dyn std::error::Error>> {
        for model in models {
            println!("{:?}", model.dir);
        }
        Ok(StateMessageOutgoing::new_info(tr!("@todo")))
    }
}

impl TraitIncoming for PsdkSudoersAddIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        // Search
        let models = PsdkInstalledModelSelect::search(&None, tr!("получаем информацию о Platform SDK"), &send_type);
        // Select
        match models.iter().count() {
            0 => StateMessageOutgoing::new_info(tr!("Platform SDK не найдены")),
            _ => match Self::run(models) {
                Ok(result) => result,
                Err(_) => StateMessageOutgoing::new_error(tr!("не удалось обновить sudoers запись")),
            },
        }
    }
}
