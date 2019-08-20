extern crate clap;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use rn::config::Config;
use rn::error::RnError;
use std::path::Path;
use std::process::Command;

pub type Result<T, E = RnError> = std::result::Result<T, E>;

fn main() {
    let matches = App::new("rn")
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
        .subcommand(SubCommand::with_name("print").about("Print the default config"))
        .get_matches();
    match run_subcommand(&matches) {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
}

fn run_subcommand(matches: &ArgMatches) -> Result<()> {
    match matches.subcommand() {
        ("init", Some(clone_matches)) => init(&clone_matches)?,
        ("update", Some(clone_matches)) => update(&clone_matches)?,
        ("build", Some(build_matches)) => build(&build_matches)?,
        ("run", Some(run_matches)) => run(&run_matches)?,
        ("print", Some(run_matches)) => print(&run_matches)?,
        _ => unreachable!(),
    }
    Ok(())
}

fn init(matches: &ArgMatches) -> Result<()> {
    if Path::new(".rn").exists() {
        return Err(RnError::RnAlreadyInitiated);
    }

    let dir = matches.value_of("default-dir").unwrap();
    let bin = matches.value_of("default-bin").unwrap();
    let args = match matches.value_of("default-args") {
        Some(args) => args,
        None => "",
    };

    let cfg = Config::new(dir, bin, args);
    cfg.save()?;
    Ok(())
}

fn update(matches: &ArgMatches) -> Result<()> {
    let mut cfg = Config::from_file()?;
    if let Some(dir) = matches.value_of("default-dir") {
        cfg.update_directory(dir);
    }
    if let Some(bin) = matches.value_of("default-bin") {
        cfg.update_binary(bin);
    }
    if let Some(args) = matches.value_of("default-args") {
        cfg.update_args(args);
    }

    cfg.save()?;
    Ok(())
}

fn build(matches: &ArgMatches) -> Result<()> {
    let cfg = Config::from_file()?;
    let directory = matches
        .value_of("directory")
        .or_else(|| Some(cfg.get_directory()))
        .ok_or(RnError::BuildDirectory)?;

    Command::new("ninja")
        .arg("-C")
        .arg(directory)
        .spawn()
        .expect("Failed to run ninja")
        .wait()
        .expect("Failed to wait o build command");

    Ok(())
}

fn run(matches: &ArgMatches) -> Result<()> {
    let cfg = Config::from_file()?;
    let directory = matches
        .value_of("directory")
        .or_else(|| Some(cfg.get_directory()))
        .ok_or(RnError::BuildDirectory)?;

    let bin = matches
        .value_of("binary")
        .or_else(|| Some(cfg.get_binary()))
        .ok_or(RnError::BinaryMissing)?;

    let args = matches
        .value_of("directory")
        .or_else(|| Some(cfg.get_directory()))
        .ok_or(RnError::BinaryArgsMissing)?;

    Command::new(bin)
        .current_dir(directory)
        .arg(args)
        .spawn()
        .expect(format!("Failed to run the binary ({} {})", bin, args).as_str())
        .wait()
        .expect("Failed to wait o build command");

    Ok(())
}

fn print(_matches: &ArgMatches) -> Result<()> {
    let cfg = Config::from_file()?;
    cfg.print();
    Ok(())
}
