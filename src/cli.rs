use clap::{App, AppSettings, Arg, SubCommand};

pub fn app<'a, 'b>() -> App<'a, 'b> {
    App::new("rn")
        .version("1.0")
        .about("Helper program for gn and ninja projects")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("init")
                .about("Init rn in this directory")
                .arg(
                    Arg::with_name("default-dir")
                        .help("Set the default output directory")
                        .long("default-dir")
                        .required(true)
                        .takes_value(true)
                        .value_name("DIRECTORY"),
                )
                .arg(
                    Arg::with_name("default-bin")
                        .help("Set the default binary to run")
                        .long("default-bin")
                        .required(true)
                        .takes_value(true)
                        .value_name("BINARY"),
                )
                .arg(
                    Arg::with_name("default-args")
                        .help("Set the default arguments for the binary")
                        .long("default-args")
                        .required(false)
                        .takes_value(true)
                        .value_name("ARGUMENTS"),
                ),
        )
        .subcommand(
            SubCommand::with_name("update")
                .about("update the default arguments")
                .arg(
                    Arg::with_name("default-dir")
                        .help("Set the default output directory")
                        .long("default-dir")
                        .takes_value(true)
                        .value_name("DIRECTORY"),
                )
                .arg(
                    Arg::with_name("default-bin")
                        .help("Set the default binary to run")
                        .long("default-bin")
                        .takes_value(true)
                        .value_name("BINARY"),
                )
                .arg(
                    Arg::with_name("default-args")
                        .help("Set the default arguments for the binary")
                        .long("default-args")
                        .takes_value(true)
                        .value_name("ARGUMENTS"),
                ),
        )
        .subcommand(
            SubCommand::with_name("build")
                .about("Run the build script")
                .arg(
                    Arg::with_name("directory")
                        .long("directory")
                        .takes_value(true)
                        .value_name("DIRECTORY")
                        .help("Path to directory with build script"),
                ),
        )
        .subcommand(
            SubCommand::with_name("run")
                .about("Run binary")
                .arg(
                    Arg::with_name("directory")
                        .help("Run binary from this directory")
                        .long("directory")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("binary")
                        .help("Binary to run")
                        .long("binary")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("args")
                        .help("Args to use when running the binary")
                        .long("arguments")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("clean")
                .about("Clean up output directory")
                .arg(
                    Arg::with_name("directory")
                        .long("directory")
                        .takes_value(true)
                        .value_name("DIRECTORY")
                        .help("Path to directory with build script"),
                ),
        )
        .subcommand(SubCommand::with_name("print").about("Print the default config"))
}
