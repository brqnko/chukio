#![feature(rustc_private)]

use std::path::PathBuf;

use chukio::Interpreter;

#[test]
fn interprets_calc() -> anyhow::Result<()> {
    let source_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/ios/calc.rs");

    Interpreter::new()?.interpret(source_path)
}
