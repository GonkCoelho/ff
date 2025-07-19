use walkdir::{DirEntry, WalkDir};
use crate::Args;


pub struct FileSearcher {
    pub args: Args,
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
    pub fn new(args: Args) -> Self {
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
    pub fn is_allowed(&self, entry: &DirEntry) -> bool {
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
    pub fn name_matches(&self, entry: &DirEntry) -> bool {
        if let Some(name) = entry.path().file_name().and_then(|n| n.to_str()) {
            if self.args.ignore_case {
                name.to_lowercase().contains(&self.args.filename.to_lowercase())
            } else {
                name.contains(&self.args.filename)
            }
        } else {
            false
        }
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
    pub fn file_type_matches(&self, entry: &DirEntry) -> bool {
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

    pub fn search(&self){
        let walker = WalkDir::new(&self.args.path);
            

        let walker = if let Some(depth) = self.args.max_depth{
            walker.max_depth(depth)
        } else {
            walker
        };

        for entry in walker
            .into_iter()
            .filter_map(Result::ok)
        {
            if self.is_allowed(&entry) && self.name_matches(&entry) && self.file_type_matches(&entry) {
                println!("{}", entry.path().display());
            }
        }
    }

    /// Executes the file search using the configured arguments.
    ///
    /// Recursively traverses the file system starting from the given path,
    /// and prints out the entries that match the search criteria.
    pub fn run(&self) {
        
        self.search();
    }
}