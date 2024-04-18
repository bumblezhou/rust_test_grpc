//client.rs
use crate::greeter::greeter_client::GreeterClient;
use crate::greeter::{FactorialRequest, IsOddRequest, MultiplyRequest};

pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;
    let request = tonic::Request::new(MultiplyRequest { a: 3, b: 4 });
    let start = std::time::Instant::now();
    let response = client.multiply(request).await?;

    let mut request_1 = tonic::Request::new(IsOddRequest::default());
    request_1.get_mut().number = response.into_inner().result;
    let response_1 = client.is_odd(request_1).await?;
    println!(
        "RESPONSE={:?} turn around time is : {}",
        response_1.into_inner().is_odd,
        start.elapsed().as_millis()
    );
    let start_fact = std::time::Instant::now();
    let mut request_1 = tonic::Request::new(FactorialRequest::default());
    request_1.get_mut().a = 20;
    let response_1 = client.factorial(request_1).await?;
    println!(
        "RESPONSE={:?} turn around time is : {}",
        response_1.into_inner().b,
        start_fact.elapsed().as_millis()
    );

    Ok(())
}