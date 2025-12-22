use std::path::PathBuf;
use glob::glob;
use protobuf_codegen::Codegen;


fn build_protos_in_dir(dir: &str, namespace: &str) {
    let proto_sources: Vec<PathBuf> = glob(&format!("{}/*.proto", dir))
        .expect(&format!("Failed to glob protobuf source files in {}", dir))
        .filter_map(|e| e.ok())
        .collect();

    Codegen::new()
        .pure()
        .cargo_out_dir(&format!("pbgen_{}", namespace))
        .inputs(&proto_sources)
        .include(dir)
        .run_from_script();
}


fn main() {
    build_protos_in_dir("src/protos/refbox", "refbox");
    build_protos_in_dir("src/protos/vision", "vision");
}
