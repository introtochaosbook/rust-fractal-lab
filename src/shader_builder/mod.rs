use std::borrow::Cow;
use std::fs;
use std::path::PathBuf;

use once_cell::sync::Lazy;
use regex::{Captures, Regex};

static INCLUDE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new("//\\s*<inject:(?P<file>[^>]+)>").unwrap());

pub fn build_shader(input: &str) -> Cow<str> {
    let ret = INCLUDE_REGEX.replace_all(input, |caps: &Captures| {
        let file = caps.name("file").unwrap();
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let shader_root = root.join("src").join("shader_builder").join("includes");
        let shader_file = shader_root.join(file.as_str());

        fs::read_to_string(shader_file).unwrap()
    });

    ret.lines().enumerate().for_each(|(i, line)| {
        eprintln!("{:>3}: {}", i + 1, line);
    });

    ret
}
