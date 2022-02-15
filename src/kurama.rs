use clap::{ Parser };
use crate::commands::Commands;

#[derive(Parser)]
#[clap(name = "kurama")]
#[clap(about = "A Static site generator", long_about = None)]
#[clap(version = "0.1")]
pub struct Kurama {
    #[clap(subcommand)]
    pub command: Commands
}