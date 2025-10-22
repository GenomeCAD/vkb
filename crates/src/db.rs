//! Iceberg related tools

/* std use */

/* crate use */

/* module declaration */
pub mod exploded;
pub mod unified;

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
    pub fn list_columns(&self) -> &[&str] {
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
        let mut builder = iceberg_rust::spec::schema::Schema::builder();
        let mut builder_ref = &mut builder;

        for column_name in self.list_columns() {
            builder_ref =
                builder_ref.with_struct_field(iceberg::spec::columns()[column_name].clone());
        }

        Ok(builder.build()?)
    }
}

impl std::string::ToString for Table {
    fn to_string(&self) -> String {
	match self {
	Table::Annotsv => "annotsv".to_string(),
        Table::Clinvar => "clinvar".to_string(),
        Table::Coverage => "coverage".to_string(),
        Table::Genotyping => "genotyping".to_string(),
        Table::Gnomad => "gnomad".to_string(),
        Table::Snpeff => "snpeff".to_string(),
        Table::Symptom => "symptom".to_string(),
        Table::Variant => "variant".to_string(),
        Table::Vep => "vep".to_string(),
	}
    }
}


#[derive(std::fmt::Debug, std::clone::Clone)]
#[cfg_attr(feature = "bin", derive(clap::ValueEnum))]
pub enum PartionGroup {
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
