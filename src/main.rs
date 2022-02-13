mod configuration;

use crate::configuration::Configuration;


fn main() {
    let site = match Configuration::from_file("config.json") {
        Ok(configuration) => configuration,
        _ => Configuration::new()
    };
}


