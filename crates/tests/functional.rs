//! Functional test check regression in help message

/* std use */

/* crate use */

/* project use */

#[cfg(feature = "bin")]
#[test]
fn load_variant_csv_unified() -> vkb::error::Result<()> {
    let temp_dir = tempfile::tempdir()?;
    let temp_path = temp_dir.path();
    let exploded_path = temp_path.join("exploded");
    let unified_path = temp_path.join("unified");

    let mut cmd = assert_cmd::Command::cargo_bin("vkb")?;
    cmd.args([
        "-c",
        &format!("{}", exploded_path.display()),
        "csv2unified",
        "-t",
        "variant",
        "-p",
        "genome",
        "-i",
        "tests/data/variant.csv",
        "-o",
        &format!("{}", unified_path.display()),
    ]);

    let assert = cmd.assert();

    assert.success().stderr(b"" as &[u8]).stdout(b"" as &[u8]);

    Ok(())
}
