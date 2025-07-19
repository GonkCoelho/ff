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

    for entry in WalkDir::new(&args.path)
        .into_iter()
        .filter_map(Result::ok)
    {
        let is_file = entry.file_type().is_file();
        let is_dir = entry.file_type().is_dir();

        let should_include = if args.only_dirs {
            is_dir
        } else if args.include_dirs {
            is_file || is_dir
        } else {
            is_file
        };

        if should_include {
            if let Some(name) = entry.path().file_name().and_then(|n| n.to_str()) {
                if name.contains(&args.filename) {
                    println!("{}", entry.path().display());
                }
            }
        }
    }
}

