pub use self::input::*;
pub use self::output::*;
use mizer_devices::{DeviceManager, DeviceRef};
use mizer_node::{Injector, SelectVariant};

mod input;
mod output;

trait G13InjectorExt {
    fn get_devices(&self) -> Vec<SelectVariant>;
}

impl G13InjectorExt for Injector {
    fn get_devices(&self) -> Vec<SelectVariant> {
        let device_manager = self.get::<DeviceManager>().unwrap();

        device_manager
            .current_devices()
            .into_iter()
            .flat_map(|device| {
                if let DeviceRef::G13(g13) = device {
                    Some(SelectVariant::from(g13.id))
                } else {
                    None
                }
            })
            .collect()
    }
}
