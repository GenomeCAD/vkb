//! Functional test check regression in help message

/* std use */

/* crate use */

/* project use */

#[test]
#[ignore]
fn functional() -> vkb::error::Result<()> {
    let mut cmd = assert_cmd::Command::cargo_bin("vkb")?;
    cmd.args(["-i", "ACTGactg"]);

    let assert = cmd.assert();

    assert
        .success()
        .stderr(b"" as &[u8])
        .stdout(b"cagtCAGT\n" as &[u8]);

    Ok(())
}
