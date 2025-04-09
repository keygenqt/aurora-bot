use dbus_crossroads::IfaceBuilder;

use crate::{models::client::{outgoing::{DataOutgoing, TraitOutgoing}, state_message::outgoing::StateMessageOutgoing}, service::dbus::server::{DbusOnly, IfaceData}, tools::{macros::tr, single}};
pub struct FaqDbusMethods {}

impl FaqDbusMethods {
    pub fn name(key: DbusOnly) -> String {
        serde_variant::to_variant_name(&key)
            .unwrap()
            .to_string()
    }

    pub fn search(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            FaqDbusMethods::name(DbusOnly::FaqSearch),
            ("search",),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (search,): (String,)| async move {
                let result = match single::get_request().get_search(search) {
                    Ok(value) => DataOutgoing::serialize(FaqDbusMethods::name(DbusOnly::FaqSearch), value.clone()),
                    Err(_) => StateMessageOutgoing::new_error(tr!("что-то пошло не так, попробуйте выполнить позже")).to_json(),
                };
                ctx.reply(Ok((result,)))
            },
        );
    }
}
