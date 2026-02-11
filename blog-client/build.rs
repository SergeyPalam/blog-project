fn main() {
    println!("cargo:rerun-if-changed=proto/blog.proto");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/**");
    if let Err(e) = tonic_prost_build::configure()
        .build_client(true)
        .build_server(false)
        .out_dir("proto")
        .compile_protos(
            &["proto/blog.proto"],
            &["proto"],
        )
    {
        println!("cargo:warning={e}");
    }
}