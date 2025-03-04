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
pub mod flutter_available {
    pub mod model;
    pub mod select;
}
pub mod flutter_installed {
    pub mod model;
    pub mod select;
}
pub mod psdk_available {
    pub mod model;
    pub mod select;
}
pub mod psdk_installed {
    pub mod model;
    pub mod select;
}
pub mod sdk_available {
    pub mod model;
    pub mod select;
}
pub mod sdk_installed {
    pub mod model;
    pub mod select;
}

pub trait TraitModel {
    fn get_id(&self) -> String;
    fn get_key(&self) -> String;
    fn print(&self);
}
