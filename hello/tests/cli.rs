/* The #[test] attribute tells Rust to run this function when testing. */

/* #[test]
fn works() { assert!(true);
}
*/
/* The assert! macro asserts that a Boolean expression is true. */

/*
use std::process::Command; /* 1 */

fn runs(){
    let mut cmd = Command::new("ls"); /* 2 */
    let res = cmd.output(); /* 3 */
    assert!(res.is_ok()); /* 4 */
}
*/

/* 1 - I can access Command Line Tools via Rust */

/* 2 - Create a new Command to run ls. The let keyword will bind a value to a variable. The mut keyword will make this variable mutable so that it can change. By default, Rust Variables are Immutable */

/* 3 - Run the command and capture the output, which will be a Result.*/

/* 4 - Verify that the result is an Ok variant.*/

/*
use assert_cmd::Command; /* 1 */

#[test]
fn runs() {
let mut cmd = Command::cargo_bin("hello").unwrap(); /* 2 */
cmd.assert().success(); /* 3 */
} */

/* 1 - Import assert_cmd::Command */

/* 2 - Create a Command to run hello in the current crate. This returns a Result, and the code calls Result::unwrap because the binary should be found. If it isn’t, then unwrap will cause a panic and the test will fail, which is a good thing. */

/* 3 - Use Assert::success to ensure the command succeeded.*/

/* 
use assert_cmd::Command;

#[test]
fn true_ok() {
let mut cmd = Command::cargo_bin("true").unwrap(); 
cmd.assert().success();
}
*/

/* Check if the Hello World Program is Working */

use assert_cmd::Command;

#[test]
fn runs() {
let mut cmd = Command::cargo_bin("hello").unwrap(); 
cmd.assert().success().stdout("Hello, world!\n"); /* 1 */
}

/* 1 - Verify that the command exits successfully and prints the expected text to STDOUT. */