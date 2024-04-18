//server.rs
use crate::greeter::greeter_server::{Greeter, GreeterServer};
use crate::greeter::{
    self, FactorialRequest, FactorialResponse, GreetRequest, GreetResponse, IsOddRequest,
    IsOddResponse, MultiplyRequest, MultiplyResponse,
};
use tonic::{transport::Server, Request, Response, Status};

fn calc_factorial(num: u32) -> u64 {
    if num == 0 {
        return 1u64;
    }
    (num as u64) * calc_factorial(num - 1)
}

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn greet(
        &self,
        request: Request<GreetRequest>,
    ) -> Result<Response<GreetResponse>, Status> {
        let reply = greeter::GreetResponse {
            greeting: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }

    async fn multiply(
        &self,
        request: Request<MultiplyRequest>,
    ) -> Result<Response<MultiplyResponse>, Status> {
        let MultiplyRequest { a, b } = request.into_inner();
        let response = MultiplyResponse { result: a * b };
        Ok(Response::new(response))
    }

    async fn is_odd(
        &self,
        request: Request<IsOddRequest>,
    ) -> Result<Response<IsOddResponse>, Status> {
        let number = request.into_inner().number;
        let response = IsOddResponse {
            is_odd: number % 2 != 0,
        };
        Ok(Response::new(response))
    }

    async fn factorial(
        &self,
        request: Request<FactorialRequest>,
    ) -> Result<Response<FactorialResponse>, Status> {
        let num = request.into_inner().a;
        let res = calc_factorial(num);
        Ok(Response::new(FactorialResponse { b: res }))
    }
}

pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}