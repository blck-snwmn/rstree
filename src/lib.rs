use std::fs;
use std::path::Path;

pub fn get_tree(path: &Path) -> std::io::Result<()> {
    if !path.is_dir() {
        return Ok(());
    }
    let paths = fs::read_dir(path)?;
    for path in paths {
        path.map(|e| println!("{:?}", e.path()))?;
    }
    Ok(())
}
