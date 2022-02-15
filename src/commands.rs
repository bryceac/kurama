use clap::{ AppSettings, Subcommand };

#[derive(Subcommand)]
pub enum Commands {
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    New {
        path: String
    },
    Gen {},
    Serve {}
}