//! Exploded databases

/* std use */

/* crate use */

/* project use */
use crate::error;
use crate::iceberg;

use crate::iceberg::catalog::Catalog as _;

pub async fn create<P>(catalog_path: P) -> error::Result<()>
where
    P: std::convert::AsRef<std::path::Path>,
{
    if catalog_path.as_ref().exists() {
        std::fs::remove_dir_all(catalog_path.as_ref())?;
    }

    let catalog =
        iceberg::catalog::SqliteFilesystem::from_path(catalog_path.as_ref(), "exploded").await?;

    let _variant_table = iceberg_rust::table::Table::builder()
        .with_name("variant")
        .with_location("variant")
        .with_schema(iceberg::spec::variant_schema()?)
        .build(&["exploded".to_string()], catalog.clone())
        .await?;

    let _coverage_table = iceberg_rust::table::Table::builder()
        .with_name("coverage")
        .with_location("coverage")
        .with_schema(iceberg::spec::coverage_schema()?)
        .build(&["exploded".to_string()], catalog.clone())
        .await?;

    let _gnomad_table = iceberg_rust::table::Table::builder()
        .with_name("gnomad")
        .with_location("gnomad")
        .with_schema(iceberg::spec::gnomad_schema()?)
        .build(&["exploded".to_string()], catalog.clone())
        .await?;

    let _vepsnpeff_table = iceberg_rust::table::Table::builder()
        .with_name("vepsnpeff")
        .with_location("vepsnpeff")
        .with_schema(iceberg::spec::vepsnpeff_schema()?)
        .build(&["exploded".to_string()], catalog.clone())
        .await?;

    let _annotsv_table = iceberg_rust::table::Table::builder()
        .with_name("annotsv")
        .with_location("annotsv")
        .with_schema(iceberg::spec::annotsv_schema()?)
        .build(&["exploded".to_string()], catalog.clone())
        .await?;

    let _genotyping_table = iceberg_rust::table::Table::builder()
        .with_name("genotyping")
        .with_location("genotyping")
        .with_schema(iceberg::spec::genotyping_schema()?)
        .build(&["exploded".to_string()], catalog.clone())
        .await?;

    let _symptom_table = iceberg_rust::table::Table::builder()
        .with_name("symptom")
        .with_location("symptom")
        .with_schema(iceberg::spec::symptom_schema()?)
        .build(&["exploded".to_string()], catalog.clone())
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    /* std use */

    /* crate use */

    /* project use */
}
