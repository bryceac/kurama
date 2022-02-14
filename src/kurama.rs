use clap::{ Parser };
use crate::commands::Commands;

#[derive(Parser)]
#[clap(name = "kurama")]
#[clap(about = "A Static site generator", long_about = None)]
pub struct Kurama {
    #[clap(subcommand)]
    command: Commands
}