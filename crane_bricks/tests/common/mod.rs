use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

pub fn init_logger() {
    let _ = env_logger::builder()
        // Include all events in tests
        .filter_level(log::LevelFilter::max())
        // Ensure events are captured by `cargo test`
        .is_test(true)
        // Ignore errors initializing the logger if tests race to configure it
        .try_init();
}

pub fn test_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests")
}

pub fn brick_dir(brick: &str) -> PathBuf {
    test_dir().join("bricks/").join(brick)
}

/// Add a file from the tests/data dir to the temp dir
pub fn add_test_data(temp: &Path, file: &str) {
    let mut data_file = File::options()
        .read(true)
        .open(test_dir().join("data/").join(file))
        .unwrap();
    let mut content = String::new();
    data_file.read_to_string(&mut content).unwrap();

    let mut target_file = File::create(temp.join(file)).unwrap();
    target_file.write(content.as_bytes()).unwrap();
}

pub fn file_content(path: &Path) -> String {
    let mut file = File::options().read(true).open(path).unwrap();
    let mut output = String::new();
    file.read_to_string(&mut output).unwrap();
    output
}
