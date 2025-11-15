fn main() {
    cynic_codegen::register_schema("wcalive")
        .from_sdl_file("wcalive.graphql")
        .unwrap()
        .as_default()
        .unwrap();
}
