use std::process::Command;

#[test]
fn runs() {
    let mut cmd = Command::new("target/debug/hello");
    let res = cmd.output();
    assert!(res.is_ok());
}
