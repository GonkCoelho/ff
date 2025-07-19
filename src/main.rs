use clap::Parser;
use ff::{Args};
use ff::file_Searcher::{FileSearcher};

/// Entry point of the program. Parses CLI arguments and initiates the search.
fn main() {
    let args = Args::parse();
    let searcher = FileSearcher::new(args);
    searcher.run();
}
