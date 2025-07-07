use clap::Parser;
use fs_extra::dir;

#[derive(Default, Parser)]
#[clap(about = "deletes the output directory", long_about = None)]
pub struct Clean {}

impl Clean {
    pub async fn run(&self) {
        if let Err(error) = dir::remove("output") {
            println!("{}", error)
        }
    }
}