use book::{book_api_server::BookApi, BookRequest, BookResponse};
use std::error::Error;
use tonic::transport::Server;

pub mod book {
    tonic::include_proto!("book");

    pub(crate) const FILE_DESCRIPTION_SET: &[u8] =
        tonic::include_file_descriptor_set!("book_descriptor");
}

mod google_book_api;

pub struct BookService {}

impl BookService {
    pub fn new() -> Self {
        Self {}
    }
}

#[tonic::async_trait]
impl BookApi for BookService {
    async fn retrive(
        &self,
        request: tonic::Request<BookRequest>,
    ) -> Result<tonic::Response<BookResponse>, tonic::Status> {
        let isbn = request.into_inner().isbn;
        Ok(tonic::Response::new(
            google_book_api::retrive_bookinfo(&isbn).await?,
        ))
    }

    async fn register(
        &self,
        request: tonic::Request<BookRequest>,
    ) -> Result<tonic::Response<BookResponse>, tonic::Status> {
        Ok(tonic::Response::new(BookResponse {
            isbn: request.into_inner().isbn,
            title: "Smaple Book".to_string(),
            authors: vec!["iphone".to_string()],
            thumbnail: "s".to_string(),
            publish_date: "a".to_string(),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
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
