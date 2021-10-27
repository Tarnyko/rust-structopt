use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    version: i16,
}

fn main() {
    let opt = Opt::from_args();
    println!("Hello, structopt: {:?}", opt);
}
