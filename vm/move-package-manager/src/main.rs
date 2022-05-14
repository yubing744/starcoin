// Copyright (c) The Starcoin Core Contributors
// SPDX-License-Identifier: Apache-2.0

pub mod lib;

use lib::{CliOptions, Commands};

fn main() -> Result<()> {
    let error_descriptions: ErrorMapping =
        bcs_ext::from_bytes(stdlib::ERROR_DESCRIPTIONS).expect("Decode err map failed");
    let args: CliOptions = CliOptions::parse();

    let move_args = &args.move_args;
    let natives = starcoin_natives();

    match args.cmd {
        Commands::IntegrationTest(cmd) => run_integration_test(args.move_args, cmd),
        Commands::Package { cmd } => handle_package_commands(
            &move_args.package_path,
            move_args.build_config.clone(),
            &cmd,
            natives,
        ),
        Commands::Sandbox { storage_dir, cmd } => cmd.handle_command(
            natives,
            &genesis_config::G_LATEST_GAS_SCHEDULE,
            &error_descriptions,
            move_args,
            &storage_dir,
        ),
        Commands::Experimental { storage_dir, cmd } => cmd.handle_command(move_args, &storage_dir),
        Commands::Release(releasement) => handle_release(move_args, releasement),
        Commands::CompatibilityCheck(cmd) => handle_compatibility_check(move_args, cmd),
    }
}
