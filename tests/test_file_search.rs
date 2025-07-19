use ff::{Args};
use ff::file_searcher::{FileSearcher};
use std::fs::{self, File};
use tempfile::TempDir;
use walkdir::WalkDir;

// Helper function to create a FileSearcher with default args
fn create_searcher(
    filename: &str,
    path: &str,
    include_dirs: bool,
    only_dirs: bool,
    file_type: Option<String>,
) -> FileSearcher {
    let args = Args {
        filename: filename.to_string(),
        path: path.to_string(),
        include_dirs,
        only_dirs,
        file_type,
    };
    FileSearcher::new(args)
}

// Helper function to create a temporary directory structure for testing
fn create_test_structure() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let base_path = temp_dir.path();

    // Create files
    File::create(base_path.join("test_file.txt")).expect("Failed to create test_file.txt");
    File::create(base_path.join("another_test.rs")).expect("Failed to create another_test.rs");
    File::create(base_path.join("no_match.py")).expect("Failed to create no_match.py");
    File::create(base_path.join("README.md")).expect("Failed to create README.md");

    // Create directories
    fs::create_dir(base_path.join("test_dir")).expect("Failed to create test_dir");
    fs::create_dir(base_path.join("src")).expect("Failed to create src");
    fs::create_dir(base_path.join("another_test_folder")).expect("Failed to create another_test_folder");

    // Create nested structure
    fs::create_dir(base_path.join("nested")).expect("Failed to create nested");
    File::create(base_path.join("nested").join("nested_test.txt"))
        .expect("Failed to create nested_test.txt");
    fs::create_dir(base_path.join("nested").join("test_nested_dir"))
        .expect("Failed to create test_nested_dir");

    temp_dir
}

#[test]
fn test_new_creates_file_searcher() {
    let args = Args {
        filename: "test".to_string(),
        path: ".".to_string(),
        include_dirs: false,
        only_dirs: false,
        file_type: None,
    };
    let searcher = FileSearcher::new(args);
    assert_eq!(searcher.args.filename, "test");
    assert_eq!(searcher.args.path, ".");
    assert!(!searcher.args.include_dirs);
    assert!(!searcher.args.only_dirs);
    assert!(searcher.args.file_type.is_none());
}

#[test]
fn test_files_only_search() {
    let temp_dir = create_test_structure();
    let searcher = create_searcher("test", temp_dir.path().to_str().unwrap(), false, false, None);

    let mut matches = Vec::new();
    for entry in WalkDir::new(temp_dir.path()).into_iter().filter_map(Result::ok) {
        if searcher.is_allowed(&entry) && searcher.name_matches(&entry) && searcher.file_type_matches(&entry) {
            matches.push(entry.path().to_path_buf());
        }
    }

    // Should find test_file.txt, another_test.rs, and nested_test.txt
    assert_eq!(matches.len(), 3, "Should find 3 matching files");
    
    let match_names: Vec<String> = matches
        .iter()
        .map(|p| p.file_name().unwrap().to_str().unwrap().to_string())
        .collect();
    
    assert!(match_names.contains(&"test_file.txt".to_string()));
    assert!(match_names.contains(&"another_test.rs".to_string()));
    assert!(match_names.contains(&"nested_test.txt".to_string()));
}

#[test]
fn test_directories_only_search() {
    let temp_dir = create_test_structure();
    let searcher = create_searcher("test", temp_dir.path().to_str().unwrap(), false, true, None);

    let mut matches = Vec::new();
    for entry in WalkDir::new(temp_dir.path()).into_iter().filter_map(Result::ok) {
        if searcher.is_allowed(&entry) && searcher.name_matches(&entry) && searcher.file_type_matches(&entry) {
            matches.push(entry.path().to_path_buf());
        }
    }

    // Should find test_dir, another_test_folder, and test_nested_dir
    assert_eq!(matches.len(), 3, "Should find 3 matching directories");
    
    let match_names: Vec<String> = matches
        .iter()
        .map(|p| p.file_name().unwrap().to_str().unwrap().to_string())
        .collect();
    
    assert!(match_names.contains(&"test_dir".to_string()));
    assert!(match_names.contains(&"another_test_folder".to_string()));
    assert!(match_names.contains(&"test_nested_dir".to_string()));
}

#[test]
fn test_file_type_filtering() {
    let temp_dir = create_test_structure();
    let searcher = create_searcher("test", temp_dir.path().to_str().unwrap(), false, false, Some("txt".to_string()));

    let mut matches = Vec::new();
    for entry in WalkDir::new(temp_dir.path()).into_iter().filter_map(Result::ok) {
        if searcher.is_allowed(&entry) && searcher.name_matches(&entry) && searcher.file_type_matches(&entry) {
            matches.push(entry.path().to_path_buf());
        }
    }

    // Should only find test_file.txt and nested_test.txt
    assert_eq!(matches.len(), 2, "Should find 2 matching .txt files");
    
    let match_names: Vec<String> = matches
        .iter()
        .map(|p| p.file_name().unwrap().to_str().unwrap().to_string())
        .collect();
    
    assert!(match_names.contains(&"test_file.txt".to_string()));
    assert!(match_names.contains(&"nested_test.txt".to_string()));
}

#[test]
fn test_include_both_files_and_dirs() {
    let temp_dir = create_test_structure();
    let searcher = create_searcher("test", temp_dir.path().to_str().unwrap(), true, false, None);

    let mut matches = Vec::new();
    for entry in WalkDir::new(temp_dir.path()).into_iter().filter_map(Result::ok) {
        if searcher.is_allowed(&entry) && searcher.name_matches(&entry) && searcher.file_type_matches(&entry) {
            matches.push(entry.path().to_path_buf());
        }
    }

    // Should find all files and directories containing "test"
    assert_eq!(matches.len(), 6, "Should find 6 matching files and directories");
}

#[test]
fn test_no_matches() {
    let temp_dir = create_test_structure();
    let searcher = create_searcher("nonexistent", temp_dir.path().to_str().unwrap(), true, false, None);

    let mut matches = Vec::new();
    for entry in WalkDir::new(temp_dir.path()).into_iter().filter_map(Result::ok) {
        if searcher.is_allowed(&entry) && searcher.name_matches(&entry) && searcher.file_type_matches(&entry) {
            matches.push(entry.path().to_path_buf());
        }
    }

    assert_eq!(matches.len(), 0, "Should find no matches for nonexistent pattern");
}

#[test]
fn test_case_sensitive_search() {
    let temp_dir = create_test_structure();
    let searcher = create_searcher("Test", temp_dir.path().to_str().unwrap(), true, false, None);

    let mut matches = Vec::new();
    for entry in WalkDir::new(temp_dir.path()).into_iter().filter_map(Result::ok) {
        if searcher.is_allowed(&entry) && searcher.name_matches(&entry) && searcher.file_type_matches(&entry) {
            matches.push(entry.path().to_path_buf());
        }
    }

    // Should find no matches since all test files use lowercase "test"
    assert_eq!(matches.len(), 0, "Should find no matches for case-sensitive 'Test'");
}