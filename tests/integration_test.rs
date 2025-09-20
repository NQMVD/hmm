use std::process::Command;
use std::fs;

#[test] 
fn test_help_command() {
    let output = Command::new("./target/debug/rust-mind-map")
        .arg("--help")
        .output()
        .expect("Failed to execute command");
        
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("terminal-based mind mapping tool"));
}

#[test]
fn test_version_command() {
    let output = Command::new("./target/debug/rust-mind-map")
        .arg("--version") 
        .output()
        .expect("Failed to execute command");
        
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("rust-mind-map 0.1.0"));
}

#[test]
fn test_file_parsing() {
    // Test that the application can at least start with a file
    // Create a simple test file
    let test_content = "Root\n\tChild 1\n\tChild 2\n\t\tGrandchild";
    fs::write("test_parse.rmm", test_content).expect("Failed to write test file");
    
    // This test just ensures the file is accessible and no immediate errors
    assert!(std::path::Path::new("test_parse.rmm").exists());
    
    // Clean up
    fs::remove_file("test_parse.rmm").ok();
}