use std::fs;
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
            links.push((file_name.to_string(), link));
        }
    }

    // Sort links based on the leading number in the file name
    links.sort_by_key(|(file_name, _)| {
        file_name
            .split('_')
            .next()
            .and_then(|n| n.parse::<i32>().ok())
            .unwrap_or(0)
    });

    let mut file = fs::File::create(readme_path)?;
    writeln!(file, "# IBP Proposals\n")?;
    for (_, link) in links {
        writeln!(file, "{}", link)?;
    }

    Ok(())
}
