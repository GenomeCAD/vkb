//! Iceberg related tools

/* std use */

/* crate use */

/* module declaration */
pub mod exploded;
pub mod unified;

use std::fmt::Formatter;

/* project use */
use crate::error;
use crate::iceberg;

/* public reexport */

#[derive(std::fmt::Debug, std::clone::Clone, strum::EnumIter)]
#[cfg_attr(feature = "bin", derive(clap::ValueEnum))]
pub enum Table {
    Annotsv,
    Clinvar,
    Coverage,
    Genotyping,
    Gnomad,
    Snpeff,
    Symptom,
    Variant,
    Vep,
}

impl Table {
    pub fn to_name_slice(&self) -> &[&str] {
        match self {
            &Table::Annotsv => &[
                "chromosome",
                "start",
                "end",
                "variant_class",
                "second_chromosome",
                "second_start",
                "second_end",
                "impact",
                "effect",
                "gene_symbol",
                "transcript_id",
                "canonical",
            ],
            Table::Clinvar => &["chromosome", "start"],
            Table::Coverage => &["chromosome", "start", "an"],
            Table::Genotyping => &[
                "chromosome",
                "start",
                "end",
                "reference",
                "alternate",
                "second_chromosome",
                "second_start",
                "second_end",
                "sample_name",
                "genotype",
                "inheritance",
            ],
            Table::Gnomad => &[
                "chromosome",
                "start",
                "reference",
                "alternate",
                "gnomad_af",
                "gnomad_ac",
                "gnomad_an",
            ],
            Table::Snpeff => &[
                "chromosome",
                "start",
                "reference",
                "alternate",
                "impact",
                "effect",
            ],
            Table::Symptom => &[
                "sample_name",
                "affected",
                "preindication",
                "hpos",
                "karyotypic_sex",
            ],
            Table::Variant => &[
                "chromosome",
                "start",
                "end",
                "reference",
                "alternate",
                "variant_type",
                "second_chromosome",
                "second_start",
                "second_end",
            ],
            Table::Vep => &[
                "chromosome",
                "start",
                "reference",
                "alternate",
                "impact",
                "effect",
            ],
        }
    }

    pub fn to_schema(&self) -> error::Result<iceberg_rust::spec::schema::Schema> {
        columns2schema(self.to_name_slice())
    }
}

fn columns2schema(columns: &[&str]) -> error::Result<iceberg_rust::spec::schema::Schema> {
    let mut builder = iceberg_rust::spec::schema::Schema::builder();
    let mut builder_ref = &mut builder;

    for column_name in columns {
        builder_ref = builder_ref.with_struct_field(iceberg::spec::columns()[column_name].clone());
    }

    Ok(builder.build()?)
}

impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut Formatter) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Table::Annotsv => write!(f, "annotsv"),
            Table::Clinvar => write!(f, "clinvar"),
            Table::Coverage => write!(f, "coverage"),
            Table::Genotyping => write!(f, "genotyping"),
            Table::Gnomad => write!(f, "gnomad"),
            Table::Snpeff => write!(f, "snpeff"),
            Table::Symptom => write!(f, "symptom"),
            Table::Variant => write!(f, "variant"),
            Table::Vep => write!(f, "vep"),
        }
    }
}

#[derive(std::fmt::Debug, std::clone::Clone)]
#[cfg_attr(feature = "bin", derive(clap::ValueEnum))]
pub enum PartitionGroup {
    Annotation,
    AnnotationGenome,
    AnnotationGenomeSample,
    AnnotationSample,
    AnnotationSampleGenome,
    Genome,
    GenomeAnnotation,
    GenomeAnnotationSample,
    GenomeSample,
    GenomeSampleAnnotation,
    Sample,
    SampleAnnotation,
    SampleAnnotationGenome,
    SampleGenome,
}

impl PartitionGroup {
    pub const fn to_name_slice(&self) -> &[&str] {
        match self {
            PartitionGroup::Annotation => iceberg::spec::ANNOTATION_PARTITIONS,
            PartitionGroup::AnnotationGenome => constcat::concat_slices!([&str]:
                                     iceberg::spec::ANNOTATION_PARTITIONS,
                                     iceberg::spec::GENOME_PARTITIONS,
            ),
            PartitionGroup::AnnotationGenomeSample => constcat::concat_slices!([&str]:
                                           iceberg::spec::ANNOTATION_PARTITIONS,
                                           iceberg::spec::GENOME_PARTITIONS,
                                           iceberg::spec::SAMPLE_PARTITIONS,
            ),
            PartitionGroup::AnnotationSample => constcat::concat_slices!([&str]:
                                     iceberg::spec::ANNOTATION_PARTITIONS,
                                     iceberg::spec::SAMPLE_PARTITIONS,
            ),
            PartitionGroup::AnnotationSampleGenome => constcat::concat_slices!([&str]:
                                           iceberg::spec::ANNOTATION_PARTITIONS,
                                           iceberg::spec::SAMPLE_PARTITIONS,
                                     iceberg::spec::GENOME_PARTITIONS,
            ),
            PartitionGroup::Genome => iceberg::spec::GENOME_PARTITIONS,
            PartitionGroup::GenomeAnnotation => constcat::concat_slices!([&str]:
                                     iceberg::spec::GENOME_PARTITIONS,
                                     iceberg::spec::ANNOTATION_PARTITIONS,
            ),
            PartitionGroup::GenomeAnnotationSample => constcat::concat_slices!([&str]:
                                     iceberg::spec::GENOME_PARTITIONS,
                                           iceberg::spec::ANNOTATION_PARTITIONS,
                                           iceberg::spec::SAMPLE_PARTITIONS,
            ),
            PartitionGroup::GenomeSample => constcat::concat_slices!([&str]:
                                     iceberg::spec::GENOME_PARTITIONS,
                                     iceberg::spec::SAMPLE_PARTITIONS,
            ),
            PartitionGroup::GenomeSampleAnnotation => constcat::concat_slices!([&str]:
                                           iceberg::spec::GENOME_PARTITIONS,
                                           iceberg::spec::SAMPLE_PARTITIONS,
                                     iceberg::spec::ANNOTATION_PARTITIONS,
            ),
            PartitionGroup::Sample => iceberg::spec::SAMPLE_PARTITIONS,

            _ => todo!(),
        }
    }

    pub fn to_partition_spec(&self) -> error::Result<iceberg_rust::spec::partition::PartitionSpec> {
        partition_groups2partition_spec(self.to_name_slice())
    }
}

impl std::fmt::Display for PartitionGroup {
    fn fmt(&self, f: &mut Formatter) -> std::result::Result<(), std::fmt::Error> {
        match self {
            PartitionGroup::Annotation => write!(f, "annotation"),
            PartitionGroup::AnnotationGenome => write!(f, "annotation_genome"),
            PartitionGroup::AnnotationGenomeSample => write!(f, "annotation_genome_sample"),
            PartitionGroup::AnnotationSample => write!(f, "annotation_sample"),
            PartitionGroup::AnnotationSampleGenome => write!(f, "annotation_sample_genome"),
            PartitionGroup::Genome => write!(f, "genome"),
            PartitionGroup::GenomeAnnotation => write!(f, "genome_annotation"),
            PartitionGroup::GenomeAnnotationSample => write!(f, "genome_annotation_sample"),
            PartitionGroup::GenomeSample => write!(f, "genome_sample"),
            PartitionGroup::GenomeSampleAnnotation => write!(f, "genome_sample_annotation"),
            PartitionGroup::Sample => write!(f, "sample"),
            PartitionGroup::SampleAnnotation => write!(f, "sample_annotation"),
            PartitionGroup::SampleAnnotationGenome => write!(f, "sample_annotation_genome"),
            PartitionGroup::SampleGenome => write!(f, "sample_genome"),
        }
    }
}

fn partition_groups2partition_spec(
    partitions_names: &[&str],
) -> error::Result<iceberg_rust::spec::partition::PartitionSpec> {
    let mut part_builder = iceberg_rust::spec::partition::PartitionSpec::builder();
    for partitition_name in partitions_names {
        part_builder.with_partition_field(iceberg::spec::partitions()[partitition_name].clone());
    }

    Ok(part_builder.build()?)
}
