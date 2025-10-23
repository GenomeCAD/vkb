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
use vkb::parser;

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
        log::info!("Start clean output path.");
        std::fs::remove_dir_all(subcmd.output_path())?;
        log::info!("End clean output path.");
    }

    log::info!("Start create unified database.");
    db::unified::create(
        subcmd.output_path(),
        subcmd.tables(),
        subcmd.partitions(),
        subcmd.drop_columns(),
    )
    .await?;
    log::info!("End create unified database.");

    let _exploded_catalog =
        iceberg::catalog::SqliteFilesystem::from_path(arguments.catalog_path(), "exploded").await?;
    let unified_catalog =
        iceberg::catalog::SqliteFilesystem::from_path(subcmd.output_path(), "unified").await?;

    if let Some(input_path) = subcmd.input_path() {
        log::info!(
            "Start insertion of {} data in unified database.",
            input_path.display()
        );
        // Read input csv and add data in unified
        let namespace =
            iceberg_rust::catalog::namespace::Namespace::try_new(&["unified".to_string()])?;

        let mut read_builder = parser::ArrowCsvReader::<std::path::PathBuf>::builder();
        let mut read_builder_ref = read_builder
            .input_path(input_path.clone())
            .tables(subcmd.tables().to_vec());

        match std::fs::File::open(input_path)
            .map(|r| Box::new(r) as Box<dyn std::io::Read>)
            .map(niffler::sniff)??
            .1
        {
            niffler::Format::No => read_builder_ref = read_builder_ref.gziped(false),
            niffler::Format::Gzip => read_builder_ref = read_builder_ref.gziped(true),
            compression @ _ => todo!("Compression format {:?} not support", compression),
        }
        let reader = read_builder_ref.build()?;

        for table_id in unified_catalog.list_tabulars(&namespace).await?.iter() {
            log::info!("Start insertion in table {}.", table_id);

            let mut table = match unified_catalog.clone().load_tabular(&table_id).await? {
                iceberg_rust::catalog::tabular::Tabular::Table(t) => t,
                _ => todo!("View or MaterializeView are not support"),
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
            input_path.display()
        );
    } else {
        // Use exploded catalog to generate data to integrate in unified
        todo!("Aggregation of exploded db aren't yet support")
    }

    Ok(())
}
