fn main() {
    tauri_build::try_build(
        tauri_build::Attributes::new()
            .capabilities_path_pattern("./capabilities/app/*.json"),
    )
    .unwrap();
}
