use assert_cmd::Command;
use serde_json::json;
use std::fs;
use tempfile::tempdir;

#[test]
fn merge_basic() -> std::io::Result<()> {
    let (input_dir, output_dir, _files) = setup_merge_test()?;
    let output_file = output_dir.path().join("merged.json");

    let output = Command::cargo_bin("jsrmx")
        .unwrap()
        .arg("merge")
        .arg(input_dir.path())
        .arg(&output_file)
        .output()?;

    assert!(
        output.status.success(),
        "Basic merge command failed: {:?}",
        output
    );

    let merged_content: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&output_file)?)?;

    let expected_content = json!({
        "alpha": {"letter": {"uppercase": "A", "lowercase": "a"}, "position": 1},
        "bravo": {"letter": {"uppercase": "B", "lowercase": "b"}, "position": 2},
        "charlie": {"letter": {"uppercase": "C", "lowercase": "c"}, "position": 3},
        "delta": {"letter": {"uppercase": "D", "lowercase": "d"}, "position": 4},
        "echo": {"letter": {"uppercase": "E", "lowercase": "e"}, "position": 5},
        "foxtrot": {"letter": {"uppercase": "F", "lowercase": "f"}, "position": 6},
    });

    assert_eq!(
        merged_content, expected_content,
        "Merged content does not match expected"
    );

    Ok(())
}

// TODO: The --compact option is not yet implemented for the merge command
// #[test]
// fn merge_compact() -> std::io::Result<()> {
//     let (input_dir, output_dir, _) = setup_merge_test()?;
//     let output_file = output_dir.path().join("merged.json");

//     let output = Command::cargo_bin("jsrmx")
//         .unwrap()
//         .arg("merge")
//         .arg("--compact")
//         .arg(input_dir.path())
//         .arg(&output_file)
//         .output()?;

//     assert!(
//         output.status.success(),
//         "Merge command with --compact failed: {:?}",
//         output
//     );

//     let compact_content = fs::read_to_string(&output_file)?;
//     assert!(
//         !compact_content.contains('\n'),
//         "Compact output should not contain newlines"
//     );

//     Ok(())
// }

#[test]
fn merge_filter() -> std::io::Result<()> {
    let (input_dir, output_dir, _) = setup_merge_test()?;
    let output_file = output_dir.path().join("merged.json");

    let output = Command::cargo_bin("jsrmx")
        .unwrap()
        .arg("merge")
        .arg("--filter")
        .arg("alpha|charlie")
        .arg(input_dir.path())
        .arg(&output_file)
        .output()?;

    assert!(
        output.status.success(),
        "Merge command with --filter failed: {:?}",
        output
    );

    let filtered_content: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&output_file)?)?;

    let expected_filtered = json!({
        "alpha": {"letter": {"uppercase": "A", "lowercase": "a"}, "position": 1},
        "charlie": {"letter": {"uppercase": "C", "lowercase": "c"}, "position": 3},
    });

    assert_eq!(
        filtered_content, expected_filtered,
        "Filtered content does not match expected"
    );

    Ok(())
}

#[test]
fn merge_with_drop() -> std::io::Result<()> {
    let (input_dir, output_dir, _) = setup_merge_test()?;
    let output_file = output_dir.path().join("merged.json");

    let output = Command::cargo_bin("jsrmx")
        .unwrap()
        .arg("merge")
        .arg("--drop=alpha.position,foxtrot")
        .arg(input_dir.path())
        .arg(&output_file)
        .output()?;

    assert!(
        output.status.success(),
        "Merge command with --drop failed: {:?}",
        output
    );

    let dropped_content: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&output_file)?)?;

    let expected_content = json!({
        "alpha": {"letter": {"uppercase": "A", "lowercase": "a"}},
        "bravo": {"letter": {"uppercase": "B", "lowercase": "b"}, "position": 2},
        "charlie": {"letter": {"uppercase": "C", "lowercase": "c"}, "position": 3},
        "delta": {"letter": {"uppercase": "D", "lowercase": "d"}, "position": 4},
        "echo": {"letter": {"uppercase": "E", "lowercase": "e"}, "position": 5},
    });

    assert_eq!(
        dropped_content, expected_content,
        "Dropped content does not match expected"
    );

    Ok(())
}

// TODO: The --trim option is not yet implemented for the merge command
// #[test]
// fn merge_trim() -> std::io::Result<()> {
//     let (input_dir, output_dir, _) = setup_merge_test()?;
//     let output_file = output_dir.path().join("merged.json");

//     let output = Command::cargo_bin("jsrmx")
//         .unwrap()
//         .arg("merge")
//         .arg("--trim")
//         .arg(".json")
//         .arg(input_dir.path())
//         .arg(&output_file)
//         .output()?;

//     assert!(
//         output.status.success(),
//         "Merge command with --trim failed: {:?}",
//         output
//     );

//     let trimmed_content: serde_json::Value =
//         serde_json::from_str(&fs::read_to_string(&output_file)?)?;
//     let expected_content = json!({
//         "alpha": {"uppercase": "A", "lowercase": "a", "position": 1},
//         "bravo": {"uppercase": "B", "lowercase": "b", "position": 2},
//         "charlie": {"uppercase": "C", "lowercase": "c", "position": 3},
//         "delta": {"uppercase": "D", "lowercase": "d", "position": 4},
//         "echo": {"uppercase": "E", "lowercase": "e", "position": 5},
//         "foxtrot": {"uppercase": "F", "lowercase": "f", "position": 6},
//     });

//     assert_eq!(
//         trimmed_content, expected_content,
//         "Trimmed content should match the basic merge result"
//     );

//     Ok(())
// }

type MergeTestSetup = (
    tempfile::TempDir,
    tempfile::TempDir,
    Vec<(String, serde_json::Value)>,
);

fn setup_merge_test() -> std::io::Result<MergeTestSetup> {
    let input_dir = tempdir()?;
    let output_dir = tempdir()?;

    let files = vec![
        (
            "alpha.json".to_string(),
            json!({"letter": {"uppercase": "A", "lowercase": "a"}, "position": 1}),
        ),
        (
            "bravo.json".to_string(),
            json!({"letter": {"uppercase": "B", "lowercase": "b"}, "position": 2}),
        ),
        (
            "charlie.json".to_string(),
            json!({"letter": {"uppercase": "C", "lowercase": "c"}, "position": 3}),
        ),
        (
            "delta.json".to_string(),
            json!({"letter": {"uppercase": "D", "lowercase": "d"}, "position": 4}),
        ),
        (
            "echo.json".to_string(),
            json!({"letter": {"uppercase": "E", "lowercase": "e"}, "position": 5}),
        ),
        (
            "foxtrot.json".to_string(),
            json!({"letter": {"uppercase": "F", "lowercase": "f"}, "position": 6}),
        ),
    ];

    for (filename, content) in &files {
        fs::write(
            input_dir.path().join(filename),
            serde_json::to_string_pretty(content)?,
        )?;
    }

    Ok((input_dir, output_dir, files))
}
