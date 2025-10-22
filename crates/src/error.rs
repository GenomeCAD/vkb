//! Error definition of vkb

/* std use */

/* crate use */

/* project use */

/// Enum to define error
#[derive(std::fmt::Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Log(#[from] log::SetLoggerError),

    #[error(transparent)]
    StdIo(#[from] std::io::Error),

    #[error(transparent)]
    Iceberg(#[from] iceberg_rust::error::Error),

    #[error(transparent)]
    IcebergSql(#[from] iceberg_sql_catalog::error::Error),

    #[error(transparent)]
    Sqlx(#[from] sqlx::error::Error),
}

/// Alias of result
pub type Result<T> = anyhow::Result<T>;
