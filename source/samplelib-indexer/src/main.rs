use std::{collections::HashMap, fs::{self, DirEntry}, io::Result, path::Path, sync::LazyLock};

static IGNORE_LIST: LazyLock<HashMap<&'static str, bool>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert("index_to_manifest", true);
    map.insert("manifest.yaml", true);
    map.insert("source", true);
    map
});

fn print_dir(entry: DirEntry, out: &mut String, level: usize) -> Result<&mut String> {
    let dir = entry.path();
    let tab_space = "  ";
    let fname = entry.file_name().into_string().unwrap();
    let ftype = entry.file_type().unwrap();

    if !(fname.starts_with(".") || IGNORE_LIST.contains_key(fname.as_str())) {
        println!("{}", fname);
        if ftype.is_dir() || ftype.is_file() {
            let line = format!(
                "{}{}{}",
                tab_space.repeat(level),
                if level > 0 { "- "} else { "" },
                if ftype.is_dir() { format!("{}", fname) } else { fname }
            );

            out.push_str(&line);
            out.push_str("\n");
        }
    } else { return Ok(out) }

    for entry in fs::read_dir(dir)? {
        let dir = entry.unwrap();

        if print_dir(dir, out, level + 1).is_ok() {
            continue;
        }
    }

    Ok(out)
}

fn main() -> Result<()> {
    let root = Path::new(".");

    let mut out = String::new();

    for entry in fs::read_dir(root)? {
        if print_dir(entry.unwrap(), &mut out, 0).is_ok() {
            continue;
        }
        // println!("{}", dir.into_string().unwrap());
    }

    fs::write(root.join("manifest.yaml").as_path(), out)
}
