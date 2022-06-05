use microservices::mssurname_client::MssurnameClient;
use microservices::SurnameRequest;

pub mod microservices {
    tonic::include_proto!("microservices");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut clientsurname = MssurnameClient::connect("http://[::1]:50051").await?;
    let request = tonic::Request::new(SurnameRequest {
        name: "james".to_owned(),
        surname: "ga".to_owned(),
    });

    //getting response
    let response = clientsurname.send_surname(request).await?;
    println!("Response => {:#?}", response);

    Ok(())
}
