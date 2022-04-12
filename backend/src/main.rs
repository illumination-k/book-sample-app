use std::error::Error;
use book::{book_api_server::BookApi, RetriveBookResponse};
use tonic::transport::Server;

pub mod book {
    tonic::include_proto!("book");

    pub(crate) const FILE_DESCRIPTION_SET: &[u8] =
    tonic::include_file_descriptor_set!("book_descriptor");
}

pub struct BookService {}

impl BookService {
    pub fn new() -> Self {
        Self {}
    }
}

#[tonic::async_trait]
impl BookApi for BookService {
    async fn retrive(&self, request: tonic::Request<book::RetriveBookRequest>) -> Result<tonic::Response<book::RetriveBookResponse>, tonic::Status> {
        Ok(tonic::Response::new(RetriveBookResponse {
            id: request.into_inner().id,
            title: "Smaple Book".to_string(),
            author: "iphone".to_string(),
            publish: "a".to_string()
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let addr = "0.0.0.0:50051".parse()?;

    let reflection = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(book::FILE_DESCRIPTION_SET)
        .build()?;
    
    Server::builder()
        .add_service(reflection)
        .add_service(book::book_api_server::BookApiServer::new(BookService::new()))
        .serve(addr)
        .await?;

    Ok(())
}