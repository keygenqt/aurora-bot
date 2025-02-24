/// Module services
pub mod command {
    pub mod exec;
}
pub mod dbus {
    pub mod server;
}
pub mod requests {
    pub mod client;
    pub mod methods;
}
pub mod responses {
    pub mod common;
    pub mod faq;
    pub mod gitlab_tags;
    pub mod user;
}
pub mod ssh {
    pub mod client;
}
pub mod websocket {
    pub mod client;
}
