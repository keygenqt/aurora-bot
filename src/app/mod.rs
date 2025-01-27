/// Module application
pub mod api {
    pub mod convert;
    pub mod create;
    pub mod enums;
    pub mod handler;
    pub mod incoming {
        pub mod models;
    }
    pub mod outgoing {
        pub mod models;
    }
}
pub mod device {
    pub mod methods;
}
pub mod emulator {
    pub mod methods;
}
