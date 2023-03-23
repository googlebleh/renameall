fn main() -> anyhow::Result<()>
{
    let fmts = [
        "%B %d %Y.pdf",
        "%B %Y.pdf",
    ];
    for entry in std::fs::read_dir(".")? {
        let entry = entry?;
        let fname = String::from(entry.file_name().to_str().unwrap());

        for orig_fmt in &fmts {
            if let Ok(date) = chrono::NaiveDate::parse_from_str(&fname, orig_fmt) {
                let new_fname = format!("{}", date.format("%Y.%m.%d.pdf"));
                println!("{} --> {}", fname, new_fname);
                std::fs::rename(&fname, new_fname);
            }
        }
    }
    Ok(())
}
