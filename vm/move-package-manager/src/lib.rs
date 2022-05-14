pub mod cmds;

use anyhow::Result;
use std::path::PathBuf;
use clap::{Args, Parser};
use move_package::BuildConfig;

use move_core_types::errmap::ErrorMapping;
use starcoin_config::genesis_config;
use starcoin_vm_runtime::natives::starcoin_natives;

use move_cli::package::cli::handle_package_commands;
use move_cli::{experimental, package, sandbox, DEFAULT_STORAGE_DIR};
use cmds::compatibility_check::{
    handle_compatibility_check, CompatibilityCheckCommand,
};
use cmds::releasement::{handle_release, Releasement};
use cmds::integration_test::{run_integration_test, IntegrationTestCommand};


#[derive(Parser)]
pub enum Commands {
    /// Execute a package command. Executed in the current directory or the closest containing Move
    /// package.
    #[clap(name = "package")]
    Package {
        #[clap(subcommand)]
        cmd: package::cli::PackageCommand,
    },
    /// Release the package.
    #[clap(name = "release")]
    Release(Releasement),
    /// Execute a sandbox command.
    #[clap(name = "sandbox")]
    Sandbox {
        /// Directory storing Move resources, events, and module bytecodes produced by module publishing
        /// and script execution.
        #[clap(long, default_value = DEFAULT_STORAGE_DIR, parse(from_os_str))]
        storage_dir: PathBuf,
        #[clap(subcommand)]
        cmd: sandbox::cli::SandboxCommand,
    },
    /// (Experimental) Run static analyses on Move source or bytecode.
    #[clap(name = "experimental")]
    Experimental {
        /// Directory storing Move resources, events, and module bytecodes produced by module publishing
        /// and script execution.
        #[clap(long, default_value = DEFAULT_STORAGE_DIR, parse(from_os_str))]
        storage_dir: PathBuf,
        #[clap(subcommand)]
        cmd: experimental::cli::ExperimentalCommand,
    },
    /// Run integration tests in tests dir.
    #[clap(name = "integration-test", alias = "spectest")]
    IntegrationTest(IntegrationTestCommand),

    /// Check compatibility of modules comparing with remote chain state.
    #[clap(name = "check-compatibility")]
    CompatibilityCheck(CompatibilityCheckCommand),
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct CliOptions {
    #[clap(flatten)]
    move_args: Move,

    #[clap(subcommand)]
    cmd: Commands,
}