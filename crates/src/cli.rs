//! Command Line Interface of vkb

/* std use */

/* crate use */

/* project use */
use crate::catalog;
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
    /// Insert classic bioinformatic information in exploded catalog
    Convert(Convert),

    /// Generate a unified table from exploded catalog
    Aggregate(Aggregate),

    /// Use a csv to populate unified table
    Csv2unified(Csv2unified),

    #[cfg(feature = "rest_server")]
    /// Start a beacon REST server on unified catalog
    Beacon(Beacon),
}

#[derive(clap::Parser, std::fmt::Debug, std::clone::Clone)]
pub struct Convert {
    /// Exploded catalog path
    #[clap(short = 'e', long = "exploded-path")]
    exploded_path: std::path::PathBuf,

    /// Input path
    #[clap(short = 'i', long = "input-path")]
    input_path: std::path::PathBuf,

    /// Input type
    #[clap(short = 't', long = "type")]
    input_type: InputType,

    /// Tables where data are write
    #[clap(short = 'T', long = "tables")]
    tables: Vec<catalog::Table>,

    /// Overwrite catalog
    #[clap(short = 'o', long = "overwrite")]
    overwrite: bool,
}

impl Convert {
    pub fn exploded_path(&self) -> &std::path::PathBuf {
        &self.exploded_path
    }

    pub fn input_path(&self) -> &std::path::PathBuf {
        &self.input_path
    }

    pub fn input_type(&self) -> &InputType {
        &self.input_type
    }

    pub fn tables(&self) -> &[catalog::Table] {
        &self.tables
    }

    pub fn overwrite(&self) -> bool {
        self.overwrite
    }
}

#[derive(clap::Parser, std::fmt::Debug, std::clone::Clone)]
pub struct Aggregate {
    /// Exploded catalog path
    #[clap(short = 'e', long = "exploded-path")]
    exploded_path: std::path::PathBuf,

    /// Unified catalog path
    #[clap(short = 'u', long = "unified-path")]
    unified_path: std::path::PathBuf,

    /// Tables use to create unified table
    #[clap(short = 't', long = "tables")]
    tables: Vec<catalog::Table>,

    /// Name of columns to drop
    #[clap(short = 'd', long = "drop-columns")]
    drop_columns: Vec<String>,

    /// Method of aggregation
    #[clap(short = 'm', long = "method")]
    method: Method,

    /// Partition use
    #[clap(short = 'p', long = "partitions")]
    partitions: Vec<catalog::PartitionGroup>,
}

impl Aggregate {
    pub fn exploded_path(&self) -> &std::path::PathBuf {
        &self.exploded_path
    }

    pub fn unified_path(&self) -> &std::path::PathBuf {
        &self.unified_path
    }

    pub fn tables(&self) -> &[catalog::Table] {
        &self.tables
    }

    pub fn drop_columns(&self) -> &[String] {
        &self.drop_columns
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn partitions(&self) -> &[catalog::PartitionGroup] {
        &self.partitions
    }
}

#[derive(clap::Parser, std::fmt::Debug, std::clone::Clone)]
pub struct Csv2unified {
    /// Unified catalog path
    #[clap(short = 'u', long = "unified-path")]
    unified_path: std::path::PathBuf,

    /// Input data path
    #[clap(short = 'i', long = "input-path")]
    input_path: std::path::PathBuf,

    /// Tables use to create unified table
    #[clap(short = 't', long = "tables")]
    tables: Vec<catalog::Table>,

    /// Partition use
    #[clap(short = 'p', long = "partitions")]
    partitions: Vec<catalog::PartitionGroup>,
}

impl Csv2unified {
    pub fn unified_path(&self) -> &std::path::PathBuf {
        &self.unified_path
    }

    pub fn input_path(&self) -> &std::path::PathBuf {
        &self.input_path
    }

    pub fn tables(&self) -> &[catalog::Table] {
        &self.tables
    }

    pub fn partitions(&self) -> &[catalog::PartitionGroup] {
        &self.partitions
    }
}

#[cfg(feature = "rest_server")]
#[derive(clap::Parser, std::fmt::Debug, std::clone::Clone)]
pub struct Beacon {
    /// Unified catalog path
    #[clap(short = 'u', long = "unified-path")]
    unified_path: std::path::PathBuf,

    /// Set port of beacon server
    #[clap(short = 'p', long = "port")]
    port: Option<u16>,

    /// Set ip adress
    #[clap(short = 'a', long = "address")]
    address: Option<core::net::IpAddr>,
}

#[cfg(feature = "rest_server")]
impl Beacon {
    pub fn unified_path(&self) -> &std::path::PathBuf {
        &self.unified_path
    }

    pub fn port(&self) -> u16 {
        self.port.unwrap_or(8080)
    }

    pub fn address(&self) -> error::Result<core::net::IpAddr> {
        Ok(self.address.unwrap_or("127.0.0.1".parse()?))
    }
}

#[derive(clap::ValueEnum, std::fmt::Debug, std::clone::Clone)]
pub enum Method {
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
