//! Variant Knowledge Base

#![warn(missing_docs)]

/* std use */

/* crate use */
use anyhow::Context as _;
use clap::Parser as _;

/* project use */
use vkb::cli;
use vkb::error;

fn main() -> error::Result<()> {
    // Parse argument
    let arguments = cli::Arguments::parse();

    // Setup logger
    stderrlog::new()
        .module(module_path!())
        .quiet(arguments.quiet())
        .verbosity(arguments.verbosity())
        .timestamp(arguments.timestamp())
        .init()
        .context("stderrlog already create a logger")?;

    match arguments.subcommand() {
        cli::SubCommand::Convert(subcmd) => convert(&arguments, subcmd),
        cli::SubCommand::Exploded2unified(subcmd) => exploded2unified(&arguments, subcmd),
    }
}

fn convert(_arguments: &cli::Arguments, _subcmd: &cli::Convert) -> error::Result<()> {
    Ok(())
}

fn exploded2unified(
    _arguments: &cli::Arguments,
    _subcmd: &cli::Exploded2unified,
) -> error::Result<()> {
    Ok(())
}
