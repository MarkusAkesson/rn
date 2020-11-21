use clap::ArgMatches;
use rn::cli;
use rn::config::Config;
use rn::error::RnError;
use std::path::Path;
use std::process::{Command, Stdio};

pub type Result<T, E = RnError> = std::result::Result<T, E>;

fn main() {
    let args = cli::app().get_matches();
    match run_subcommand(&args) {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    }
}

fn run_subcommand(matches: &ArgMatches) -> Result<()> {
    match matches.subcommand() {
        ("init", Some(clone_matches)) => init(&clone_matches)?,
        ("update", Some(clone_matches)) => update(&clone_matches)?,
        ("build", Some(build_matches)) => build(&build_matches)?,
        ("run", Some(run_matches)) => run(&run_matches)?,
        ("print", Some(run_matches)) => print(&run_matches)?,
        ("clean", Some(run_matches)) => clean(&run_matches)?,
        _ => return Err(RnError::UnknownSubcommand),
    }
    Ok(())
}

fn init(matches: &ArgMatches) -> Result<()> {
    if Path::new(".rn").exists() {
        return Err(RnError::RnAlreadyInitiated);
    }

    let dir = matches.value_of("default-dir").unwrap();
    let bin = matches.value_of("default-bin").unwrap();
    let args = matches.value_of("default-args");

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
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .spawn()
        .expect("Failed to run ninja")
        .wait()
        .expect("Failed while waiting for command to finish");

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

    let bin_args = matches.value_of("args").or_else(|| cfg.get_args());

    let mut cmd = Command::new(format!("./{}", bin));

    if let Some(args) = bin_args {
        cmd.arg(args);
    }

    cmd.current_dir(directory)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .spawn()
        .expect("Failed to run the binary")
        .wait()
        .expect("Failed while waiting for command to finish");

    Ok(())
}

fn print(_matches: &ArgMatches) -> Result<()> {
    let cfg = Config::from_file()?;
    cfg.print();
    Ok(())
}

fn clean(matches: &ArgMatches) -> Result<()> {
    let cfg = Config::from_file()?;
    let directory = matches
        .value_of("directory")
        .or_else(|| Some(cfg.get_directory()))
        .ok_or(RnError::BuildDirectory)?;

    Command::new("gn")
        .arg("clean")
        .arg(directory)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .spawn()
        .expect("Failed to run gn clean")
        .wait()
        .expect("Failed while waiting for command to finish");

    Ok(())
}
