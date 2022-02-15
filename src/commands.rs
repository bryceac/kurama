use clap::{ AppSettings, Subcommand };

#[derive(Subcommand)]
pub enum Commands {
    #[clap(about = "deletes the output directory", long_about = None)]
    Clean {},
    #[clap(about = "create a project directory at the given path", long_about = None)]
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Create {
        path: String
    },
    #[clap(about = "build the website", long_about = None)]
    Generate {},
    #[clap(about = "create project directory structure in current directory", long_about = None)]
    Init {},
    #[clap(about = "start dev server, to preview a website", long_about = None)]
    Serve {}
}