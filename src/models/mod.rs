use crate::utils::macros::print_info;

/// Application data models
pub mod configuration;
pub mod incoming;
pub mod outgoing;

pub mod emulator {
    pub mod model;
    pub mod session;
}
pub mod device {
    pub mod model;
}
pub mod flutter {
    pub mod model;
}
pub mod psdk {
    pub mod model;
}
pub mod sdk {
    pub mod model;
}

pub trait TraitModel {
    fn print(&self);
}

impl dyn TraitModel {
    pub fn print_list<T: TraitModel>(models: Vec<T>) {
        if models.is_empty() {
            print_info!("эмуляторы не найдены")
        }
        for (index, e) in models.iter().enumerate() {
            if index != 0 {
                println!()
            }
            e.print()
        }
    }
}
