use std::fs;
use std::path::Path;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let proposals_dir = "./proposals";
    let readme_path = "./README.md";
    let mut links = Vec::new();

    for entry in fs::read_dir(proposals_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            let link = format!("- [{}]({}/{})", file_name, proposals_dir, file_name);
            links.push(link);
        }
    }

    let mut file = fs::File::create(readme_path)?;
    writeln!(file, "# IBP Proposals\n")?;
    for link in links {
        writeln!(file, "{}", link)?;
    }

    Ok(())
}
