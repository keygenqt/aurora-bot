use dbus_crossroads::IfaceBuilder;

use crate::feature::outgoing::DataOutgoing;
use crate::feature::outgoing::TraitOutgoing;
use crate::feature::state_message::outgoing::StateMessageOutgoing;
use crate::service::dbus::server::DbusOnly;
use crate::service::dbus::server::IfaceData;
use crate::tools::macros::tr;
use crate::tools::single;
pub struct OnlyDbusMethods {}

impl OnlyDbusMethods {
    pub fn name(key: DbusOnly) -> String {
        serde_variant::to_variant_name(&key).unwrap().to_string()
    }

    pub fn search(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            OnlyDbusMethods::name(DbusOnly::FaqSearch),
            ("search",),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (search,): (String,)| async move {
                let result = match single::get_request().get_search(search) {
                    Ok(value) => DataOutgoing::serialize(OnlyDbusMethods::name(DbusOnly::FaqSearch), value.clone()),
                    Err(_) => StateMessageOutgoing::new_error(tr!("что-то пошло не так, попробуйте выполнить позже"))
                        .to_json(),
                };
                ctx.reply(Ok((result,)))
            },
        );
    }

    pub fn fun_can_you_c_plus_plus_do_that(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            OnlyDbusMethods::name(DbusOnly::CanYouCPlusPlusDoThat),
            (),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (): ()| async move { ctx.reply(Ok(("I don't think so",))) },
        );
    }
}
