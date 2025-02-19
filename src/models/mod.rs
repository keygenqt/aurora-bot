/// Application data models
pub mod client;
pub mod configuration;

pub mod emulator {
    pub mod model;
    pub mod select;
    pub mod session;
}
pub mod device {
    pub mod model;
    pub mod select;
}
pub mod flutter {
    pub mod model;
    pub mod select;
}
pub mod psdk {
    pub mod model;
    pub mod select;
}
pub mod sdk {
    pub mod model;
    pub mod select;
}

pub trait TraitModel {
    fn get_id(&self) -> String;
    fn get_key(&self) -> String;
    fn print(&self);
}
