use std::{
    path::PathBuf,
    vec,
};

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

fn init_logger() {
    let _ = env_logger::builder()
        // Include all events in tests
        .filter_level(log::LevelFilter::max())
        // Ensure events are captured by `cargo test`
        .is_test(true)
        // Ignore errors initializing the logger if tests race to configure it
        .try_init();
}

fn test_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests")
}

fn brick_dir(brick: &str) -> PathBuf {
    test_dir().join("bricks/").join(brick)
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
