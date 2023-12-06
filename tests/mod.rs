mod grouping;
mod others;
mod radicals;
mod scripted;

macro_rules! snap_test {
    ($input:expr $(, name: $name:expr)?) => {
        use std::str::FromStr;
        let mut settings = insta::Settings::clone_current();
        let input = $input.expect("Failed to render MathMl.");

        let input = xmlem::Document::from_str(&input).expect(&format!("input: {} is not valid XML.", input)).to_string_pretty();

        let base_dir = env!("CARGO_MANIFEST_DIR");

        let path = std::path::Path::new(base_dir).join("tests/snapshots");
        settings.set_snapshot_path(path);
        settings.set_prepend_module_to_snapshot(false);

        settings.bind(|| {
            insta::assert_snapshot!($($name,)? input);
        });
    };
}

use snap_test;
