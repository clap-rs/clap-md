// Adopted from https://github.com/rust-clique/man/blob/fc11e3765a3a94e4ba6e1943eb747fd704f69a61/examples/main.rs

extern crate clap;
extern crate clap_md;

use clap::{Arg, Command};
use clap_md::app_to_md;

fn main() {
    let a = Command::new("testapp")
    .about("Pointless application")
    .arg_required_else_help(true)
    .subcommand_required(true)
    .author("Katharina Fey <kookie@spacekookie.de>")
    .long_about("Lorem Ipsum bla bla bla")
    .arg(Arg::new("debug").short('d').help("Make program output debug messages"))
    .arg(Arg::new("output").short('o').takes_value(true).help("Output File"))
    .subcommand(Command::new("foo").arg(Arg::new("bar").short('b').long("barr")));

    let markdown = app_to_md(a, 1).unwrap();
    println!("{}", markdown);
}