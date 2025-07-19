use walkdir::WalkDir;
use clap::Parser;


#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// The name to search for (partial match)
    filename: String,

    /// The directory to start from
    #[arg(short, long, default_value = ".", value_name = "PATH")]
    path: String,

    /// Include directories in the search (in addition to files)
    #[arg(short = 'd', long = "include-dirs")]
    include_dirs: bool,

    /// Only include directories (exclude files)
    #[arg(short = 'D', long = "only-dirs")]
    only_dirs: bool,

    /// Specific file types
    #[arg(short = 't', long = "file-type")]
    file_type: Option<String>,
}

/// Function to determine if a file or directory is allowed based on the provided arguments.
/// 
/// # Parameters:
/// - `is_file`: A boolean indicating whether the current item is a file.
/// - `is_dir`: A boolean indicating whether the current item is a directory.
/// - `args`: An instance of `Args` holding configuration options.
/// 
/// # Returns:
/// - A boolean value indicating whether the current item is allowed based on the arguments.
fn is_allowed(is_file: bool, is_dir: bool, args: &Args) -> bool {

    
    return if args.only_dirs {
        is_dir
    } else if args.include_dirs {
        is_file || is_dir
    } else {
        is_file
    };

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

        let should_include = is_allowed(is_file, is_dir, &args);

        if should_include {
            if let Some(name) = entry.path().file_name().and_then(|n| n.to_str()) {
                if name.contains(&args.filename) {
                // ✅ Check file type if specified
                if let Some(ref file_type) = args.file_type {
                    if let Some(ext) = entry.path().extension().and_then(|e| e.to_str()) {
                        if ext == file_type {
                            println!("{}", entry.path().display());
                        }
                    }
                } else {
                    // No file_type filter set — include all matching names
                    println!("{}", entry.path().display());
                }
            }
            }
        }
    }
}

