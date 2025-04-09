use dbus_crossroads::IfaceBuilder;

use crate::models::client::outgoing::DataOutgoing;
use crate::models::client::outgoing::TraitOutgoing;
use crate::models::client::state_message::outgoing::StateMessageOutgoing;
use crate::service::dbus::server::DbusOnly;
use crate::service::dbus::server::IfaceData;
use crate::tools::macros::tr;
use crate::tools::single;
pub struct FaqDbusMethods {}

impl FaqDbusMethods {
    pub fn name(key: DbusOnly) -> String {
        serde_variant::to_variant_name(&key).unwrap().to_string()
    }

    pub fn search(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            FaqDbusMethods::name(DbusOnly::FaqSearch),
            ("search",),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (search,): (String,)| async move {
                let result = match single::get_request().get_search(search) {
                    Ok(value) => DataOutgoing::serialize(FaqDbusMethods::name(DbusOnly::FaqSearch), value.clone()),
                    Err(_) => StateMessageOutgoing::new_error(tr!("что-то пошло не так, попробуйте выполнить позже"))
                        .to_json(),
                };
                ctx.reply(Ok((result,)))
            },
        );
    }
}
