/// Script to Generate the AST definitions for the `rinlox` Lox parser
///
/// This script generates a file called `expr.rs` in the directory passed
/// as the argument
///
/// In there, it will generate an Enum with a variant per type of Expression
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;

// TODO(alvaro): Move this whole script to a macro that takes the rules
// and generates the structs with the right methods

fn main() -> std::io::Result<()> {
    let args = std::env::args();
    if args.len() != 2 {
        println!("usage: generate-ast <output-dir>");
        std::process::exit(1);
    }
    let output_dir = args.into_iter().nth(1).unwrap();
    define_ast(
        &output_dir,
        "Expr",
        &[
            "Binary: Expr left, Token operator, Expr right",
            "Grouping: Expr expression",
            "Literal: Object value",
            "Unary: Token operator, Expr right",
        ],
    )?;

    Ok(())
}

fn define_ast(output_dir: &str, base_name: &str, types: &[&str]) -> std::io::Result<()> {
    let output_path = Path::new(output_dir).join(format!("{}.rs", &base_name.to_ascii_lowercase()));
    let f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(output_path)?;
    let mut buffer = BufWriter::new(f);

    // Write the preamble
    writeln!(
        buffer,
        "/// AST Definition types (autogenerated by `generate-ast`)"
    )?;
    writeln!(buffer, "use crate::lexer::Token;")?;
    writeln!(buffer)?;
    writeln!(buffer, "#[derive(Debug, Copy, Clone)]")?;
    writeln!(buffer, "pub struct Object {{}}")?;
    writeln!(buffer)?;
    writeln!(buffer, "#[derive(Debug, Clone)]")?;
    writeln!(buffer, "pub enum {}<'a> {{", base_name)?;

    for typ in types {
        let mut type_parts = typ.split(':');
        let class_name = type_parts.next().expect("type should have a name").trim();
        let fields = type_parts.next().expect("type should have fields").trim();
        define_type(&mut buffer, base_name, class_name, fields)?;
    }

    writeln!(buffer, "}}")?;

    Ok(())
}

fn define_type(
    buffer: &mut BufWriter<File>,
    base_name: &str,
    class_name: &str,
    fields: &str,
) -> std::io::Result<()> {
    writeln!(buffer, "    {} {{", class_name)?;

    for field in fields.split(',') {
        let mut field_parts = field.trim().split_whitespace();
        let field_type = field_parts.next().expect("field should have a type").trim();
        let field_name = field_parts.next().expect("field should have a name").trim();

        if field_type == base_name {
            // This is a recursive definition, so we need to add a 'a reference
            // indirection
            writeln!(buffer, "        {}: &'a {}<'a>,", field_name, field_type)?;
        } else {
            writeln!(buffer, "        {}: {},", field_name, field_type)?;
        }
    }

    writeln!(buffer, "    }},")?;
    Ok(())
}
