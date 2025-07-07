use clap::Parser;
use crate::{ Clean, Create, Init, Generate, Serve };

#[derive(Parser)]
pub enum Commands {
    Clean(Clean),
    Create(Create),
    Generate(Generate),
    Init(Init),
    Serve(Serve)
}