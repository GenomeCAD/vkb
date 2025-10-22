//! Exploded databases

/* std use */

/* crate use */
use strum::IntoEnumIterator as _;

/* project use */
use crate::db;
use crate::error;
use crate::iceberg;

use crate::iceberg::catalog::Catalog as _;

pub async fn create<P>(catalog_path: P) -> error::Result<()>
where
    P: std::convert::AsRef<std::path::Path>,
{
    let catalog =
        iceberg::catalog::SqliteFilesystem::from_path(catalog_path.as_ref(), "exploded").await?;

    for table in db::Table::iter() {
        let _table = iceberg_rust::table::Table::builder()
            .with_name(table.to_string())
            .with_location(table.to_string())
            .with_schema(table.to_schema()?)
            .build(&["exploded".to_string()], catalog.clone())
            .await?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    /* std use */

    /* crate use */

    /* project use */
}
