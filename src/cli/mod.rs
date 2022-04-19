use ansi_term::Colour::{Cyan, Green, Purple, Yellow};
use clap::{crate_authors, crate_description, crate_name, crate_version, AppSettings, Parser};
use std::{env, process::exit};

use crate::commands::init::init;

use self::{
    app::App,
    help::{build_build_help_message, build_run_help_message},
};

pub mod app;
mod help;

#[derive(Debug, Parser)]
pub enum Command {
    /// Run a Fleet project
    Run,
    /// Build a Fleet project
    Build,
}

#[derive(Debug, Parser)]
#[clap(
    name = crate_name!(),
    version = crate_version!(),
    about = crate_description!(),
    author = crate_authors!(),
    global_setting = AppSettings::ArgRequiredElseHelp
)]

pub struct CLI {
    #[clap(subcommand)]
    pub subcommand: Command,
}

impl CLI {
    // pub fn parse()
    pub fn handle_failure() {
        // check if it's a configuration issue
        match rustc_version::version_meta().unwrap().channel {
            rustc_version::Channel::Nightly => {
                // no issues here
            }
            _ => {
                println!(
                    "{} You are not using a {} compiler. Run {}.",
                    Yellow.paint("=>"),
                    Purple.paint("`nightly`"),
                    Cyan.paint("`rustup default nightly`"),
                );
            }
        }

        // check if sccache is installed
        let sccache_path = std::path::Path::new(&dirs::home_dir().unwrap())
            .join(".cargo")
            .join("bin")
            .join("sccache");

        if !sccache_path.exists() {
            println!(
                "{} You have not installed {}. Run {}.",
                Yellow.paint("=>"),
                Purple.paint("`sccache`"),
                Cyan.paint("`cargo install sccache`"),
            );
        }

        // check if lld is available (on linux) and zld on macos
        if cfg!(unix) {
            let lld_path = std::path::Path::new("/usr/bin/lld");

            if !lld_path.exists() {
                println!(
                    "{} You have not installed {}. Run {}.",
                    Yellow.paint("=>"),
                    Purple.paint("`lld`"),
                    Cyan.paint("`sudo apt install lld`"),
                );
            }

            // check if clang is available
            let clang_path = std::path::Path::new("/usr/bin/clang");

            if !clang_path.exists() {
                println!(
                    "{} You have not installed {}. Run {}.",
                    Yellow.paint("=>"),
                    Purple.paint("`clang`"),
                    Cyan.paint("`sudo apt install clang`"),
                );
            }
        } else if cfg!(macos) {
            let zld_path = std::path::Path::new("/usr/bin/zld");

            if !zld_path.exists() {
                println!(
                    "{} You have not installed {}. Run {}.",
                    Yellow.paint("=>"),
                    Purple.paint("`zld`"),
                    Cyan.paint("`brew install zld`"),
                );
            }
        }

        exit(1);
    }

    pub fn display_help(cmd: &str) {
        let mut help_menu = format!(
            r#"{} {}
Dimension <team@dimension.dev>
The blazing fast build tool for Rust.

{}:
    fleet <SUBCOMMAND>

{}:
    -h, --help       Print help information
    -V, --version    Print version information

{}:
    build    Build a Fleet project
    run      Run a Fleet project"#,
            Green.paint("fleet"),
            env!("CARGO_PKG_VERSION"),
            Yellow.paint("USAGE"),
            Yellow.paint("OPTIONS"),
            Yellow.paint("SUBCOMMANDS"),
        );

        if cmd == "run" {
            help_menu = build_run_help_message()
        } else if cmd == "build" {
            help_menu = build_build_help_message()
        }
        println!("{}", help_menu)
    }

    pub fn run() {
        let args = std::env::args().collect::<Vec<String>>();
        let app = App::new();

        if args.len() <= 1 {
            CLI::display_help("help");
        } else {
            let cmd = &args[1];

            if args.contains(&String::from("--help")) || args.contains(&String::from("-h")) {
                CLI::display_help(cmd);
                std::process::exit(1)
            }

            match cmd.as_str() {
                "run" => {
                    init(app);
                    // get all args after the subcommand
                    let args: Vec<String> = args.iter().skip(2).map(|s| s.to_string()).collect();
                    // Run the crate
                    let status = std::process::Command::new("cargo")
                        .arg("run")
                        .args(args)
                        .status()
                        .unwrap();

                    if !status.success() {
                        CLI::handle_failure();
                    }
                }
                "build" => {
                    init(app);

                    let args: Vec<String> = args.iter().skip(2).map(|s| s.to_string()).collect();

                    let status = std::process::Command::new("cargo")
                        .arg("build")
                        .args(args)
                        .status()
                        .unwrap();

                    if !status.success() {
                        CLI::handle_failure();
                    }
                }
                _ => {}
            }
        }
    }
}
