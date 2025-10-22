//! Variant Knowledge Base

#![warn(missing_docs)]

/* std use */

/* crate use */
use anyhow::Context as _;
use clap::Parser as _;

/* project use */
use vkb::cli;
use vkb::db;
use vkb::error;
use vkb::iceberg;

use vkb::iceberg::catalog::Catalog as _;

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

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(arguments.threads()?)
        .thread_name("vkb_worker")
        .build()?;

    match arguments.subcommand() {
        cli::SubCommand::Convert(subcmd) => {
            runtime.block_on(async { convert(&arguments, subcmd).await })
        }
        cli::SubCommand::Aggregate(subcmd) => {
            runtime.block_on(async { aggregate(&arguments, subcmd).await })
        }
    }
}

async fn convert(arguments: &cli::Arguments, subcmd: &cli::Convert) -> error::Result<()> {
    if subcmd.overwrite() || !arguments.catalog_path().exists() {
        log::info!("Create catalog");
        db::exploded::create(arguments.catalog_path()).await?;
    }

    let _catalog =
        iceberg::catalog::SqliteFilesystem::from_path(arguments.catalog_path(), "exploded").await?;

    Ok(())
}

async fn aggregate(arguments: &cli::Arguments, subcmd: &cli::Aggregate) -> error::Result<()> {
    if subcmd.output_path().exists() {
        log::info!("Overwrite output catalog");
        std::fs::remove_dir_all(subcmd.output_path())?;
    }

    db::unified::create(subcmd.output_path(), subcmd.tables(), subcmd.drop_columns()).await?;

    let _exploded_catalog =
        iceberg::catalog::SqliteFilesystem::from_path(arguments.catalog_path(), "exploded").await?;
    let _unified_catalog =
        iceberg::catalog::SqliteFilesystem::from_path(subcmd.output_path(), "unified").await?;

    Ok(())
}
