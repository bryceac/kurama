mod site;

use crate::site::Site;


fn main() {
    let site = match Site::from_file("config.json") {
        Ok(configuration) => configuration,
        _ => Site::new()
    };
}


