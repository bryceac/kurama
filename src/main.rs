mod configuration;

use crate::configuration::Configuration;


fn main() {
    let site = Configuration::from_file("config.json").expect("Could not load configuration");
}


