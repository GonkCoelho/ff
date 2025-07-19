use walkdir::WalkDir;
use clap::Parser;


#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// The name to search for (partial match)
    filename: String,

    /// The directory to start from
    #[arg(default_value = ".")]
    path: String,

    /// Include directories in the search (in addition to files)
    #[arg(short = 'd', long = "include-dirs")]
    include_dirs: bool,

    /// Only include directories (exclude files)
    #[arg(short = 'D', long = "only-dirs")]
    only_dirs: bool,
}


fn main() {
    //Parses the arguments in the Struct ARGS
    let args = Args::parse();

    println!("Searching for: {}", args.filename);
    println!("Starting in: {}", args.path);

    //Saving the variables 
    let filename = args.filename;
    let starting_dir = args.path;

    // Cycling 
    for entry in WalkDir::new(starting_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        if let Some(file_name) = entry.path().file_name().and_then(|n| n.to_str()) {
            if file_name.contains(&filename) {
                println!("{}", entry.path().display());
            }
        }
    }
}

