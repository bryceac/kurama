mod configuration;

use crate::configuration::Configuration;


fn main() {
    let site_configuration = Configuration::from_file("config.json").expect("Could not load configuration");
}


