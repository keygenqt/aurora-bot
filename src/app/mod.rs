/// Module application
pub mod api {
    pub mod convert;
    pub mod enums;
    pub mod handler;
    pub mod incoming {
        pub mod create;
        pub mod models;
    }
    pub mod outgoing {
        pub mod create;
        pub mod models;
    }
}
pub mod device {
    pub mod methods;
}
pub mod emulator {
    pub mod methods;
}
pub mod settings {
    pub mod methods;
}
