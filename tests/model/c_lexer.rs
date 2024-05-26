use std::{fs, path::PathBuf};

use obfus_eval::model::c_lexer::{CLexer, Lexer};

#[test]
fn c_source_lexical_analysis_test() {
    let local_path: PathBuf = PathBuf::from("tests/resource/lexer_analysis_test_complex.c");
    let path: PathBuf = fs::canonicalize(&local_path).unwrap();

    // let tokens: Vec<Token> = CLexer::analyze(&path);
    let tokens = CLexer::analyze(&path);
    dbg!(tokens);
}
