//! Command Line Interface of vkb

/* std use */

/* crate use */

/* project use */
use crate::error;

#[derive(clap::Parser, std::fmt::Debug)]
#[clap(
    name = "vkb",
    bin_name = "vkb",
    version = "0.1.0",
    author = "Pierre Marijon <pierre.marijon@genomecad.fr>"
)]
pub struct Arguments {
    // Specific parameter
    /// Catalog path
    #[clap(short = 'c', long = "catalog-path")]
    catalog_path: std::path::PathBuf,

    /// SubCommand command
    #[clap(subcommand)]
    subcommand: SubCommand,

    // Generic parameter
    /// Number of threads use if not set try to use maximum
    #[clap(short = 't', long = "thread")]
    threads: Option<usize>,

    /// Silence all output
    #[clap(short = 'q', long = "quiet")]
    quiet: bool,

    /// Verbose mode (-v, -vv, -vvv, etc)
    #[clap(short = 'v', long = "verbosity", action = clap::ArgAction::Count)]
    verbosity: u8,

    /// Timestamp (sec, ms, ns, none)
    #[clap(short = 'T', long = "timestamp")]
    ts: Option<stderrlog::Timestamp>,
}

impl Arguments {
    /// Catalog path
    pub fn catalog_path(&self) -> &std::path::PathBuf {
        &self.catalog_path
    }

    /// Subcommand
    pub fn subcommand(&self) -> &SubCommand {
        &self.subcommand
    }

    /// Get number of threads
    pub fn threads(&self) -> error::Result<usize> {
        Ok(self
            .threads
            .unwrap_or(std::thread::available_parallelism()?.get()))
    }

    /// Get verbosity level
    pub fn verbosity(&self) -> usize {
        self.verbosity as usize
    }

    /// Get quiet
    pub fn quiet(&self) -> bool {
        self.quiet
    }

    /// Get timestamp granularity
    pub fn timestamp(&self) -> stderrlog::Timestamp {
        self.ts.unwrap_or(stderrlog::Timestamp::Off)
    }
}

#[derive(clap::Subcommand, std::fmt::Debug, std::clone::Clone)]
pub enum SubCommand {
    /// Insert classic bioinformatic information in exploded database
    Convert(Convert),

    /// Generate a unified table from exploded database
    Exploded2unified(Exploded2unified),
}

#[derive(clap::Parser, std::fmt::Debug, std::clone::Clone)]
pub struct Convert {
    /// Input path
    #[clap(short = 'i', long = "input-path")]
    input_path: std::path::PathBuf,

    /// Input type
    #[clap(short = 't', long = "type")]
    input_type: InputType,

    /// Tables where data are write
    #[clap(short = 'T', long = "tables")]
    tables: Vec<Table>,
}

impl Convert {
    pub fn input_path(&self) -> &std::path::PathBuf {
        &self.input_path
    }

    pub fn input_type(&self) -> &InputType {
        &self.input_type
    }

    pub fn tables(&self) -> &[Table] {
        &self.tables
    }
}

#[derive(clap::Parser, std::fmt::Debug, std::clone::Clone)]
pub struct Exploded2unified {
    /// Tables use to create unified table
    #[clap(short = 'T', long = "tables")]
    tables: Vec<Table>,

    /// Name of columns to drop
    #[clap(short = 'd', long = "drop-columns")]
    drop_columns: Vec<String>,

    /// Type of aggregation
    #[clap(short = 'a', long = "aggregation")]
    aggregation: Aggregation,

    /// Output path
    #[clap(short = 'o', long = "output-path")]
    output_path: std::path::PathBuf,
}

impl Exploded2unified {
    pub fn tables(&self) -> &[Table] {
        &self.tables
    }

    pub fn drop_columns(&self) -> &[String] {
        &self.drop_columns
    }

    pub fn aggregation(&self) -> &Aggregation {
        &self.aggregation
    }

    pub fn output_path(&self) -> &std::path::PathBuf {
        &self.output_path
    }
}

#[derive(clap::ValueEnum, std::fmt::Debug, std::clone::Clone)]
pub enum Aggregation {
    Genotype,
}

#[derive(clap::ValueEnum, std::fmt::Debug, std::clone::Clone)]
pub enum InputType {
    Gvcf,
    Vcf,
    Tsv,
    Phenopacket,
    Json,
}

#[derive(clap::ValueEnum, std::fmt::Debug, std::clone::Clone)]
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
