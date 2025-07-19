use clap::Parser;
use ff::{Cli, Commands};
use ff::file_Searcher::FileSearcher;

fn main() {
    let cli = Cli::parse();

    match cli.command {
    Some(Commands::Version) => {
        println!("file_searcher version {}", env!("CARGO_PKG_VERSION"));
    }
    Some(Commands::Search(args)) => {
        let searcher = FileSearcher::new(args);
        searcher.run();
    }
    None => {
        eprintln!("No subcommand provided. Use --help to see usage.");
    }
}
}
