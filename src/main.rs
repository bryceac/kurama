mod archive;
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
mod paginator;
mod post;
mod section;
mod serve;
mod save_string;
mod store;


use crate::{
    clean::Clean as Clean,
    create::Create as Create,
    generate::Generate as Generate,
    init::Init as Init, 
    serve::Serve as Serve,
    commands::Commands,
    configuration::Configuration as Configuration,
    kurama::Kurama,
    archive::Archive as Archive,
    link::Link as Link,
    metadata::Metadata as Metadata, 
    section::Section as Section, 
    navigation_item::NavigationItem as NavigationItem,
    page::Page as Page,
    paginator::Paginator as Paginator,
    paginator::PaginationMethod as PaginationMethod,
    post::Post as Post,
    save_string::Save as Save,
    store::Store as Store,
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