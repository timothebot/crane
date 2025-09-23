use std::vec;

use crane_bricks::{
    actions::{
        Action,
        common::Common,
        insert_file::{FileExistsAction, InsertFileAction},
    },
    brick::{Brick, BrickConfig},
    context::ActionContext,
};
use log::debug;

use common::*;

mod common;

#[test]
fn test_test_functions() {
    let tmpdir = tempfile::tempdir().unwrap();
    add_test_data(tmpdir.path(), "Test.toml");

    assert!(tmpdir.path().join("Test.toml").exists());
}

#[test]
fn test_actions_parse() {
    init_logger();

    let config = r#"
name = "hi"

[[actions]]
action = "insert_file"

"#;
    let config_parsed: BrickConfig = toml::from_str(config).unwrap();
    let config: BrickConfig = BrickConfig::new(
        String::from("hi"),
        vec![Action::InsertFile(InsertFileAction {
            common: Common::default(),
            if_file_exists: FileExistsAction::Append,
        })],
    );
    assert_eq!(config_parsed, config);
}

#[test]
fn test_insert_file() {
    init_logger();

    let brick = Brick::try_from(brick_dir("insert_with_config")).unwrap();
    debug!("{:?}", brick);

    let ctx = ActionContext { dry_run: false };
    let tmpdir = tempfile::tempdir().unwrap();
    brick.execute(&ctx, tmpdir.path()).unwrap();
    assert!(tmpdir.path().join("TEST_A").exists());
    assert!(!tmpdir.path().join("TEST_B").exists());
    assert!(!tmpdir.path().join("brick.toml").exists());
}

#[test]
fn test_without_config() {
    init_logger();

    let brick = Brick::try_from(brick_dir("insert_no_config")).unwrap();
    debug!("{:?}", brick);

    assert_eq!(1, brick.config().actions().len());

    let ctx = ActionContext { dry_run: false };
    let tmpdir = tempfile::tempdir().unwrap();
    brick.execute(&ctx, tmpdir.path()).unwrap();
    assert!(tmpdir.path().join("TEST_B").exists());
}

#[test]
fn test_modify_append() {
    init_logger();

    let brick = Brick::try_from(brick_dir("modify_append")).unwrap();

    let tmpdir = tempfile::tempdir().unwrap();
    add_test_data(tmpdir.path(), "Test.toml");
    let ctx = ActionContext { dry_run: false };

    brick.execute(&ctx, tmpdir.path()).unwrap();
    let res_content = file_content(&tmpdir.path().join("Test.toml"));
    debug!("{}", res_content);
    assert!(res_content.contains("[dependencies]\nserde = \"1\"\n"))
}

#[test]
fn test_modify_prepend() {
    init_logger();

    let brick = Brick::try_from(brick_dir("modify_prepend")).unwrap();

    let tmpdir = tempfile::tempdir().unwrap();
    add_test_data(tmpdir.path(), "Test.toml");
    let ctx = ActionContext { dry_run: false };

    brick.execute(&ctx, tmpdir.path()).unwrap();
    let res_content = file_content(&tmpdir.path().join("Test.toml"));
    debug!("{}", res_content);
    assert!(res_content.contains("serde = \"1\"\n[dependencies]"))
}

#[test]
fn test_modify_replace() {
    init_logger();

    let brick = Brick::try_from(brick_dir("modify_replace")).unwrap();

    let tmpdir = tempfile::tempdir().unwrap();
    add_test_data(tmpdir.path(), "Test.toml");
    let ctx = ActionContext { dry_run: false };

    brick.execute(&ctx, tmpdir.path()).unwrap();
    let res_content = file_content(&tmpdir.path().join("Test.toml"));
    debug!("{}", res_content);
    assert!(!res_content.contains("[dependencies]"));
    assert!(res_content.contains("[dev-dependencies]"));
}

#[test]
fn test_command() {
    init_logger();

    let brick = Brick::try_from(brick_dir("run_command")).unwrap();

    let tmpdir = tempfile::tempdir().unwrap();
    let ctx = ActionContext { dry_run: false };

    brick.execute(&ctx, tmpdir.path()).unwrap();
    assert!(tmpdir.path().join("test.txt").exists());
}
