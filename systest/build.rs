extern crate ctest;

use std::env;

fn main() {
    let mut cfg = ctest::TestGenerator::new();
    if let Ok(out) = env::var("DEP_LZMA_INCLUDE") {
        cfg.include(&out);
    }
    
    cfg.header("lzma.h");
    cfg.type_name(|n, _s| n.to_string());
    cfg.define("LZMA_API_STATIC", None);
    cfg.skip_type(|n| n == "__enum_ty");
    cfg.generate("../lzma-sys/src/lib.rs", "all.rs");
}
