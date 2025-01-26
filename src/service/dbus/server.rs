use dbus_tokio::connection;
use futures::future;
use tokio::time::sleep;
use dbus::channel::MatchingReceiver;
use dbus::message::MatchRule;
use dbus_crossroads::Crossroads;
use std::time::Duration;

struct Fn { called_count: u32 }

/// gdbus call --session --dest com.keygenqt.aurora_bot --object-path /bot --method com.keygenqt.aurora_bot.method "Vitaliy"
/// gdbus monitor --session --dest com.keygenqt.aurora_bot --object-path /bot com.keygenqt.aurora_bot.fn_async
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut cr = Crossroads::new();
    let (resource, c) = connection::new_session_sync()?;

    cr.set_async_support(Some((c.clone(), Box::new(|x| { tokio::spawn(x); }))));

    let iface_token = cr.register("com.keygenqt.aurora_bot", |b| {
        b.signal::<(String,), _>("fn_async", ("sender",));
        b.method_with_cr_async("method", ("name",), ("reply",), |mut ctx, cr, (name,): (String,)| {
            let hello: &mut Fn = cr.data_mut(ctx.path()).unwrap(); // ok_or_else(|| MethodErr::no_path(ctx.path()))?;
            println!("Incoming hello call from {}!", name);
            hello.called_count += 1;
            let s = format!("Hello {}! This API has been used {} times.", name, hello.called_count);
            async move {
                sleep(Duration::from_millis(500)).await;
                let signal_msg = ctx.make_signal("fn_async", (name,));
                ctx.push_msg(signal_msg);
                ctx.reply(Ok((s,)))
            }
        });
    });
    cr.insert("/bot", &[iface_token], Fn { called_count: 0});
    c.start_receive(MatchRule::new_method_call(), Box::new(move |msg, conn| {
        cr.handle_message(msg, conn).unwrap();
        true
    }));
    let _handle = tokio::spawn(async {
        let err = resource.await;
        panic!("Lost connection to D-Bus: {}", err);
    });
    c.request_name("com.keygenqt.aurora_bot", false, true, false).await?;
    future::pending::<()>().await;
    unreachable!()
}
