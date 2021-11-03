use structopt::StructOpt;
use std::net::TcpListener;
use std::io::Write;    // for TcpStream.flush()

#[derive(Debug, StructOpt)]
#[structopt(name = "arguments", about = "Command-line arguments.")]
struct Opt {
    version: i16,
}

fn main() {
    let opt = Opt::from_args();
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();    // "mut" for .flush()
        let response = "HTTP/1.1 200 OK\r\n\r\n";

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();

        println!("Hello, structopt: {:?}", opt);
    }
}
