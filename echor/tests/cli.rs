use assert_cmd::Command;
use predicates::prelude::*; /* 1 */

#[test]
fn dies_no_args()/* 2 */{ 
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.assert() /* 3 */
        .failure()
        .stderr(predicate::str::contains("USAGE"));
}

/* 1 - Import the predicates crate. This library implements an interface to “predicates” - boolean-valued functions of one argument. The prelude module has all the essentials to work with predicates */

/* 2 - Comment from the author: "I often put the word dies somewhere in the test name to make it clear that the program is expected to fail under the given conditions."

If I run cargo test dies, then Cargo will run all the tests with names containing the string dies.
*/

/* 3 - Run the program with no arguments and assert that it fails and prints a usage statement to STDERR. */
