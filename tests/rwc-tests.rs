use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

const INPUT_PATH: &str = "tests/inputs";
const EXPECTED_PATH: &str = "tests/expected";
const EMPTY: &str = "empty.txt";
const ONE_LINE_AND_NEW_EMPTY_LINE: &str = "one_line_and_new_empty_line.txt";
const ONE_LINE_AND_NO_NEW_LINE_BYTE: &str = "one_line_and_no_new_line_byte.txt";
const TWO_LINES: &str = "two_lines.txt";
const TWO_EMPTY_LINES: &str = "two_empty_lines.txt";
const THREE_LINES: &str = "three_lines.txt";
const TWO_PLUS_THREE_LINES: &str = "two_plus_three_lines.txt";

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn test(file_name: &str) -> TestResult {
    let expected = fs::read_to_string(format!("{}/{}", EXPECTED_PATH, file_name))?;
    let mut cmd = Command::cargo_bin("rwc")?;
    cmd.arg(format!("{}/{}", INPUT_PATH, file_name))
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn dies_no_arg() -> TestResult {
    let mut cmd = Command::cargo_bin("rwc")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

#[test]
fn no_file() -> TestResult {
    let mut cmd = Command::cargo_bin("rwc")?;
    cmd.arg("tests/inputs/cant-touch-this-2.txt")
        .assert()
        .success()
        .stderr(predicate::str::contains("No such file or directory"));
    Ok(())
}

#[test]
fn runs() -> TestResult {
    let mut cmd = Command::cargo_bin("rwc")?;
    cmd.arg("tests/inputs/empty.txt").assert().success();
    Ok(())
}

#[test]
fn empty_file() -> TestResult {
    test(EMPTY)
}

#[test]
fn one_line_and_new_empty_line() -> TestResult {
    test(ONE_LINE_AND_NEW_EMPTY_LINE)
}

#[test]
fn one_line_and_no_new_line_byte() -> TestResult {
    test(ONE_LINE_AND_NO_NEW_LINE_BYTE)
}

#[test]
fn two_lines() -> TestResult {
    test(TWO_LINES)
}

#[test]
fn two_empty_lines() -> TestResult {
    test(TWO_EMPTY_LINES)
}

#[test]
fn three_lines() -> TestResult {
    test(THREE_LINES)
}

#[test]
fn two_plus_three_lines() -> TestResult {
    let expected = fs::read_to_string(format!("{}/{}", EXPECTED_PATH, TWO_PLUS_THREE_LINES))?;
    let mut cmd = Command::cargo_bin("rwc")?;
    cmd.args(&[
        format!("{}/{}", INPUT_PATH, TWO_LINES),
        format!("{}/{}", INPUT_PATH, THREE_LINES),
    ])
    .assert()
    .success()
    .stdout(expected);
    Ok(())
}

fn test_with_flag(input_file: &str, expected_file: &str, flag: &str) -> TestResult {
    let expected = fs::read_to_string(format!("{}/{}", EXPECTED_PATH, expected_file))?;
    let mut cmd = Command::cargo_bin("rwc")?;
    cmd.args(&[format!("{}/{}", INPUT_PATH, input_file), String::from(flag)])
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn flag_m() -> TestResult {
    test_with_flag(TWO_LINES, "two_lines_only_chars.txt", "-m")
}

#[test]
fn flag_l() -> TestResult {
    test_with_flag(THREE_LINES, "three_lines_l_flag.txt", "-l")
}

#[test]
fn flag_w() -> TestResult {
    test_with_flag(THREE_LINES, "three_lines_w_flag.txt", "-w")
}

#[test]
fn flag_lmw() -> TestResult {
    test_with_flag(THREE_LINES, "three_lines_lmw_flag.txt", "-lmw")
}

#[test]
fn flag_wlm() -> TestResult {
    test_with_flag(THREE_LINES, "three_lines_lmw_flag.txt", "-wlm")
}
