//! Functional test check regression in help message

/* std use */

/* crate use */

/* project use */

#[cfg(feature = "bin")]
const HELP: &[u8] = b"Usage: vkb [OPTIONS] <COMMAND>

Commands:
  convert      Insert classic bioinformatic information in exploded database
  aggregate    Generate a unified table from exploded database
  csv2unified  Use a csv to populate unified table
  help         Print this message or the help of the given subcommand(s)

Options:
  -t, --thread <THREADS>  Number of threads use if not set try to use maximum
  -q, --quiet             Silence all output
  -v, --verbosity...      Verbose mode (-v, -vv, -vvv, etc)
  -T, --timestamp <TS>    Timestamp (sec, ms, ns, none)
  -h, --help              Print help
  -V, --version           Print version
";

#[cfg(feature = "bin")]
const HELP_CONVERT: &[u8] = b"Insert classic bioinformatic information in exploded database

Usage: vkb convert [OPTIONS] --exploded-path <EXPLODED_PATH> --input-path <INPUT_PATH> --type <INPUT_TYPE>

Options:
  -e, --exploded-path <EXPLODED_PATH>  Exploded catalog path
  -i, --input-path <INPUT_PATH>        Input path
  -t, --type <INPUT_TYPE>              Input type [possible values: gvcf, vcf, tsv, phenopacket, json]
  -T, --tables <TABLES>                Tables where data are write [possible values: annotsv, clinvar, coverage, genotyping, gnomad, snpeff, symptom, variant, vep]
  -o, --overwrite                      Overwrite catalog
  -h, --help                           Print help
";

#[cfg(feature = "bin")]
const HELP_AGGREGATE: &[u8] = b"Generate a unified table from exploded database

Usage: vkb aggregate [OPTIONS] --exploded-path <EXPLODED_PATH> --unified-path <UNIFIED_PATH> --method <METHOD>

Options:
  -e, --exploded-path <EXPLODED_PATH>  Exploded catalog path
  -u, --unified-path <UNIFIED_PATH>    Unified catalog path
  -t, --tables <TABLES>                Tables use to create unified table [possible values: annotsv, clinvar, coverage, genotyping, gnomad, snpeff, symptom, variant, vep]
  -d, --drop-columns <DROP_COLUMNS>    Name of columns to drop
  -m, --method <METHOD>                Method of aggregation [possible values: genotype]
  -p, --partitions <PARTITIONS>        Partition use [possible values: annotation, annotation-genome, annotation-genome-sample, annotation-sample, annotation-sample-genome, genome, genome-annotation, genome-annotation-sample, genome-sample, genome-sample-annotation, sample, sample-annotation, sample-annotation-genome, sample-genome]
  -h, --help                           Print help
";

#[cfg(feature = "bin")]
const HELP_CSV2UNIFIED: &[u8] = b"Use a csv to populate unified table

Usage: vkb csv2unified [OPTIONS] --unified-path <UNIFIED_PATH> --input-path <INPUT_PATH>

Options:
  -u, --unified-path <UNIFIED_PATH>  Unified catalog path
  -i, --input-path <INPUT_PATH>      Input data path
  -t, --tables <TABLES>              Tables use to create unified table [possible values: annotsv, clinvar, coverage, genotyping, gnomad, snpeff, symptom, variant, vep]
  -p, --partitions <PARTITIONS>      Partition use [possible values: annotation, annotation-genome, annotation-genome-sample, annotation-sample, annotation-sample-genome, genome, genome-annotation, genome-annotation-sample, genome-sample, genome-sample-annotation, sample, sample-annotation, sample-annotation-genome, sample-genome]
  -h, --help                         Print help
";

#[cfg(all(feature = "bin", feature = "rest_server"))]
const HELP_BEACON: &[u8] = b"Start a beacon REST server on unified

Usage: vkb --catalog-path <CATALOG_PATH> beacon [OPTIONS]

Options:
  -p, --port <PORT>        Set port of beacon server
  -a, --address <ADDRESS>  Set ip adress
  -h, --help               Print help
";

#[cfg(feature = "bin")]
#[test]
fn help() -> vkb::error::Result<()> {
    let mut cmd = assert_cmd::Command::cargo_bin("vkb")?;
    cmd.args(["-h"]);

    let assert = cmd.assert();

    assert.success().stderr(b"" as &[u8]).stdout(HELP);

    Ok(())
}

#[cfg(feature = "bin")]
#[test]
fn help_convert() -> vkb::error::Result<()> {
    let mut cmd = assert_cmd::Command::cargo_bin("vkb")?;
    cmd.args(["convert", "-h"]);

    let assert = cmd.assert();

    assert.success().stderr(b"" as &[u8]).stdout(HELP_CONVERT);

    Ok(())
}

#[cfg(feature = "bin")]
#[test]
fn help_aggregate() -> vkb::error::Result<()> {
    let mut cmd = assert_cmd::Command::cargo_bin("vkb")?;
    cmd.args(["aggregate", "-h"]);

    let assert = cmd.assert();

    assert.success().stderr(b"" as &[u8]).stdout(HELP_AGGREGATE);

    Ok(())
}

#[cfg(feature = "bin")]
#[test]
fn help_csv2unified() -> vkb::error::Result<()> {
    let mut cmd = assert_cmd::Command::cargo_bin("vkb")?;
    cmd.args(["csv2unified", "-h"]);

    let assert = cmd.assert();

    assert
        .success()
        .stderr(b"" as &[u8])
        .stdout(HELP_CSV2UNIFIED);

    Ok(())
}

#[cfg(all(feature = "bin", feature = "rest_server"))]
#[test]
fn help_beacon() -> vkb::error::Result<()> {
    let mut cmd = assert_cmd::Command::cargo_bin("vkb")?;
    cmd.args(["beacon", "-h"]);

    let assert = cmd.assert();

    assert.success().stderr(b"" as &[u8]).stdout(HELP_BEACON);

    Ok(())
}
