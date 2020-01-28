pub mod hello_cri {
    tonic::include_proto!("cri");
}

use hello_cri::runtime_service_client::RuntimeServiceClient;
use hello_cri::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = RuntimeServiceClient::connect("http://127.0.0.1:50051").await?;
    let request = tonic::Request::new(VersionRequest {
        version: "foo".into(),
    });
    let response = client.version(request).await?;
    println!("Version: {:?}", response);

    Ok(())
}
