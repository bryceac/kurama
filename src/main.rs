mod site;

use crate::site::Site;


fn main() {
    match Site::from_file("config.json") {
        Ok(configuration) => println!("{}", configuration.name),
        Err(error) => println!("{}", error)
    }
}


