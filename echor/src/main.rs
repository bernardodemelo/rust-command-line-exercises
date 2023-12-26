/* fn main() {
    println!("{:?}", std::env::args());     
}
*/

use clap::{ App, Arg } /* 1 */;
fn main() {
    let matches = App::new("echor") /* 2 */
        .version("0.1.0") /* 3 */
        .author("Bernardo de Melo <bernardo@mail.com>") /* 4 */
        .about("Rust echo") /* 5 */
        .arg(
            Arg::with_name("text") /* 6 */
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1)
        )
        .arg(
            Arg::with_name("omit_newline") /* 7 */
                .short("n")
                .help("Do not print newline")
                .takes_value(false)
        )
        .get_matches(); /* 8 */
    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");
    print!("{}{}", text.join(" "), if omit_newline {""} else {"\n"});
}

/* 1 - Import both clap::App and Arg structs. A struct is a a custom data type that groups together related pieces of data under a single name. Similar to OOP Classes, but they lack Inheritance and Polymorphism. clap is a crate (Rust library)  */

/* 2 - Create a new App with the name 'echor'. */

/* 3 - Use semantic version information. */

/* 4 - Include your name and email address. */

/* 5 - Short version about the program. */

/* 6 - Create a new Arg with the name text. This is a required positional argument that must appear at least once and can be repeated.
 */

/* 7 - Create a new Arg with the name omit_newline. This is a flag that has only the short name -n and takes no value. */

/* 8 - Tell the App to parse the arguments. */

/* 9 - Pretty-printing the artguments, using {:#?} to include newlines and indentations to help me read the output. */
