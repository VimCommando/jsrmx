use assert_cmd::Command;
use serde_json::json;
use std::fs;
use tempfile::tempdir;

fn create_input_files(dir: &tempfile::TempDir) -> std::io::Result<()> {
    let files = vec![
        (
            "alpha.json",
            json!({"letter": {"lowercase": "a", "uppercase": "A"}, "name": "alpha", "position": 1}),
        ),
        (
            "bravo.json",
            json!({"letter": {"lowercase": "b", "uppercase": "B"}, "name": "bravo", "position": 2}),
        ),
        (
            "charlie.json",
            json!({"letter": {"lowercase": "c", "uppercase": "C"}, "name": "charlie", "position": 3}),
        ),
        (
            "delta.json",
            json!({"letter": {"lowercase": "d", "uppercase": "D"}, "name": "delta", "position": 4}),
        ),
        (
            "echo.json",
            json!({"letter": {"lowercase": "e", "uppercase": "E"}, "name": "echo", "position": 5}),
        ),
        (
            "foxtrot.json",
            json!({"letter": {"lowercase": "f", "uppercase": "F"}, "name": "foxtrot", "position": 6}),
        ),
    ];

    for (filename, content) in files {
        let file_path = dir.path().join(filename);
        fs::write(file_path, content.to_string())?;
    }

    Ok(())
}

#[test]
fn bundle_command() -> std::io::Result<()> {
    let input_dir = tempdir()?;
    let output_dir = tempdir()?;
    create_input_files(&input_dir)?;

    let output_file = output_dir.path().join("letters.ndjson");

    // Run the bundle command
    let output = Command::cargo_bin("jsrmx")
        .unwrap()
        .arg("bundle")
        .arg(input_dir.path())
        .arg(&output_file)
        .output()?;

    assert!(
        output.status.success(),
        "Bundle command failed: {:?}",
        output
    );

    // Check that the output file was created
    assert!(output_file.exists(), "Output file was not created");

    // Read and parse the output file
    let output_content = fs::read_to_string(&output_file)?;
    let output_lines: Vec<serde_json::Value> = output_content
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();

    // Expected output (order may vary)
    let expected_output = vec![
        json!({"name":"alpha","letter":{"uppercase":"A","lowercase":"a"},"position":1}),
        json!({"name":"bravo","letter":{"uppercase":"B","lowercase":"b"},"position":2}),
        json!({"name":"charlie","letter":{"uppercase":"C","lowercase":"c"},"position":3}),
        json!({"name":"delta","letter":{"uppercase":"D","lowercase":"d"},"position":4}),
        json!({"name":"echo","letter":{"uppercase":"E","lowercase":"e"},"position":5}),
        json!({"name":"foxtrot","letter":{"uppercase":"F","lowercase":"f"},"position":6}),
    ];

    // Check that all expected items are in the output
    for expected in &expected_output {
        assert!(
            output_lines.contains(expected),
            "Expected item not found in output: {:?}",
            expected
        );
    }

    // Check that the number of items matches
    assert_eq!(
        expected_output.len(),
        output_lines.len(),
        "Number of output items doesn't match expected"
    );

    Ok(())
}

#[test]
fn bundle_command_stdout() -> std::io::Result<()> {
    let input_dir = tempdir()?;
    create_input_files(&input_dir)?;

    // Run the bundle command with stdout
    let output = Command::cargo_bin("jsrmx")
        .unwrap()
        .arg("bundle")
        .arg(input_dir.path())
        .arg("-")
        .output()?;

    assert!(
        output.status.success(),
        "Bundle command failed: {:?}",
        output
    );

    // Parse the stdout output
    let stdout_content = String::from_utf8_lossy(&output.stdout);
    let output_lines: Vec<serde_json::Value> = stdout_content
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();

    // Expected output (order may vary)
    let expected_output = vec![
        json!({"name":"alpha","letter":{"uppercase":"A","lowercase":"a"},"position":1}),
        json!({"name":"bravo","letter":{"uppercase":"B","lowercase":"b"},"position":2}),
        json!({"name":"charlie","letter":{"uppercase":"C","lowercase":"c"},"position":3}),
        json!({"name":"delta","letter":{"uppercase":"D","lowercase":"d"},"position":4}),
        json!({"name":"echo","letter":{"uppercase":"E","lowercase":"e"},"position":5}),
        json!({"name":"foxtrot","letter":{"uppercase":"F","lowercase":"f"},"position":6}),
    ];

    // Check that all expected items are in the output
    for expected in &expected_output {
        assert!(
            output_lines.contains(expected),
            "Expected item not found in output: {:?}",
            expected
        );
    }

    // Check that the number of items matches
    assert_eq!(
        expected_output.len(),
        output_lines.len(),
        "Number of output items doesn't match expected"
    );

    Ok(())
}

#[test]
fn bundle_command_with_drop() -> std::io::Result<()> {
    let input_dir = tempdir()?;
    create_input_files(&input_dir)?;

    // Run the bundle command with stdout
    let output = Command::cargo_bin("jsrmx")
        .unwrap()
        .arg("bundle")
        .arg("--drop=position,letter.lowercase")
        .arg(input_dir.path())
        .arg("-")
        .output()?;

    assert!(
        output.status.success(),
        "Bundle command failed: {:?}",
        output
    );

    // Parse the stdout output
    let stdout_content = String::from_utf8_lossy(&output.stdout);
    let output_lines: Vec<serde_json::Value> = stdout_content
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();

    // Expected output (order may vary)
    let expected_output = vec![
        json!({"name":"alpha","letter":{"uppercase":"A"}}),
        json!({"name":"bravo","letter":{"uppercase":"B"}}),
        json!({"name":"charlie","letter":{"uppercase":"C"}}),
        json!({"name":"delta","letter":{"uppercase":"D"}}),
        json!({"name":"echo","letter":{"uppercase":"E"}}),
        json!({"name":"foxtrot","letter":{"uppercase":"F"}}),
    ];

    // Check that all expected items are in the output
    for expected in &expected_output {
        assert!(
            output_lines.contains(expected),
            "Expected item not found in output: {:?}",
            expected
        );
    }

    // Check that the number of items matches
    assert_eq!(
        expected_output.len(),
        output_lines.len(),
        "Number of output items doesn't match expected"
    );

    Ok(())
}
