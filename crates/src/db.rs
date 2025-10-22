//! Iceberg related tools

/* std use */

/* crate use */

/* module declaration */
pub mod exploded;
pub mod unified;

/* project use */

/* public reexport */

#[derive(std::fmt::Debug, std::clone::Clone)]
#[cfg_attr(feature = "bin", derive(clap::ValueEnum))]
pub enum Table {
    Variant,
    Coverage,
    Symptom,
    Genotyping,
    Gnomad,
    Clinvar,
    Vep,
    Snpeff,
    Annotsv,
}
