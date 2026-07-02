use clap::ValueEnum;

#[derive(Default, ValueEnum, Clone, Debug)]
pub enum BuildMode {
    #[default]
    Dev,
    Release
}