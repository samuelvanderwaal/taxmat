use assert_cmd::Command;

#[test]
fn parse_subscan_to_bitcointax() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.args([
        "./tests/subscan.csv",
        "./tests/output.csv",
        "-i", 
        "subscan",
        "--coin", 
        "DOT",
        "-q",
        "Q1",
    ])
    .assert()
    .success();
}

#[test]
fn parse_kraken_to_bitcointax() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.args([
        "./tests/kraken.csv",
        "./tests/output.csv",
        "-i", 
        "kraken",
        "--coin", 
        "DOT",
        "-q",
        "Q2",
    ])
    .assert()
    .success();
}
