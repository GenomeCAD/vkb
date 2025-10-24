//! Variant Knowledge Base

#![warn(missing_docs)]

/* std use */

/* crate use */
use anyhow::Context as _;
use clap::Parser as _;

/* project use */
use vkb::catalog;
use vkb::cli;
use vkb::error;
use vkb::iceberg;
use vkb::parser;

#[cfg(feature = "rest_server")]
use vkb::request;

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
        cli::SubCommand::Convert(subcmd) => runtime.block_on(async { convert(subcmd).await }),
        cli::SubCommand::Aggregate(subcmd) => runtime.block_on(async { aggregate(subcmd).await }),
        cli::SubCommand::Csv2unified(subcmd) => {
            runtime.block_on(async { csv2unified(subcmd).await })
        }
        #[cfg(feature = "rest_server")]
        cli::SubCommand::Beacon(subcmd) => runtime.block_on(async { beacon(subcmd).await }),
    }
}

async fn convert(subcmd: &cli::Convert) -> error::Result<()> {
    if subcmd.overwrite() || !subcmd.exploded_path().exists() {
        log::info!("Create catalog");
        catalog::exploded::create(subcmd.exploded_path()).await?;
    }

    let _catalog =
        iceberg::catalog::SqliteFilesystem::from_path(subcmd.exploded_path(), "exploded").await?;

    Ok(())
}

async fn aggregate(subcmd: &cli::Aggregate) -> error::Result<()> {
    if subcmd.unified_path().exists() {
        log::info!("Start clean output path.");
        std::fs::remove_dir_all(subcmd.unified_path())?;
        log::info!("End clean output path.");
    }

    log::info!("Start create unified database.");
    catalog::unified::create(
        subcmd.unified_path(),
        subcmd.tables(),
        subcmd.partitions(),
        subcmd.drop_columns(),
    )
    .await?;
    log::info!("End create unified database.");

    let _exploded_catalog =
        iceberg::catalog::SqliteFilesystem::from_path(subcmd.exploded_path(), "exploded").await?;
    let _unified_catalog =
        iceberg::catalog::SqliteFilesystem::from_path(subcmd.unified_path(), "unified").await?;

    // Use exploded catalog to generate data to integrate in unified
    log::error!("Aggregation of exploded catalog aren't yet support");

    Ok(())
}

async fn csv2unified(subcmd: &cli::Csv2unified) -> error::Result<()> {
    let unified_catalog =
        iceberg::catalog::SqliteFilesystem::from_path(subcmd.unified_path(), "unified").await?;

    log::info!(
        "Start insertion of {} data in unified database.",
        subcmd.input_path().display()
    );
    // Read input csv and add data in unified
    let namespace = iceberg_rust::catalog::namespace::Namespace::try_new(&["unified".to_string()])?;

    let mut read_builder = parser::ArrowCsvReader::<std::path::PathBuf>::builder();
    let mut read_builder_ref = read_builder
        .input_path(subcmd.input_path().clone())
        .tables(subcmd.tables().to_vec());

    match std::fs::File::open(subcmd.input_path())
        .map(|r| Box::new(r) as Box<dyn std::io::Read>)
        .map(niffler::sniff)??
        .1
    {
        niffler::Format::No => read_builder_ref = read_builder_ref.gziped(false),
        niffler::Format::Gzip => read_builder_ref = read_builder_ref.gziped(true),
        compression => todo!("Compression format {:?} not support", compression),
    }
    let reader = read_builder_ref.build()?;

    for table_id in unified_catalog.list_tabulars(&namespace).await?.iter() {
        log::info!("Start insertion in table {}.", table_id);

        let mut table = match unified_catalog.clone().load_tabular(table_id).await? {
            iceberg_rust::catalog::tabular::Tabular::Table(t) => t,
            _ => anyhow::bail!("View or MaterializeView are not support"),
        };

        let data_files = iceberg_rust::arrow::write::write_parquet_partitioned(
            &table,
            reader.clone().to_stream().await?,
            None,
        )
        .await?;

        let transaction = table.new_transaction(None);
        transaction.append_data(data_files).commit().await?;
        log::info!("End insertion in table {}.", table_id);
    }

    log::info!(
        "End insertion of {} data in unified database.",
        subcmd.input_path().display()
    );

    Ok(())
}

#[cfg(feature = "rest_server")]
async fn beacon(subcmd: &cli::Beacon) -> error::Result<()> {
    let config = rocket::Config {
        port: subcmd.port(),
        address: subcmd.address()?,
        log_level: rocket::config::LogLevel::Debug,
        ..rocket::Config::debug_default()
    };

    log::info!("Start server with configuration {:?}", config);

    let _rocket = rocket::custom(&config)
        .attach(request::stage())
        .launch()
        .await?;
    Ok(())
}
