use clap::{ AppSettings, Subcommand };

#[derive(Subcommand)]
pub enum Commands {
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Create {
        path: String
    },
    Gen {},
    Serve {}
}