use assert_cmd::Command;
use predicates::prelude::*;
use rand::{distributions::Alphanumeric, Rng};
use std::error::Error;
use std::fs;
use opencv::prelude::*;

type TestResult = Result<(), Box<dyn Error>>;
const PRG: &str = "starrynight";
const FOUR_STARS: &str = "tests/inputs/4_stars.png";
const FIFTY_STARS: &str = "tests/inputs/50_stars.png";
//const EMPTY: &str = "tests/inputs/empty.txt";

// --------------------------------------------------
#[test]
fn usage() -> TestResult {
    for flag in &["-h", "--help"] {
        Command::cargo_bin(PRG)?
            .arg(flag)
            .assert()
            .stdout(predicate::str::contains("Usage"));
    }
    Ok(())
}

// --------------------------------------------------
fn gen_bad_file() -> String {
    loop {
        let filename: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}

// --------------------------------------------------
#[test]
fn skips_bad_file() -> TestResult {
    let bad = gen_bad_file();
    let expected = format!(".* [(]os error 2[)]");
    Command::cargo_bin(PRG)?
        .arg(&bad)
        .assert()
        .success()
        .stderr(predicate::str::is_match(expected)?);
    Ok(())
}

// --------------------------------------------------
fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn four_stars_c8() -> TestResult {
    run(&["-c", "8", FOUR_STARS],"tests/expected/4_stars.out.txt" )
}

#[test]
fn four_stars_c4() -> TestResult {
    run(&["-c", "4", FOUR_STARS],"tests/expected/4_stars.out.txt" )
}

#[test]
fn fifty_stars_c8_verbose() -> TestResult {
    run(&["-c", "8", FIFTY_STARS],"tests/expected/50_stars.c8.v.out.txt" )
}

#[test]
fn fifty_stars_c4_verbose() -> TestResult {
    run(&["-c", "4", FIFTY_STARS],"tests/expected/50_stars.c4.v.out.txt" )
}



#[test]
fn test_cvmat_eye_at() -> TestResult{
    let eye = Mat::eye(3,3, opencv::core::CV_64FC1)? ;
    let eye = eye.to_mat()?;
    assert_eq!(1.0, *eye.at::<f64>(0)?);
    assert_eq!(0.0, *eye.at::<f64>(1)?);
    assert_eq!(0.0, *eye.at::<f64>(2)?);

    assert_eq!(0.0, *eye.at::<f64>(3)?);
    assert_eq!(1.0, *eye.at::<f64>(4)?);    
    assert_eq!(0.0, *eye.at::<f64>(5)?);

    assert_eq!(0.0, *eye.at::<f64>(6)?);
    assert_eq!(0.0, *eye.at::<f64>(7)?);
    assert_eq!(1.0, *eye.at::<f64>(8)?);
    // that was awkward ^

    assert_eq!(1.0, *eye.at_2d::<f64>(0,0)?);
    assert_eq!(1.0, *eye.at_2d::<f64>(1,1)?);
    assert_eq!(1.0, *eye.at_2d::<f64>(2,2)?);
    // slightly more intuitive ? ^

    let _zeros = Mat::zeros(4, 4, opencv::core::CV_64FC1)?.to_mat();

    Ok(())  

}