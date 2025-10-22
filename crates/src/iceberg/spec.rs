//! All schema specification

/* std use */

/* crate use */

/* project use */
use crate::error;

/*
Common columns:
- chromosome
- start
- end
- reference
- alternate
- second_chromosome
- second_start
- second_end

*/
const COMMON_FLAG: i32 = 2i32.pow(30);

/*
Variant columns:
- variant_type
*/
const VARIANT_FLAG: i32 = 2i32.pow(29);

/*
Coverage columns:
- an
*/
const COVERAGE_FLAG: i32 = 2i32.pow(28);

/*
Annotation columns:
- gnomad_af
- gnomad_an
- gnomad_ac
- clinvar_clnsig
- impact
- effect
- gene_symbol
- transcript_id
*/
const ANNOTATION_FLAG: i32 = 2i32.pow(27);

/*
Sample columns:
- sample_name
- genotype
- inheritance
- ac
- af
 */
const SAMPLE_FLAG: i32 = 2i32.pow(26);

/*
Symptom columns:
- affected
- preindication
- hpos
- karyotypic_sex
*/
const SYMPTOM_FLAG: i32 = 2i32.pow(25);

const PARTITION_FLAG: i32 = 2i32.pow(24);

const LIST_FLAG: i32 = 2i32.pow(23);

pub(crate) fn columns(
) -> &'static std::collections::HashMap<&'static str, iceberg_rust::spec::types::StructField> {
    static COLUMNS_MAP: std::sync::OnceLock<
        std::collections::HashMap<&str, iceberg_rust::spec::types::StructField>,
    > = std::sync::OnceLock::new();

    COLUMNS_MAP.get_or_init(|| {
        let mut map = std::collections::HashMap::new();

        map.insert(
            "chromosome",
            iceberg_rust::spec::types::StructField::new(
                COMMON_FLAG | 1,
                "chromosome",
                true,
                iceberg_rust::spec::types::Type::Primitive(
                    iceberg_rust::spec::types::PrimitiveType::String,
                ),
                Some("Chromosome name of variant".to_string()),
            ),
        );

        map.insert(
            "start",
            iceberg_rust::spec::types::StructField::new(
                COMMON_FLAG | 2,
                "start",
                true,
                iceberg_rust::spec::types::Type::Primitive(
                    iceberg_rust::spec::types::PrimitiveType::Long,
                ),
                Some("Start of variant in chromosome".to_string()),
            ),
        );

        map.insert(
            "end",
            iceberg_rust::spec::types::StructField::new(
                COMMON_FLAG | 3,
                "end",
                true,
                iceberg_rust::spec::types::Type::Primitive(
                    iceberg_rust::spec::types::PrimitiveType::Long,
                ),
                Some("End of variant in chromosome".to_string()),
            ),
        );

        map.insert(
            "reference",
            iceberg_rust::spec::types::StructField::new(
                COMMON_FLAG | 4,
                "reference",
                true,
                iceberg_rust::spec::types::Type::Primitive(
                    iceberg_rust::spec::types::PrimitiveType::String,
                ),
                Some("Reference sequence of variant".to_string()),
            ),
        );

        map.insert(
            "alternate",
            iceberg_rust::spec::types::StructField::new(
                COMMON_FLAG | 5,
                "alternate",
                true,
                iceberg_rust::spec::types::Type::Primitive(
                    iceberg_rust::spec::types::PrimitiveType::String,
                ),
                Some("Alternate sequence of variant".to_string()),
            ),
        );

        map.insert(
            "second_chromosome",
            iceberg_rust::spec::types::StructField::new(
                COMMON_FLAG | 6,
                "second_chromosome",
                false,
                iceberg_rust::spec::types::Type::Primitive(
                    iceberg_rust::spec::types::PrimitiveType::String,
                ),
                Some("Chromosome name of variant".to_string()),
            ),
        );

        map.insert(
            "second_start",
            iceberg_rust::spec::types::StructField::new(
                COMMON_FLAG | 7,
                "second_start",
                false,
                iceberg_rust::spec::types::Type::Primitive(
                    iceberg_rust::spec::types::PrimitiveType::Long,
                ),
                Some("Start of variant in chromosome".to_string()),
            ),
        );

        map.insert(
            "second_end",
            iceberg_rust::spec::types::StructField::new(
                COMMON_FLAG | 8,
                "second_end",
                false,
                iceberg_rust::spec::types::Type::Primitive(
                    iceberg_rust::spec::types::PrimitiveType::Long,
                ),
                Some("End of variant in chromosome".to_string()),
            ),
        );

        map.insert(
            "variant_class",
            iceberg_rust::spec::types::StructField::new(
                VARIANT_FLAG | 1,
                "variant_class",
                true,
                iceberg_rust::spec::types::Type::Primitive(
                    iceberg_rust::spec::types::PrimitiveType::String,
                ),
                Some("Class of variant".to_string()),
            ),
        );

        map.insert(
            "an",
            iceberg_rust::spec::types::StructField::new(
                COVERAGE_FLAG | 1,
                "an",
                true,
                iceberg_rust::spec::types::Type::Primitive(
                    iceberg_rust::spec::types::PrimitiveType::Long,
                ),
                Some("Allele number of this position".to_string()),
            ),
        );

        map.insert(
            "gnomad_af",
            iceberg_rust::spec::types::StructField::new(
                ANNOTATION_FLAG | 1,
                "gnomad_af",
                false,
                iceberg_rust::spec::types::Type::Primitive(
                    iceberg_rust::spec::types::PrimitiveType::Double,
                ),
                Some("Gnomad allele frequency".to_string()),
            ),
        );

        map.insert(
            "gnomad_an",
            iceberg_rust::spec::types::StructField::new(
                ANNOTATION_FLAG | 2,
                "gnomad_an",
                false,
                iceberg_rust::spec::types::Type::Primitive(
                    iceberg_rust::spec::types::PrimitiveType::Long,
                ),
                Some("Gnomad allele number of this position".to_string()),
            ),
        );

        map.insert(
            "gnomad_ac",
            iceberg_rust::spec::types::StructField::new(
                ANNOTATION_FLAG | 3,
                "gnomad_ac",
                false,
                iceberg_rust::spec::types::Type::Primitive(
                    iceberg_rust::spec::types::PrimitiveType::Long,
                ),
                Some("Gnomad allele count of this position".to_string()),
            ),
        );

        map.insert(
            "clinvar_clnsig",
            iceberg_rust::spec::types::StructField::new(
                ANNOTATION_FLAG | 4,
                "clinvar_clnsig",
                true,
                iceberg_rust::spec::types::Type::Primitive(
                    iceberg_rust::spec::types::PrimitiveType::String,
                ),
                Some("Clinvar clinical signifiance of variant".to_string()),
            ),
        );

        map.insert(
            "impact",
            iceberg_rust::spec::types::StructField::new(
                ANNOTATION_FLAG | 5,
                "impact",
                true,
                iceberg_rust::spec::types::Type::Primitive(
                    iceberg_rust::spec::types::PrimitiveType::String,
                ),
                Some("Impact of variant".to_string()),
            ),
        );

        map.insert(
            "effect",
            iceberg_rust::spec::types::StructField::new(
                ANNOTATION_FLAG | 6,
                "effect",
                true,
                iceberg_rust::spec::types::Type::Primitive(
                    iceberg_rust::spec::types::PrimitiveType::String,
                ),
                Some("Effect of variant".to_string()),
            ),
        );

        map.insert(
            "gene_symbol",
            iceberg_rust::spec::types::StructField::new(
                ANNOTATION_FLAG | 7,
                "gene_symbol",
                true,
                iceberg_rust::spec::types::Type::Primitive(
                    iceberg_rust::spec::types::PrimitiveType::String,
                ),
                Some("Gene symbol associate to impact and effect".to_string()),
            ),
        );

        map.insert(
            "transcript_id",
            iceberg_rust::spec::types::StructField::new(
                ANNOTATION_FLAG | 8,
                "transcript_id",
                true,
                iceberg_rust::spec::types::Type::Primitive(
                    iceberg_rust::spec::types::PrimitiveType::String,
                ),
                Some("transcript id associate to impact and effect".to_string()),
            ),
        );

        map.insert(
            "sample_name",
            iceberg_rust::spec::types::StructField::new(
                SAMPLE_FLAG | 1,
                "sample_name",
                false,
                iceberg_rust::spec::types::Type::Primitive(
                    iceberg_rust::spec::types::PrimitiveType::String,
                ),
                Some("sample_name".to_string()),
            ),
        );

        map.insert(
            "genotype",
            iceberg_rust::spec::types::StructField::new(
                SAMPLE_FLAG | 2,
                "genotype",
                true,
                iceberg_rust::spec::types::Type::Primitive(
                    iceberg_rust::spec::types::PrimitiveType::String,
                ),
                Some("Genotype of sample, variant pair".to_string()),
            ),
        );

        map.insert(
            "inheritance",
            iceberg_rust::spec::types::StructField::new(
                SAMPLE_FLAG | 3,
                "inheritance",
                true,
                iceberg_rust::spec::types::Type::Primitive(
                    iceberg_rust::spec::types::PrimitiveType::String,
                ),
                Some("Inheritance mode of sample, variant pair".to_string()),
            ),
        );

        map.insert(
            "ac",
            iceberg_rust::spec::types::StructField::new(
                SAMPLE_FLAG | 4,
                "ac",
                false,
                iceberg_rust::spec::types::Type::Primitive(
                    iceberg_rust::spec::types::PrimitiveType::Long,
                ),
                Some("Allele count of variant".to_string()),
            ),
        );

        map.insert(
            "af",
            iceberg_rust::spec::types::StructField::new(
                SAMPLE_FLAG | 5,
                "af",
                false,
                iceberg_rust::spec::types::Type::Primitive(
                    iceberg_rust::spec::types::PrimitiveType::Double,
                ),
                Some("Allele frequency of variant".to_string()),
            ),
        );

        map.insert(
            "affected",
            iceberg_rust::spec::types::StructField::new(
                SYMPTOM_FLAG | 1,
                "affected",
                false,
                iceberg_rust::spec::types::Type::Primitive(
                    iceberg_rust::spec::types::PrimitiveType::Boolean,
                ),
                Some("Sample are affected or not".to_string()),
            ),
        );

        map.insert(
            "preindication",
            iceberg_rust::spec::types::StructField::new(
                SYMPTOM_FLAG | 2,
                "preindication",
                true,
                iceberg_rust::spec::types::Type::Primitive(
                    iceberg_rust::spec::types::PrimitiveType::String,
                ),
                Some("Preindication of sample".to_string()),
            ),
        );

        map.insert(
            "hpos",
            iceberg_rust::spec::types::StructField::new(
                SYMPTOM_FLAG | 3,
                "hpos",
                true,
                iceberg_rust::spec::types::Type::List(iceberg_rust::spec::types::ListType {
                    element_id: SYMPTOM_FLAG | LIST_FLAG | 1,
                    element_required: true,
                    element: std::boxed::Box::new(iceberg_rust::spec::types::Type::Primitive(
                        iceberg_rust::spec::types::PrimitiveType::String,
                    )),
                }),
                Some("Hpos of sample".to_string()),
            ),
        );

        map.insert(
            "karyotypic_sex",
            iceberg_rust::spec::types::StructField::new(
                SYMPTOM_FLAG | 4,
                "karyotypic_sex",
                true,
                iceberg_rust::spec::types::Type::Primitive(
                    iceberg_rust::spec::types::PrimitiveType::String,
                ),
                Some("Karyotypic sex of sample".to_string()),
            ),
        );

        map
    })
}

const MAX_NUMBER_OF_POSITION_IN_PARTITION: u32 = 2u32.pow(20); // To increase number of partition reduce this
const NUMBER_OF_SYMBOL_PARTITION: u32 = 2u32.pow(8); // To increase number of partition increase this

pub(crate) fn partitions(
) -> &'static std::collections::HashMap<&'static str, iceberg_rust::spec::partition::PartitionField>
{
    static PARTITIONS_MAP: std::sync::OnceLock<
        std::collections::HashMap<&str, iceberg_rust::spec::partition::PartitionField>,
    > = std::sync::OnceLock::new();

    PARTITIONS_MAP.get_or_init(|| {
        let mut map = std::collections::HashMap::new();

        map.insert(
            "chromosome",
            iceberg_rust::spec::partition::PartitionField::new(
                COMMON_FLAG | 1,
                COMMON_FLAG | PARTITION_FLAG | 1,
                "chromosome",
                iceberg_rust::spec::partition::Transform::Identity,
            ),
        );

        map.insert(
            "start",
            iceberg_rust::spec::partition::PartitionField::new(
                COMMON_FLAG | 2,
                COMMON_FLAG | PARTITION_FLAG | 2,
                "start_part",
                iceberg_rust::spec::partition::Transform::Truncate(
                    MAX_NUMBER_OF_POSITION_IN_PARTITION,
                ),
            ),
        );

        map.insert(
            "variant_class",
            iceberg_rust::spec::partition::PartitionField::new(
                VARIANT_FLAG | 1,
                VARIANT_FLAG | PARTITION_FLAG | 1,
                "variant_class",
                iceberg_rust::spec::partition::Transform::Identity,
            ),
        );

        map.insert(
            "clinvar_clnsig",
            iceberg_rust::spec::partition::PartitionField::new(
                ANNOTATION_FLAG | 4,
                ANNOTATION_FLAG | PARTITION_FLAG | 4,
                "clinvar_clnsig",
                iceberg_rust::spec::partition::Transform::Identity,
            ),
        );

        map.insert(
            "impact",
            iceberg_rust::spec::partition::PartitionField::new(
                ANNOTATION_FLAG | 5,
                ANNOTATION_FLAG | PARTITION_FLAG | 5,
                "impact",
                iceberg_rust::spec::partition::Transform::Identity,
            ),
        );

        map.insert(
            "effect",
            iceberg_rust::spec::partition::PartitionField::new(
                ANNOTATION_FLAG | 6,
                ANNOTATION_FLAG | PARTITION_FLAG | 6,
                "effect",
                iceberg_rust::spec::partition::Transform::Identity,
            ),
        );

        map.insert(
            "gene_symbol",
            iceberg_rust::spec::partition::PartitionField::new(
                ANNOTATION_FLAG | 7,
                ANNOTATION_FLAG | PARTITION_FLAG | 7,
                "gene_symbol",
                iceberg_rust::spec::partition::Transform::Bucket(NUMBER_OF_SYMBOL_PARTITION),
            ),
        );

        map.insert(
            "sample_name",
            iceberg_rust::spec::partition::PartitionField::new(
                SAMPLE_FLAG | 1,
                SAMPLE_FLAG | PARTITION_FLAG | 1,
                "sample_name",
                iceberg_rust::spec::partition::Transform::Identity,
            ),
        );

        map.insert(
            "genotype",
            iceberg_rust::spec::partition::PartitionField::new(
                SAMPLE_FLAG | 2,
                SAMPLE_FLAG | PARTITION_FLAG | 2,
                "genotype",
                iceberg_rust::spec::partition::Transform::Identity,
            ),
        );

        map.insert(
            "inheritance",
            iceberg_rust::spec::partition::PartitionField::new(
                SAMPLE_FLAG | 3,
                SAMPLE_FLAG | PARTITION_FLAG | 3,
                "inheritance",
                iceberg_rust::spec::partition::Transform::Identity,
            ),
        );

        map.insert(
            "preindication",
            iceberg_rust::spec::partition::PartitionField::new(
                SYMPTOM_FLAG | 2,
                SYMPTOM_FLAG | PARTITION_FLAG | 2,
                "preindication",
                iceberg_rust::spec::partition::Transform::Identity,
            ),
        );

        map.insert(
            "karyotypic_sex",
            iceberg_rust::spec::partition::PartitionField::new(
                SYMPTOM_FLAG | 4,
                SYMPTOM_FLAG | PARTITION_FLAG | 4,
                "karyotypic_sex",
                iceberg_rust::spec::partition::Transform::Identity,
            ),
        );

        map
    })
}

pub(crate) fn group_partitions() -> &'static std::collections::HashMap<
    &'static str,
    Vec<iceberg_rust::spec::partition::PartitionField>,
> {
    static GROUP_PARTITIONS_MAP: std::sync::OnceLock<
        std::collections::HashMap<&str, Vec<iceberg_rust::spec::partition::PartitionField>>,
    > = std::sync::OnceLock::new();

    GROUP_PARTITIONS_MAP.get_or_init(|| {
        let mut map = std::collections::HashMap::new();

        map.insert(
            "genome",
            vec![
                partitions()["chromosome"].clone(),
                partitions()["position"].clone(),
                partitions()["variant_class"].clone(),
            ],
        );

        map.insert(
            "annotation",
            vec![
                partitions()["clin_sig"].clone(),
                partitions()["impact"].clone(),
                partitions()["effect"].clone(),
            ],
        );

        map.insert(
            "sample",
            vec![
                partitions()["preindication"].clone(),
                partitions()["karyotypic_sex"].clone(),
                partitions()["inher"].clone(),
                partitions()["gt"].clone(),
            ],
        );

        map
    })
}

pub fn variant_schema() -> error::Result<iceberg_rust::spec::schema::Schema> {
    Ok(iceberg_rust::spec::schema::Schema::builder()
        .with_struct_field(columns()["chromosome"].clone())
        .with_struct_field(columns()["start"].clone())
        .with_struct_field(columns()["end"].clone())
        .with_struct_field(columns()["reference"].clone())
        .with_struct_field(columns()["alternate"].clone())
        .with_struct_field(columns()["variant_class"].clone())
        .with_struct_field(columns()["second_chromosome"].clone())
        .with_struct_field(columns()["second_start"].clone())
        .with_struct_field(columns()["second_end"].clone())
        .build()?)
}

pub fn coverage_schema() -> error::Result<iceberg_rust::spec::schema::Schema> {
    Ok(iceberg_rust::spec::schema::Schema::builder()
        .with_struct_field(columns()["chromosome"].clone())
        .with_struct_field(columns()["start"].clone())
        .with_struct_field(columns()["an"].clone())
        .build()?)
}

pub fn gnomad_schema() -> error::Result<iceberg_rust::spec::schema::Schema> {
    Ok(iceberg_rust::spec::schema::Schema::builder()
        .with_struct_field(columns()["chromosome"].clone())
        .with_struct_field(columns()["start"].clone())
        .with_struct_field(columns()["reference"].clone())
        .with_struct_field(columns()["alternate"].clone())
        .with_struct_field(columns()["gnomad_af"].clone())
        .with_struct_field(columns()["gnomad_an"].clone())
        .with_struct_field(columns()["gnomad_ac"].clone())
        .build()?)
}

pub fn vepsnpeff_schema() -> error::Result<iceberg_rust::spec::schema::Schema> {
    Ok(iceberg_rust::spec::schema::Schema::builder()
        .with_struct_field(columns()["chromosome"].clone())
        .with_struct_field(columns()["start"].clone())
        .with_struct_field(columns()["reference"].clone())
        .with_struct_field(columns()["alternate"].clone())
        .with_struct_field(columns()["impact"].clone())
        .with_struct_field(columns()["effect"].clone())
        .build()?)
}

pub fn annotsv_schema() -> error::Result<iceberg_rust::spec::schema::Schema> {
    Ok(iceberg_rust::spec::schema::Schema::builder()
        .with_struct_field(columns()["chromosome"].clone())
        .with_struct_field(columns()["start"].clone())
        .with_struct_field(columns()["end"].clone())
        .with_struct_field(columns()["second_chromosome"].clone())
        .with_struct_field(columns()["second_start"].clone())
        .with_struct_field(columns()["second_end"].clone())
        .with_struct_field(columns()["impact"].clone())
        .with_struct_field(columns()["effect"].clone())
        .build()?)
}

pub fn genotyping_schema() -> error::Result<iceberg_rust::spec::schema::Schema> {
    Ok(iceberg_rust::spec::schema::Schema::builder()
        .with_struct_field(columns()["chromosome"].clone())
        .with_struct_field(columns()["start"].clone())
        .with_struct_field(columns()["end"].clone())
        .with_struct_field(columns()["reference"].clone())
        .with_struct_field(columns()["alternate"].clone())
        .with_struct_field(columns()["second_chromosome"].clone())
        .with_struct_field(columns()["second_start"].clone())
        .with_struct_field(columns()["second_end"].clone())
        .with_struct_field(columns()["sample_name"].clone())
        .with_struct_field(columns()["genotype"].clone())
        .with_struct_field(columns()["inheritance"].clone())
        .build()?)
}

pub fn symptom_schema() -> error::Result<iceberg_rust::spec::schema::Schema> {
    Ok(iceberg_rust::spec::schema::Schema::builder()
        .with_struct_field(columns()["sample_name"].clone())
        .with_struct_field(columns()["affected"].clone())
        .with_struct_field(columns()["preindication"].clone())
        .with_struct_field(columns()["hpos"].clone())
        .with_struct_field(columns()["karyotypic_sex"].clone())
        .build()?)
}
