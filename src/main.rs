use std::fs;
use tree_sitter::{Parser, WasmStore};
use wasmtime::Engine;

fn main() {
    // Read generated tree sitter parser
    let wasm = fs::read("./tree-sitter-nemo.wasm").expect("failed to open wasm file");

    // Set up Wasmtime engine and initialize tree-sitter parser
    let engine = Engine::default();
    let mut store = WasmStore::new(&engine).expect("failed to make WasmStore");
    let language = store
        .load_language("nemo", &wasm)
        .expect("failed to load language");
    let mut parser = Parser::new();
    parser
        .set_wasm_store(store)
        .expect("failed to set wasm store");
    parser
        .set_language(&language)
        .expect("failed to set language");

    // Parse a source file
    let source = fs::read_to_string("../nemo-lang/play.nemo").expect("failed to read source file");
    let tree = parser.parse(&source, None).expect("failed to parse");

    // If we intend on parsing multiple documents, we need to make sure to reset the parser here
    // parser.reset();
    println!("{}", tree.root_node().to_sexp());
}
