fn main() -> anyhow::Result<()>
{
    let input_fnames = [
        "Aug 2017.pdf",
        "Sep 2017.pdf",
        "Oct 2017.pdf",
        "Nov 2017.pdf",
        "Dec 2017.pdf",
        "Jun 2017.pdf",
        "Jul 2017.pdf",
        "Nov 2020.pdf",
        "Dec 2020.pdf",
        "Jan 2021.pdf",
        "Feb 2021.pdf",
        "Mar 2021.pdf",
    ];

    let fmts = [
        "%B %d %Y.pdf",
        "%B %Y.pdf",
    ];
    for fname in &input_fnames {

        for orig_fmt in &fmts {
            if let Ok(date) = chrono::NaiveDate::parse_from_str(&fname, orig_fmt) {
                let new_fname = format!("{}", date.format("%Y.%m.%d.pdf"));
                println!("{} --> {}", fname, new_fname);
            }
        }
    }
    Ok(())
}
