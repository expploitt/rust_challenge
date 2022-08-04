
use assert_cmd::assert::OutputAssertExt;
use assert_cmd::Command;


#[test]
fn multiples_deposits() {
    let mut cmd = Command::cargo_bin("rust_project_08").unwrap();
    let assert = cmd
        .arg("files/input/multiples_deposits.csv");

    assert.assert().stdout(
        "client,available,held,total,locked\r\n1,10.0,0.0,10.0,false\r\n"
    );
}

#[test]
fn error_withdrawal() {
    let mut cmd = Command::cargo_bin("rust_project_08").unwrap();
    let assert = cmd
        .arg("files/input/error_withdrawal.csv");

    assert.assert().stdout(
        "client,available,held,total,locked\r\n1,5.0,0.0,5.0,false\r\n"
    );
}

#[test]
fn dispute() {
    let mut cmd = Command::cargo_bin("rust_project_08").unwrap();
    let assert = cmd
        .arg("files/input/dispute.csv");

    assert.assert().stdout(
        "client,available,held,total,locked\r\n1,0.0,0.5,0.5,false\r\n"
    );
}

#[test]
fn resolve() {
    let mut cmd = Command::cargo_bin("rust_project_08").unwrap();
    let assert = cmd
        .arg("files/input/resolve.csv");

    assert.assert().stdout(
        "client,available,held,total,locked\r\n1,0.5,0.0,0.5,false\r\n"
    );
}

#[test]
fn chargeback() {
    let mut cmd = Command::cargo_bin("rust_project_08").unwrap();
    let assert = cmd
        .arg("files/input/chargeback.csv");

    assert.assert().stdout(
        "client,available,held,total,locked\r\n1,0.5,-0.5,0.0,true\r\n"
    );
}

#[test]
fn example_test() {
    let mut cmd = Command::cargo_bin("rust_project_08").unwrap();
    let assert = cmd
        .arg("files/input/example.csv");

    assert.assert().stdout(
        "client,available,held,total,locked\r\n\
        1,1.5,0.0,1.5,false\r\n\
        2,2.0,0.0,2.0,false\r\n"
    );
}