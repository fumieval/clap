use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    name: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    println!("name: {:?}", cli.name);
}
