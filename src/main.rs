fn main() -> std::io::Result<()> {
    let path = std::env::current_dir()?;
    rstree::get_tree(&path)
}
