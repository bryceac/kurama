mod clean;
mod commands;
mod create;
mod configuration;
mod generate;
mod init;
mod kurama;
mod link;
mod navigation_item;
mod metadata;
mod page;
mod section;
mod serve;
mod save_string;


use crate::{
    clean::Clean as Clean,
    create::Create as Create,
    generate::Generate as Generate,
    init::Init as Init, 
    serve::Serve as Serve,
    commands::Commands,
    configuration::Configuration as Configuration,
    kurama::Kurama,
    link::Link as Link, 
    section::Section as Section, 
    navigation_item::NavigationItem as NavigationItem,
    page::Page as Page,
    save_string::Save as Save
};

use clap::Parser;


#[tokio::main]
async fn main() {
    let site = Kurama::parse();

    match site.command {
        Commands::Clean(clean) => clean.run().await,
        Commands::Create(create) => create.run().await,
        Commands::Generate(generate) => generate.run().await,
        Commands::Init(initializer) => initializer.run().await,
        Commands::Serve(serve) => serve.run().await
    }
}