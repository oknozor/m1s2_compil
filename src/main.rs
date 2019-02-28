#![feature(box_patterns)]
#![feature(box_syntax)]
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::process::Command;
use std::string::FromUtf8Error;

use clap::App;
use clap::Arg;

use crate::ast::statement::RootNode;
use crate::c_compile::c_writer::CWriter;

pub mod ast;
pub mod file_util;
pub mod c_compile;
pub mod visitor;
pub mod interpret;


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
            .help("Sets the name of the output binary")
            .required(false)
            .value_name("OUT")
            .short("o")
            .short("out"))
        .arg(Arg::with_name("indent")
            .short("i")
            .long("indent")
            .help("Indent the generated C source with GNU indent"))
        .arg(Arg::with_name("keep-ast")
            .help("let the estree json file in place after compilation")
            .long("keep-ast")
            .short("a")
            .long("keep-ast")
            .required(false))
        .arg(Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .help("shows gcc warning")
            .required(false))
        .arg(Arg::with_name("debug")
            .long("debug")
            .short("d")
            .help("compile with gcc debug flag")
            .required(false))
        .arg(Arg::with_name("keep-c")
            .long("keep-c")
            .short("c")
            .help("let the C generated source in place after compilation")
            .required(false))
        .get_matches();

    // Collect command line args
    let source = matches.value_of("SOURCE");
    let verbose = matches.is_present("verbose");
    let indent = matches.is_present("indent");
    let keep_c = matches.is_present("keep-c");
    let keep_ast = matches.is_present("keep-ast");
    let filename = matches.value_of("out").unwrap_or("out");
    let debug = matches.is_present("debug");


    let json_estree = generate_estree(source.unwrap())
        .expect("UTF-8 Error while reading json estree generated with babylon.");

    // Create a babylon json file
    if keep_ast {
        let estree_filename = format!("{}.json", filename);
        let estree_file = File::create(estree_filename);
        estree_file.unwrap().write_all(json_estree.clone().as_bytes())
            .expect("Error writing AST to file");
    }


    let root_node: RootNode = file_util::deserialize_json(json_estree.as_str());
    let program_root = root_node.get_program_root();
    let program_root = program_root.expect("Error parsing Json AST");

    let mut writer = CWriter {
        out: &mut "".to_string(),
    };

    copy_lib();

    // build c source from estree
    writer.visit_program_root(program_root);
    write_to_file(filename, writer).expect(format!("Error writing {}", filename).as_str());
    compile(filename, verbose, debug);

    if indent {
        let mut gnu_indent = Command::new("indent");
        gnu_indent.arg(format!("{}.c", filename));
        gnu_indent.status().expect("Error while indenting file, is GNU indent installed?");
    }


    clean_filesystem(keep_c, filename)
        .expect("Something went wrong while removing generated sources");
}


// create the c_library files in the current directory
fn copy_lib() {
    let c_lib_file_error = "Error writing the standard library";
    let f_databox_h = File::create(DATABOX_H_PATH);
    let f_databox_c = File::create(DATABOX_C_PATH);
    let f_print_h = File::create(PRINT_H_PATH);
    let f_print_c = File::create(PRINT_C_PATH);

    f_databox_h.unwrap().write_all(DATABOX_H.as_bytes()).expect(c_lib_file_error);
    f_databox_c.unwrap().write_all(DATABOX_C.as_bytes()).expect(c_lib_file_error);
    f_print_h.unwrap().write_all(PRINT_H.as_bytes()).expect(c_lib_file_error);
    f_print_c.unwrap().write_all(PRINT_C.as_bytes()).expect(c_lib_file_error);
}

/// Write the generated source to file with an optional filename
fn write_to_file(filename: &str, pretty: CWriter) -> Result<(), io::Error> {
    let filename_c = format!("{}.c", filename);
    let mut file = File::create(filename_c)?;
    file.write_all(pretty.out.as_bytes())?;
    Ok(())
}


/// Get a string from babylon stdout
fn generate_estree(js_source: &str) -> Result<String, FromUtf8Error> {
    let mut babylon_cmd = Command::new("babylon");
    babylon_cmd.arg(js_source);
    let estree = babylon_cmd.output().expect("Unable to parse JS source using babylon");
    let out = String::from_utf8(estree.stdout);
    out
}

/// Compile generated source with gcc, at last !
fn compile(filename: &str, verbose: bool, debug: bool) {
    let mut gcc_cmd = Command::new("gcc");
    let filename_c = format!("{}.c", filename);
    let filename_bin = filename;
    gcc_cmd.arg(filename_c);
    if verbose { gcc_cmd.arg("-Wall"); };
    if debug { gcc_cmd.arg("-g"); };
    gcc_cmd.arg("-o");
    gcc_cmd.arg(filename_bin);
    gcc_cmd.status().expect("Failed to compile source");
}


// Just remove the mess
fn clean_filesystem(keep: bool, filename: &str) -> Result<(), io::Error> {
    if !keep {
        fs::remove_file(DATABOX_H_PATH)?;
        fs::remove_file(DATABOX_C_PATH)?;
        fs::remove_file(PRINT_H_PATH)?;
        fs::remove_file(PRINT_C_PATH)?;
        let filename_c = format!("{}.c", filename);
        fs::remove_file(filename_c)?;
    }
    Ok(())
}
