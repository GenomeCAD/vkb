//! Functional test check regression in help message

/* std use */

/* crate use */

/* project use */

const HELP: &[u8] = b"Usage: vkb [OPTIONS] --catalog-path <CATALOG_PATH> <COMMAND>

Commands:
  convert    Insert classic bioinformatic information in exploded database
  aggregate  Generate a unified table from exploded database
  help       Print this message or the help of the given subcommand(s)

Options:
  -c, --catalog-path <CATALOG_PATH>  Catalog path
  -t, --thread <THREADS>             Number of threads use if not set try to use maximum
  -q, --quiet                        Silence all output
  -v, --verbosity...                 Verbose mode (-v, -vv, -vvv, etc)
  -T, --timestamp <TS>               Timestamp (sec, ms, ns, none)
  -h, --help                         Print help
  -V, --version                      Print version
";

const HELP_CONVERT: &[u8] = b"Insert classic bioinformatic information in exploded database

Usage: vkb --catalog-path <CATALOG_PATH> convert [OPTIONS] --input-path <INPUT_PATH> --type <INPUT_TYPE>

Options:
  -i, --input-path <INPUT_PATH>  Input path
  -t, --type <INPUT_TYPE>        Input type [possible values: gvcf, vcf, tsv, phenopacket, json]
  -T, --tables <TABLES>          Tables where data are write [possible values: annotsv, clinvar, coverage, genotyping, gnomad, snpeff, symptom, variant, vep]
  -o, --overwrite                Overwrite catalog
  -h, --help                     Print help
";

const HELP_AGGREGATE: &[u8] = b"Generate a unified table from exploded database

Usage: vkb --catalog-path <CATALOG_PATH> aggregate [OPTIONS] --method <METHOD> --output-path <OUTPUT_PATH>

Options:
  -T, --tables <TABLES>              Tables use to create unified table [possible values: annotsv, clinvar, coverage, genotyping, gnomad, snpeff, symptom, variant, vep]
  -d, --drop-columns <DROP_COLUMNS>  Name of columns to drop
  -m, --method <METHOD>              Method of aggregation [possible values: genotype]
  -p, --partitions <PARTITIONS>      Partition use [possible values: annotation, annotation-genome, annotation-genome-sample, annotation-sample, annotation-sample-genome, genome, genome-annotation, genome-annotation-sample, genome-sample, genome-sample-annotation, sample, sample-annotation, sample-annotation-genome, sample-genome]
  -o, --output-path <OUTPUT_PATH>    Output path
  -h, --help                         Print help
";

#[test]
fn help() -> vkb::error::Result<()> {
    let mut cmd = assert_cmd::Command::cargo_bin("vkb")?;
    cmd.args(["-h"]);

    let assert = cmd.assert();

    assert.success().stderr(b"" as &[u8]).stdout(HELP);

    Ok(())
}

#[test]
fn help_convert() -> vkb::error::Result<()> {
    let mut cmd = assert_cmd::Command::cargo_bin("vkb")?;
    cmd.args(["convert", "-h"]);

    let assert = cmd.assert();

    assert.success().stderr(b"" as &[u8]).stdout(HELP_CONVERT);

    Ok(())
}

#[test]
fn help_aggregate() -> vkb::error::Result<()> {
    let mut cmd = assert_cmd::Command::cargo_bin("vkb")?;
    cmd.args(["aggregate", "-h"]);

    let assert = cmd.assert();

    assert.success().stderr(b"" as &[u8]).stdout(HELP_AGGREGATE);

    Ok(())
}
