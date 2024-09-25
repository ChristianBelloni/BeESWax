use walkdir::WalkDir;

fn main() {
    let wd = WalkDir::new("src/main")
        .into_iter()
        .filter_map(|a| {
            let a = a.ok()?;
            if a.file_type().is_dir() {
                Some(a)
            } else {
                if a.path().extension().is_some_and(|a| a.eq("proto")) {
                    Some(a)
                } else {
                    None
                }
            }
        })
        .filter_map(|a| {
            if a.file_type().is_file() {
                Some(a.path().to_path_buf())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    prost_build::compile_protos(&[
        "./src/main/java/com/google/devtools/build/lib/buildeventstream/proto/build_event_stream.proto",
        "./src/main/java/com/google/devtools/build/lib/packages/metrics/package_load_metrics.proto",
        "./src/main/protobuf/action_cache.proto",
        "./src/main/protobuf/command_line.proto",
        "./src/main/protobuf/invocation_policy.proto",
        "./src/main/protobuf/failure_details.proto"
    ], &["."]).unwrap()
}
