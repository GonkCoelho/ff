pub mod file_Searcher;

use clap::Parser;


#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// The name to search for (partial match)
    pub filename: String,

    /// The directory to start from
    #[arg(short, long, default_value = ".", value_name = "PATH")]
    pub path: String,

    /// Include directories in the search (in addition to files)
    #[arg(short = 'd', long = "include-dirs")]
    pub include_dirs: bool,

    /// Only include directories (exclude files)
    #[arg(short = 'D', long = "only-dirs")]
    pub only_dirs: bool,

    /// Specific file types
    #[arg(short = 't', long = "file-type")]
    pub file_type: Option<String>,

    /// Max Depth that it will go to
    #[arg(short = 'M', long = "max-depth")]
    max_depth: Option<usize>,
}



