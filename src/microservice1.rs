use microservices::msname_client::MsnameClient;
use microservices::NameRequest;

pub mod microservices {
    tonic::include_proto!("microservices");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut clientname = MsnameClient::connect("http://[::1]:50051").await?;
    let request = tonic::Request::new(NameRequest {
        name: "javier".to_owned(),
    });

    //getting response
    let response = clientname.send_name(request).await?;
    println!("Response => {:#?}", response);

    Ok(())
}
