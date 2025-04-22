/// Application data models
pub mod configuration;

pub mod emulator {
    pub mod model;
    pub mod session;
}
pub mod demo_app {
    pub mod model;
}
pub mod device {
    pub mod model;
}
pub mod flutter_available {
    pub mod model;
}
pub mod flutter_installed {
    pub mod model;
}
pub mod psdk_available {
    pub mod model;
}
pub mod psdk_installed {
    pub mod model;
}
pub mod psdk_target {
    pub mod model;
}
pub mod psdk_target_package {
    pub mod model;
}
pub mod pubspec {
    pub mod model;
}
pub mod sdk_available {
    pub mod model;
}
pub mod sdk_installed {
    pub mod model;
}

pub trait TraitModel {
    fn get_id(&self) -> String;
    fn get_key(&self) -> String;
    fn print(&self);
}
