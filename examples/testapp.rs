// Adopted from https://github.com/rust-clique/man/blob/fc11e3765a3a94e4ba6e1943eb747fd704f69a61/examples/main.rs

extern crate clap;
extern crate clap_md;

use clap::{App, AppSettings, Arg, SubCommand};
use clap_md::app_to_md;

fn main() {
    let a = App::new("testapp")
    .about("Pointless application")
    .setting(AppSettings::SubcommandRequiredElseHelp)
    .author("Katharina Fey <kookie@spacekookie.de>")
    // .author("Yosh Wuyts <y@w.s")
    .long_about("Lorem Ipsum bla bla bla")
    .arg(Arg::with_name("debug").short("d").help("Make program output debug messages"))
    .arg(Arg::with_name("output").short("o").takes_value(true).help("Output File"))
    .subcommand(SubCommand::with_name("foo").arg(Arg::with_name("bar").short("b").long("barr")));

    let markdown = app_to_md(&a, 1).unwrap();
    println!("{}", markdown);
}
