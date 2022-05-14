use anyhow::Result;
use std::path::PathBuf;
use clap::{Args, Parser};
use move_package::BuildConfig;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Move {
    /// Path to a package which the command should be run with respect to.
    #[clap(
        long = "path",
        short = 'p',
        global = true,
        parse(from_os_str),
        default_value = "."
    )]
    package_path: PathBuf,

    /// Print additional diagnostics if available.
    #[clap(short = 'v', global = true)]
    verbose: bool,

    /// Package build options
    #[clap(flatten)]
    build_config: BuildConfig,
}
