//! Exploded databases

/* std use */

/* crate use */
use itertools::Itertools as _;

/* project use */
use crate::db;
use crate::error;
use crate::iceberg;

use crate::iceberg::catalog::Catalog as _;

pub async fn create<P>(
    catalog_path: P,
    tables: &[db::Table],
    partitions: &[db::PartitionGroup],
    _drop_columns: &[String],
) -> error::Result<()>
where
    P: std::convert::AsRef<std::path::Path>,
{
    let catalog =
        iceberg::catalog::SqliteFilesystem::from_path(catalog_path.as_ref(), "unified").await?;

    let columns = tables
        .iter()
        .flat_map(db::Table::to_name_slice)
        .unique()
        .cloned()
        .collect::<Vec<&str>>();

    let schema = db::columns2schema(&columns)?;

    for part in partitions {
        let _table = iceberg_rust::table::Table::builder()
            .with_name(part.to_string())
            .with_location(part.to_string())
            .with_schema(schema.clone())
            .with_partition_spec(part.to_partition_spec()?)
            .build(&["unified".to_string()], catalog.clone())
            .await?;
    }

    Ok(())
}
