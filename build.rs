fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .compile(&["github.com/kubernetes/cri-api/pkg/apis/runtime/v1/api.proto"],
                 &["."])?;

    Ok(())
}