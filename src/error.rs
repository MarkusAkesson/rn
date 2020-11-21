use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum RnError {
    #[snafu(display("rn already initiated in the current directory"))]
    RnAlreadyInitiated,
    #[snafu(display("rn not initiated in the current directory"))]
    RnNotInitiated,
    #[snafu(display("Failed to open config file"))]
    OpeningConfigFile,
    #[snafu(display("Failed to read the yaml file"))]
    ReadYaml,
    #[snafu(display("Failed to save config"))]
    FailedToSaveFile,
    #[snafu(display("Failed to get get build directory"))]
    BuildDirectory,
    #[snafu(display("Binary missing"))]
    BinaryMissing,
    #[snafu(display("Argmunents missing"))]
    BinaryArgsMissing,
    #[snafu(display("Unknown subcommand"))]
    UnknownSubcommand,
}
