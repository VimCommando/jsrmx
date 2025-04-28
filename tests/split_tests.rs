use assert_cmd::Command;
use serde_json::json;

#[test]
fn split_file_to_directory() -> std::io::Result<()> {
    // Setup: Create a temporary directory and input file
    let temp_dir = tempfile::tempdir()?;
    let input_file = temp_dir.path().join("input.json");
    let output_dir = temp_dir.path().join("output");

    // Create input JSON file
    let input_json = json!({
        "alpha": {"uppercase": "A", "lowercase": "a", "position": 1},
        "bravo": {"uppercase": "B", "lowercase": "b", "position": 2}
    });
    std::fs::write(&input_file, serde_json::to_string_pretty(&input_json)?)?;

    // Run the split command
    let _ = Command::cargo_bin("jsrmx")
        .unwrap()
        .arg("split")
        .arg(&input_file)
        .arg(&output_dir)
        .assert()
        .success();

    // Check that output files were created
    assert!(output_dir.join("alpha.json").exists());
    assert!(output_dir.join("bravo.json").exists());

    // Check content of output files
    let alpha_content: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(output_dir.join("alpha.json"))?)?;
    assert_eq!(alpha_content, input_json["alpha"]);

    let bravo_content: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(output_dir.join("bravo.json"))?)?;
    assert_eq!(bravo_content, input_json["bravo"]);

    Ok(())
}

#[test]
fn split_stdin_to_stdout() -> std::io::Result<()> {
    let input_json = json!({
        "alpha": {"uppercase": "A", "lowercase": "a", "position": 1},
        "bravo": {"uppercase": "B", "lowercase": "b", "position": 2}
    });

    let _ = Command::cargo_bin("jsrmx")
        .unwrap()
        .arg("split")
        .arg("-") // Use stdin
        .arg("-") // Use stdout
        .write_stdin(serde_json::to_string(&input_json)?)
        .assert()
        .success()
        .stdout(predicates::str::contains("\"alpha\":"))
        .stdout(predicates::str::contains("\"bravo\":"));

    Ok(())
}

#[test]
fn split_with_drop() -> std::io::Result<()> {
    // Setup: Create a temporary directory and input file
    let temp_dir = tempfile::tempdir()?;
    let input_file = temp_dir.path().join("input.json");
    let output_dir = temp_dir.path().join("output");

    // Create input JSON file
    let input_json = json!({
        "alpha": {"letter": {"uppercase": "A", "lowercase": "a"}, "position": 1},
        "bravo": {"letter": {"uppercase": "B", "lowercase": "b"}, "position": 2},
        "charlie": {"letter": {"uppercase": "C", "lowercase": "c"}, "position": 3},
    });

    // Create expected output JSON files
    let expected_alpha = json!({
        "letter": {"uppercase": "A"},
    });
    let expected_bravo = json!({
        "letter": {"uppercase": "B"},
    });
    let expected_charlie = json!({
        "letter": {"uppercase": "C"},
    });

    std::fs::write(&input_file, serde_json::to_string_pretty(&input_json)?)?;

    // Run the split command
    let _ = Command::cargo_bin("jsrmx")
        .unwrap()
        .arg("split")
        .arg("--drop=position,letter.lowercase")
        .arg(&input_file)
        .arg(&output_dir)
        .assert()
        .success();

    // Check that output files were created
    assert!(output_dir.join("alpha.json").exists());
    assert!(output_dir.join("bravo.json").exists());
    assert!(output_dir.join("charlie.json").exists());

    // Check content of output files
    let alpha_content: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(output_dir.join("alpha.json"))?)?;
    assert_eq!(alpha_content, expected_alpha);

    let bravo_content: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(output_dir.join("bravo.json"))?)?;
    assert_eq!(bravo_content, expected_bravo);

    let charlie_content: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(output_dir.join("charlie.json"))?)?;
    assert_eq!(charlie_content, expected_charlie);

    Ok(())
}

// TODO: The --compact option is not yet implemented for the split command
// #[test]
// fn split_compact() -> std::io::Result<()> {
//     // Similar to split_stdin_to_stdout, but add --compact flag
//     // Check that output doesn't contain newlines (except between objects)
//     Ok(())
// }

// TODO: The --filter option is not yet implemented for the split command
// #[test]
// fn split_filter() -> std::io::Result<()> {
//     // Similar to split_stdin_to_stdout, but add --filter flag
//     // Check that only filtered keys are in the output
//     Ok(())
// }
