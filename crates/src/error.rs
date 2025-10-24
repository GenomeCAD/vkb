//! Error definition of vkb

/* std use */

/* crate use */

/* project use */

/// Enum to define error
#[derive(std::fmt::Debug, thiserror::Error)]
pub enum Error {
    #[cfg(feature = "rest_server")]
    #[error(transparent)]
    IpAddrParse(#[from] core::net::AddrParseError),

    #[error(transparent)]
    StdIo(#[from] std::io::Error),

    #[error(transparent)]
    Log(#[from] log::SetLoggerError),

    #[error(transparent)]
    Iceberg(#[from] iceberg_rust::error::Error),

    #[error(transparent)]
    IcebergSql(#[from] iceberg_sql_catalog::error::Error),

    #[error(transparent)]
    Sqlx(#[from] sqlx::error::Error),

    #[cfg(feature = "request")]
    #[error(transparent)]
    DataFusion(#[from] datafusion::error::DataFusionError),
}

/// Alias of result
pub type Result<T> = anyhow::Result<T>;
