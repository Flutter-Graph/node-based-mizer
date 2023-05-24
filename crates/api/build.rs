fn main() {
    println!("cargo:rerun-if-changed=protos");
    protobuf_codegen::Codegen::new()
        .pure()
        .out_dir("src/models")
        .inputs([
            "protos/connections.proto",
            "protos/fixtures.proto",
            "protos/layouts.proto",
            "protos/media.proto",
            "protos/nodes.proto",
            "protos/session.proto",
            "protos/sequencer.proto",
            "protos/effects.proto",
            "protos/transport.proto",
            "protos/programmer.proto",
            "protos/settings.proto",
            "protos/plans.proto",
            "protos/mappings.proto",
            "protos/timecode.proto",
        ])
        .include("protos")
        .run_from_script();
}