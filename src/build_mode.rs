use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug)]
pub enum BuildMode {
    Dev,
    Release
}