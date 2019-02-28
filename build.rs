use std::fs;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::Write;
use std::io;

/// This just write the c_datalib into static str constant before compilation
/// so that the c_datalib is available at run time
fn main() {
    let databox_c = fs::read_to_string("c_datalib/databox.c")
        .expect("Failed to read file with path : c_datalib/databox.c")
        .clone()
        .replace("\"", "\\\"");

    let databox_h = fs::read_to_string("c_datalib/databox.h")
        .expect("Failed to read file with path : c_datalib/databox.c")
        .clone()
        .replace("\\", "\\\\")
        .replace("\"", "\\\"");

    let print_c = fs::read_to_string("c_datalib/print.c")
        .expect("Failed to read file with path : c_datalib/print.c")
        .clone()
        .replace("\"", "\\\"");

    let print_h = fs::read_to_string("c_datalib/print.h")
        .expect("Failed to read file with path : c_datalib/print.h")
        .clone()
        .replace("\\", "\\\\")

        .replace("\"", "\\\"");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("c_lib.rs");
    let mut f = File::create(&dest_path).unwrap();

    create_temp_lib(databox_c, databox_h, print_c, print_h, &mut f)
        .expect("Error while writing c_libs");
}

fn create_temp_lib(databox_c: String, databox_h: String, print_c: String, print_h: String, f: &mut File)
 -> Result<(), io::Error> {
    let databox_c_const = b"pub const DATABOX_C: &'static str = ";
    let databox_h_const = b"pub const DATABOX_H: &'static str = ";
    let print_c_const = b"pub const PRINT_C: &'static str = ";
    let print_h_const = b"pub const PRINT_H: &'static str = ";
    f.write_all(databox_c_const)?;
    f.write_all(b"\"")?;
    f.write_all(databox_c.as_bytes())?;
    f.write_all(b"\";")?;
    f.write_all(databox_h_const)?;
    f.write_all(b"\"")?;
    f.write_all(databox_h.as_bytes())?;
    f.write_all(b"\";")?;
    f.write_all(print_c_const)?;
    f.write_all(b"\"")?;
    f.write_all(print_c.as_bytes())?;
    f.write_all(b"\";")?;
    f.write_all(print_h_const)?;
    f.write_all(b"\"")?;
    f.write_all(print_h.as_bytes())?;
    f.write_all(b"\";")?;
    Ok(())
}