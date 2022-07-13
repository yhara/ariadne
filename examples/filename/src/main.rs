use ariadne::{Report, ReportKind, Label};
use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("sample.tao").to_path_buf();
    let src = fs::read_to_string(path).unwrap();
    Report::build(ReportKind::Error, path, 34)
        .with_message("Incompatible types")
        .with_label(Label::new((path, 32..33)).with_message("This is of type Nat"))
        .with_label(Label::new((path, 42..45)).with_message("This is of type Str"))
        .finish()
        .print((path, src))
        .unwrap();
}
