use structopt::StructOpt;
use std::path::PathBuf;
use std::fs;
use std::str::FromStr;
use std::net::TcpListener;
use std::io::Write;    // for TcpStream.flush()


#[derive(Debug, StructOpt)]
#[structopt(name = "arguments", about = "Command-line arguments.")]
struct Opt {
    #[structopt(short = "v", long = "version", default_value = "1.0")]
    version: f32,
    #[structopt(parse(from_os_str))]
    path: PathBuf,
}


fn main() {
    let opt = Opt::from_args();
    let mut cnt: i32 = 1;

    if (&opt.path).exists() {
        let data = fs::read_to_string(&opt.path)
            .expect("File read I/O Error");
        println!("File contents: {}", &data);
        cnt = i32::from_str(&data)
            .unwrap_or(1);
        cnt += 1;
    }
    let mut file = fs::File::create(&opt.path)
        .expect("File open I/O Error");
    file.write_all(cnt.to_string().as_bytes())
        .expect("File write I/O Error");


    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();    // "mut" for .flush()
        let response = "HTTP/1.1 200 OK\r\n\r\n";

        println!("Responding to ping on HTTP port; deployed {} times.", cnt);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
