//! Iceberg catalog tools

/* std use */

/* crate use */
use sqlx::migrate::MigrateDatabase as _;

/* project use */
use crate::error;

#[allow(async_fn_in_trait)]
pub trait Catalog {
    async fn from_path<P>(
        catalog_path: P,
        catalog_name: &str,
    ) -> error::Result<std::sync::Arc<dyn iceberg_rust::catalog::Catalog>>
    where
        P: std::convert::AsRef<std::path::Path>;
}

#[derive(std::default::Default)]
pub struct SqliteFilesystem;

impl Catalog for SqliteFilesystem {
    async fn from_path<P>(
        catalog_path: P,
        catalog_name: &str,
    ) -> error::Result<std::sync::Arc<dyn iceberg_rust::catalog::Catalog>>
    where
        P: std::convert::AsRef<std::path::Path>,
    {
        let sql_uri = format!(
            "sqlite:{}/iceberg_{}_catalog.db",
            catalog_path.as_ref().display(),
            catalog_name,
        );

        let warehouse_location = format!("{}", catalog_path.as_ref().display());

        if !catalog_path.as_ref().exists() {
            std::fs::create_dir_all(catalog_path)?;
        }
        sqlx::Sqlite::create_database(&sql_uri).await?;

        let catalog = std::sync::Arc::new(
            iceberg_sql_catalog::SqlCatalog::new(
                &sql_uri,
                catalog_name,
                iceberg_rust::object_store::ObjectStoreBuilder::filesystem(warehouse_location),
            )
            .await?,
        );

        Ok(catalog)
    }
}

#[cfg(test)]
mod tests {
    /* std use */

    /* crate use */

    /* project use */
}
