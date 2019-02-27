#![feature(box_patterns)]
#![feature(box_syntax)]
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::env;
use std::fs::File;
use std::io;
use std::io::Write;
use std::process::Command;

use crate::ast::statement::RootNode;
use crate::c_compile::c_writer::CWriter;
use clap::App;
use clap::Arg;
use clap::SubCommand;
use std::string::FromUtf8Error;
use crate::c_compile::c_write_utils::DATABOX;
use std::fs;

pub mod ast;
pub mod file_util;
pub mod c_compile;

const DATABOX_H_PATH: &'static str = "databox.h";
const DATABOX_C_PATH: &'static str = "databox.c";
const PRINT_H_PATH: &'static str = "print.h";
const PRINT_C_PATH: &'static str = "print.c";

/// this is the genrated rust code that contains c_datalib as rust const
include!(concat!(env!("OUT_DIR"), "/c_lib.rs"));

fn main() {
    let matches = App::new("Simple rust javascript to C compiler")
        .version("1.0")
        .author("Paul D. <paul.delafosse.etu@univ-lille.fr>")
        .about("A simple Javascipt to C compiler")
        .arg(Arg::with_name("SOURCE")
            .help("Name of the target javascript file")
            .short("s")
            .long("source")
            .required(true)
            .index(1))
        .arg(Arg::with_name("out")
            .help("Sets the name of the output binary, default \"out\"")
            .required(false)
            .index(2))
        .arg(Arg::with_name("indent")
            .short("i")
            .long("indent")
            .help("Indent the generated C source with GNU indent"))
        .arg(Arg::with_name("ast")
            .help("Leave the estree json file in place after compilation")
            .short("a")
            .long("ast")
            .required(false))
        .arg(Arg::with_name("verbose")
            .help("shows gcc warning")
            .required(false))
        .get_matches();

    let source = matches.value_of("SOURCE");
    let json_estree = generate_estree(source.unwrap())
        .expect("UTF-8 Error while reading json estree generated with babylon.");
    let root_node: RootNode = file_util::deserialize_json(json_estree.as_str());
    let program_root = root_node.get_program_root();
    let program_root = program_root.expect("Error parsing Json AST");

    let mut writer = CWriter {
        out: &mut "".to_string(),
    };

    // create the c_library files in the current directory
    let c_lib_file_error = "Error writing the standard library";
    let mut f_databox_h = File::create(DATABOX_H_PATH);
    let mut f_databox_c = File::create(DATABOX_C_PATH);
    let mut f_print_h = File::create(PRINT_H_PATH);
    let mut f_print_c = File::create(PRINT_C_PATH);

    f_databox_h.unwrap().write_all(DATABOX_H.as_bytes()).expect(c_lib_file_error);
    f_databox_c.unwrap().write_all(DATABOX_C.as_bytes()).expect(c_lib_file_error);
    f_print_h.unwrap().write_all(PRINT_H.as_bytes()).expect(c_lib_file_error);
    f_print_c.unwrap().write_all(PRINT_C.as_bytes()).expect(c_lib_file_error);

    // build c source from estre
    writer.visit_program_root(program_root);

    let verbose = matches.is_present("verbose");
    if let Some(name) = matches.value_of("out") {
        write_to_file(Some(name), writer)
            .expect(format!("Error writing {}", name).as_str());

        compile(Some(name), verbose);
    } else {
        write_to_file(None, writer)
            .expect("Error writing out.c");

        compile(None, verbose);
    }

    clean_filesystem();
}

/// Write the generated source to file with an optional filename
fn write_to_file(filename: Option<&str>, pretty: CWriter) -> Result<(), io::Error> {
    let mut file;
    if let Some(name) = filename {
        file = File::create(name)?;
    } else {
        file = File::create("out.c")?;
    }
    file.write_all(pretty.out.as_bytes())?;
    Ok(())
}


/// Get a string from babylon stdout
fn generate_estree(js_source: &str) -> Result<String, FromUtf8Error> {
    let mut babylon_cmd = Command::new("babylon");
    babylon_cmd.arg(js_source);
    let estree = babylon_cmd.output()
                            .expect("Failed to generate estree.");
    let out = String::from_utf8(estree.stdout);

    out
}

/// Compile generated source with gcc, at last !
fn compile(filename: Option<&str>, verbose: bool) {
    let mut gcc_cmd = Command::new("gcc");
    let name = if let Some(name) = filename {
        name
    } else {
        "out.o"
    };

    gcc_cmd.arg("out.c");
    gcc_cmd.arg("-g");
    if verbose {
        gcc_cmd.arg("-Wall");
    }
    gcc_cmd.arg("-o");
    gcc_cmd.arg(name);

    gcc_cmd.status().expect("Failed to compile source");
}

fn clean_filesystem() {
    fs::remove_file(DATABOX_H_PATH);
    fs::remove_file(DATABOX_C_PATH);
    fs::remove_file(PRINT_H_PATH);
    fs::remove_file(PRINT_C_PATH);
}
