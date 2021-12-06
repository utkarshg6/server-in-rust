use crate::http::{ParseError, Request, Response, StatusCode};
use std::convert::TryFrom; // To use the try_from function from Request we'll need to pull the whole TryFrom trait.
use std::convert::TryInto;
use std::io::{Write, Read};
use std::net::TcpListener;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.addr);

        // If the result is OK, it'll unwrap the result into variable
        // otherwise it'll stop the program from executing
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {:?}", String::from_utf8_lossy(&buffer));

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    handler.handle_request(&request)
                                    // dbg!(request);
                                    // Response::new(
                                    //     StatusCode::Ok,
                                    //     Some("<h1>It works too!!!</h1>".to_string()),    
                                    // )
                                }
                                Err(e) => {
                                    handler.handle_bad_request(&e)
                                    // println!("Failed to parse a request: {}", e);
                                    // Response::new(
                                    //     StatusCode::BadRequest,
                                    //     None,
                                    // )
                                }
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                            // let res: &Result<Request, _> = &buffer[..].try_into(); 
                        }
                        Err(e) => {
                            println!("Failed to read from connection: {}", e)
                        }
                    };
                }
                Err(e) => {
                    println!("Failed to establish a connection: {}", e)
                }
            }
        }
    }
}
