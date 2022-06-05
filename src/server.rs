use tonic::{transport::Server, Request, Response, Status};

//msname
use microservices::msname_server::{Msname, MsnameServer};
use microservices::{NameRequest, NameResponse};

//mssurname
use microservices::mssurname_server::{Mssurname, MssurnameServer};
use microservices::{SurnameRequest, SurnameResponse};

pub mod microservices {
    tonic::include_proto!("microservices");
}

#[derive(Debug, Default)]
pub struct MsnameService {}

#[tonic::async_trait]
impl Msname for MsnameService {
    async fn send_name(
        &self,
        request: Request<NameRequest>,
    ) -> Result<Response<NameResponse>, Status> {
        println!("Microservice name have asked for a request: {:#?}", request);
        println!("Server waiting for a request ...");

        let req = request.into_inner();

        //reply to msname
        let reply = NameResponse {
            successful: true,
            message: format!("We've sent the name: {}", req.name).into(),
        };
        Ok(Response::new(reply))
    }
}

#[derive(Debug, Default)]
pub struct MssurnameService {}

#[tonic::async_trait]
impl Mssurname for MssurnameService {
    async fn send_surname(
        &self,
        request: Request<SurnameRequest>,
    ) -> Result<Response<SurnameResponse>, Status> {
        println!("Microservice name have asked for a request: {:#?}", request);
        println!("Server waiting for a request ...");

        let req = request.into_inner();

        //reply to mssurname
        let reply = SurnameResponse {
            successful: true,
            message: format!("We've sent the name: {} {}", req.name, req.surname).into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    //create a service
    let msn_service = MsnameService::default();

    //create a service
    let mssurname_service = MssurnameService::default();

    //adding services to Server
    Server::builder()
        .add_service(MsnameServer::new(msn_service))
        .add_service(MssurnameServer::new(mssurname_service))
        .serve(addr)
        .await?;

    Ok(())
}
