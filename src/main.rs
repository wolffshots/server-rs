use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

// default route
fn handle_index(request_type: &str) -> String{
    match request_type {
        "GET" => {
            let contents = fs::read_to_string("static/index.html").unwrap();
            format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                contents.len(),
                contents
            )
        },
        // handle unimplemented routes
        _ => handle_error(405, "method not allowed on route")
    }
}

// handle generic error and message
fn handle_error(code: u16, message: &str) -> String{
    format!("HTTP/1.1 {0} OK\r\n\r\n{0} - {1}",code, message)
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let request = &String::from_utf8_lossy(&buffer[..]);
    println!("Request:\n{}", request);
    let request_type_and_location: Vec<_> = request.split("\r\n").collect();
    let request_type_and_location: Vec<_> = request_type_and_location[0].split(" ").collect();
    let request_type = request_type_and_location[0];
    let request_location = request_type_and_location[1];
    println!("type: {}\nlocation: {}\n", request_type, request_location);

    let response = match request_location {
        // root
        "/" | "/index" | "/index.html" | "/static/index.html" => handle_index(request_type),
        // 404
        _ => handle_error(404, "route not handled")
    };


    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}