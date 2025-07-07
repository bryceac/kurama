use clap::{ Parser };
use crate::commands::Commands;

#[derive(Parser)]
#[clap(about = "A Static site generator", long_about = None,
author = "Bryce Campbell <tonyhawk2100@gmail.com>",
version = "0.2.0")]
pub struct Kurama {
    #[clap(subcommand)]
    pub command: Commands
}