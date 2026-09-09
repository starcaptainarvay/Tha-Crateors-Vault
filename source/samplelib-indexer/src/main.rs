use std::{collections::HashSet, fs::{self, DirEntry}, io::Result, path::Path, sync::LazyLock};

static IGNORE_LIST: LazyLock<HashSet<String>> = LazyLock::new(|| {
    let path = Path::new(".ignore");
    let contents = fs::read_to_string(path).unwrap_or("".into());

    HashSet::<String>::from_iter(contents.lines().map(String::from))
});

fn print_dir(entry: DirEntry, out: &mut String, level: usize) -> Result<()> {
    let fname = entry.file_name().into_string().unwrap_or_default();
    if fname.starts_with('.') || IGNORE_LIST.contains(fname.as_str()) { return Ok(()); }

    let ftype = entry.file_type()?;
    let indent = "  ".repeat(level);

    if ftype.is_dir() {
        out.push_str(format!("{}{}:\n", indent, fname).as_str());
        for item in fs::read_dir(entry.path())? {
            print_dir(item?, out, level + 1)?;
        }
    } else if ftype.is_file() {
        out.push_str(format!("{}- {}\n", indent, fname).as_str());
    }

    println!("Indexed {}", entry.path().display());

    Ok(())
}

fn main() -> Result<()> {
    let root = Path::new(".");

    let mut out = String::new();

    for entry in fs::read_dir(root)? {
        if print_dir(entry.unwrap(), &mut out, 0).is_ok() {
            continue;
        }
    }

    fs::write(root.join("manifest.yaml").as_path(), out)
}
