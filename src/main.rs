use clap::Parser;
use walkdir::{DirEntry, WalkDir};

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

struct FileSearcher {
    args: Args,
}

impl FileSearcher {
    /// Constructs a new `FileSearcher` with the provided CLI arguments.
    ///
    /// # Arguments
    ///
    /// * `args` - An instance of `Args` containing user input
    ///
    /// # Returns
    ///
    /// A `FileSearcher` ready to perform the search.
    fn new(args: Args) -> Self {
        FileSearcher { args }
    }

    /// Checks whether the entry should be included in the results based on its type
    /// (file or directory) and the search configuration.
    ///
    /// # Arguments
    ///
    /// * `entry` - A reference to the `DirEntry` being evaluated
    ///
    /// # Returns
    ///
    /// `true` if the entry should be included; otherwise `false`.
    fn is_allowed(&self, entry: &DirEntry) -> bool {
        let is_file = entry.file_type().is_file();
        let is_dir = entry.file_type().is_dir();

        if self.args.only_dirs {
            return is_dir;
        } else if self.args.include_dirs {
            return is_file || is_dir;
        } else {
            return is_file;
        }
    }

    /// Checks if the name of the file or directory matches the search string.
    ///
    /// # Arguments
    ///
    /// * `entry` - A reference to the `DirEntry` being evaluated
    ///
    /// # Returns
    ///
    /// `true` if the entry's name contains the target string; otherwise `false`.
    fn name_matches(&self, entry: &DirEntry) -> bool {
        entry
            .file_name()
            .to_str()
            .map(|name| name.contains(&self.args.filename))
            .unwrap_or(false)
    }

    /// Checks if the file extension matches the user-specified file type (if provided).
    ///
    /// # Arguments
    ///
    /// * `entry` - A reference to the `DirEntry` being evaluated
    ///
    /// # Returns
    ///
    /// `true` if the file type matches or no type was specified; otherwise `false`.
    fn file_type_matches(&self, entry: &DirEntry) -> bool {
        match &self.args.file_type {
            Some(file_type) => entry
                .path()
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext == file_type)
                .unwrap_or(false),
            None => true,
        }
    }

    /// Executes the file search using the configured arguments.
    ///
    /// Recursively traverses the file system starting from the given path,
    /// and prints out the entries that match the search criteria.
    fn run(&self) {
        println!("Searching for: {}", self.args.filename);
        println!("Starting in: {}", self.args.path);

        for entry in WalkDir::new(&self.args.path)
            .into_iter()
            .filter_map(Result::ok)
        {
            if self.is_allowed(&entry) && self.name_matches(&entry) && self.file_type_matches(&entry) {
                println!("{}", entry.path().display());
            }
        }
    }
}

/// Entry point of the program. Parses CLI arguments and initiates the search.
fn main() {
    let args = Args::parse();
    let searcher = FileSearcher::new(args);
    searcher.run();
}
