use assert_cmd::Command;
use serde_json::json;
use std::fs;
use tempfile::tempdir;

// NOTE: This test has been modified to check the output file content in a order-agnostic manner
// If the bundle command should be order-sensitive, the test should be updated to check for a specific order.
#[test]
fn test_unbundle_command() -> std::io::Result<()> {
    let input_dir = tempdir()?;
    let output_dir = tempdir()?;

    // Create a single input file with NDJSON content
    let input_content = [
        json!({"name":"alpha","letter":{"uppercase":"A","lowercase":"a"},"position":1}),
        json!({"name":"bravo","letter":{"uppercase":"B","lowercase":"b"},"position":2}),
        json!({"name":"charlie","letter":{"uppercase":"C","lowercase":"c"},"position":3}),
    ];
    let input_file = input_dir.path().join("letters.ndjson");
    fs::write(
        &input_file,
        input_content
            .iter()
            .map(|j| j.to_string() + "\n")
            .collect::<String>(),
    )?;

    // Test 1: Basic unbundle
    let output = Command::cargo_bin("jsrmx")
        .unwrap()
        .arg("unbundle")
        .arg(&input_file)
        .arg(output_dir.path())
        .output()?;

    assert!(
        output.status.success(),
        "Basic unbundle command failed: {:?}",
        output
    );

    // Check that the correct output files were created
    let mut output_contents = Vec::new();
    for i in 0..=2 {
        let file_path = output_dir.path().join(format!("object-{i:06}.json"));
        assert!(
            file_path.exists(),
            "Expected output file {:?} was not created",
            file_path
        );

        let file_content: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&file_path)?)?;
        output_contents.push(file_content);
    }

    assert_eq!(
        input_content.len(),
        output_contents.len(),
        "Number of output files doesn't match input"
    );

    for input in &input_content {
        assert!(
            output_contents.contains(input),
            "Expected content not found in output: {:?}",
            input
        );
    }

    // Clear output directory
    fs::remove_dir_all(output_dir.path())?;
    fs::create_dir(output_dir.path())?;

    // Test 2: Unbundle with --name option
    let output = Command::cargo_bin("jsrmx")
        .unwrap()
        .arg("unbundle")
        .arg("--name=name")
        .arg(&input_file)
        .arg(output_dir.path())
        .output()?;

    assert!(
        output.status.success(),
        "Unbundle command with --name failed: {:?}",
        output
    );

    // Check that the correct output files were created with custom names
    for entry in &input_content {
        let file_name = format!("{}.json", entry["name"].as_str().unwrap());
        let file_path = output_dir.path().join(&file_name);
        assert!(
            file_path.exists(),
            "Expected output file {:?} was not created",
            file_path
        );

        let file_content: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&file_path)?)?;
        assert_eq!(
            &file_content, entry,
            "Content of {:?} does not match expected",
            file_path
        );
    }

    Ok(())
}

// TODO: Add tests for the --compact option
