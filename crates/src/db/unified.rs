//! Exploded databases

/* std use */

/* crate use */

/* project use */
use crate::error;
use crate::iceberg;

use crate::iceberg::catalog::Catalog as _;

pub async fn create<P>(
    catalog_path: P,
    _tables: &[crate::cli::Table],
    _drop_columns: &[String],
) -> error::Result<()>
where
    P: std::convert::AsRef<std::path::Path>,
{
    let catalog =
        iceberg::catalog::SqliteFilesystem::from_path(catalog_path.as_ref(), "unified").await?;

    let schema = iceberg_rust::spec::schema::Schema::builder()
        // Dna view
        .with_struct_field(iceberg::spec::columns()["chromosome"].clone())
        .with_struct_field(iceberg::spec::columns()["start"].clone())
        .with_struct_field(iceberg::spec::columns()["end"].clone())
        .with_struct_field(iceberg::spec::columns()["variant_class"].clone())
        .with_struct_field(iceberg::spec::columns()["reference"].clone())
        .with_struct_field(iceberg::spec::columns()["alternate"].clone())
        // Annotation view
        .with_struct_field(iceberg::spec::columns()["gene_symbol"].clone())
        .with_struct_field(iceberg::spec::columns()["transcript_id"].clone())
        .with_struct_field(iceberg::spec::columns()["canonical"].clone())
        .with_struct_field(iceberg::spec::columns()["impact"].clone())
        .with_struct_field(iceberg::spec::columns()["effect"].clone())
        .with_struct_field(iceberg::spec::columns()["gnomad_af"].clone())
        .with_struct_field(iceberg::spec::columns()["clinvar_clnsig"].clone())
        // Sample view
        .with_struct_field(iceberg::spec::columns()["sample_name"].clone())
        .with_struct_field(iceberg::spec::columns()["genotype"].clone())
        .with_struct_field(iceberg::spec::columns()["inheritance"].clone())
        .with_struct_field(iceberg::spec::columns()["karyotypic_sex"].clone())
        .with_struct_field(iceberg::spec::columns()["preindication"].clone())
        .with_struct_field(iceberg::spec::columns()["hpos"].clone())
        .build()?;

    // Single table
    for (k, v) in iceberg::spec::group_partitions() {
        generate_table(schema.clone(), k, v.as_slice(), catalog.clone()).await?;
    }

    // Double table
    for ((k1, v1), (k2, v2)) in itertools::iproduct!(
        iceberg::spec::group_partitions(),
        iceberg::spec::group_partitions(),
    ) {
        if k1 == k2 {
            continue;
        }

        generate_table(
            schema.clone(),
            &format!("{}_{}", k1, k2),
            v1.iter().chain(v2).cloned().collect::<Vec<_>>().as_slice(),
            catalog.clone(),
        )
        .await?;
    }

    // triple table
    for ((k1, v1), (k2, v2), (k3, v3)) in itertools::iproduct!(
        iceberg::spec::group_partitions(),
        iceberg::spec::group_partitions(),
        iceberg::spec::group_partitions()
    ) {
        if k1 == k2 || k1 == k3 || k2 == k3 {
            continue;
        }

        generate_table(
            schema.clone(),
            &format!("{}_{}_{}", k1, k2, k3),
            v1.iter()
                .chain(v2)
                .chain(v3)
                .cloned()
                .collect::<Vec<_>>()
                .as_slice(),
            catalog.clone(),
        )
        .await?;
    }

    Ok(())
}

async fn generate_table(
    schema: iceberg_rust::spec::schema::Schema,
    name: &str,
    partitions: &[iceberg_rust::spec::partition::PartitionField],
    catalog: std::sync::Arc<dyn iceberg_rust::catalog::Catalog>,
) -> error::Result<()> {
    let mut part_builder = iceberg_rust::spec::partition::PartitionSpec::builder();
    for part in partitions {
        part_builder.with_partition_field(part.clone());
    }

    let _table = iceberg_rust::table::Table::builder()
        .with_name(name)
        .with_location(name)
        .with_schema(schema)
        .with_partition_spec(part_builder.build()?)
        .build(&["cadmos".to_string()], catalog.clone())
        .await?;

    Ok(())
}
