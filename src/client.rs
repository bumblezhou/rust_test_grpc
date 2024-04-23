//client.rs
use crate::greeter::greeter_client::GreeterClient;
use crate::greeter::{GreetRequest, FactorialRequest, IsOddRequest, MultiplyRequest};

pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_greet = std::time::Instant::now();
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;
    let greet_request = tonic::Request::new(GreetRequest { name: "test_name".to_string() });
    let greet_response = client.greet(greet_request).await?;
    let greeting = greet_response.into_inner().greeting;
    println!(
        "greet(test_name) received: {:?} turn around time is : {}",
        greeting,
        start_greet.elapsed().as_millis()
    );

    let start_multiply = std::time::Instant::now();
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;
    let request = tonic::Request::new(MultiplyRequest { a: 3, b: 4 });
    let response = client.multiply(request).await?;
    let result = response.into_inner().result;
    println!(
        "3 * 4 received: {:?} turn around time is : {}",
        result,
        start_multiply.elapsed().as_millis()
    );

    let start_is_odd = std::time::Instant::now();
    let mut request_1 = tonic::Request::new(IsOddRequest::default());
    request_1.get_mut().number = result;
    let response_1 = client.is_odd(request_1).await?;
    println!(
        "is_odd(3*4) receive: {:?} turn around time is : {}",
        response_1.into_inner().is_odd,
        start_is_odd.elapsed().as_millis()
    );

    let start_fact = std::time::Instant::now();
    let mut request_1 = tonic::Request::new(FactorialRequest::default());
    request_1.get_mut().a = 20;
    let response_1 = client.factorial(request_1).await?;
    println!(
        "factorial(20) received: {:?} turn around time is : {}",
        response_1.into_inner().b,
        start_fact.elapsed().as_millis()
    );

    Ok(())
}