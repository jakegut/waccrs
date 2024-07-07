use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

use clap::{value_parser, Arg, Command};
use waccrs::lexer;
use waccrs::errors::Result;

fn main() -> Result<()> {

    let matches = cli().get_matches();

    let mut buffer = Vec::new();
    let source_file_path: &PathBuf = matches.get_one("source_file").ok_or("source_file required")?;

    let mut source_file = File::open(source_file_path)?;
    source_file.read_to_end(&mut buffer)?;

    let mut lex = lexer::Lexer::new(buffer);

    lex.lex()?;

    println!("{:#?}", lex.tokens);

    if matches.get_flag("lex") {
        return Ok(())
    }

    Ok(())
}

fn cli() -> Command {
    Command::new("waccrs")
        .arg_required_else_help(true)
        .arg(
            Arg::new("lex")
                .long("lex")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("parse")
                .long("parse")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("codegen")
                .long("codegen")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("source_file")
                .action(clap::ArgAction::Set)
                .value_parser(value_parser!(PathBuf))
                .required(true)
        )
}
