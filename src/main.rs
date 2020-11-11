mod opts;
mod templates;

use std::path::{Path, PathBuf};
use std::{fs, io};

use pulldown_cmark::{html, Parser};

use structopt::StructOpt;

use opts::Horn;
use templates::Ramen;

fn main() {
    let _: Horn = StructOpt::from_args();

    if fs::read_dir("./build").is_err() {
        fs::create_dir("./build").unwrap();
    }

    let mut ramens: Vec<(PathBuf, Ramen)> = Vec::new();

    fs::read_dir("./source")
        .expect("Please put your source files in `source` directory of current directory.")
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if !path.is_dir() {
                if path.extension().filter(|&ext| ext == "md").is_some() {
                    let source = fs::read_to_string(&path).ok()?;
                    Some((path, source))
                } else {
                    fs::copy(&path, Path::new("./build").join(path.file_name().unwrap())).unwrap();
                    None
                }
            } else {
                None
            }
        })
        .map(|(path, source)| {
            let mut source = source.splitn(3, "---");
            source.next();

            let front_matter = source.next().unwrap();
            let md = source.next().unwrap();

            let ramen: Ramen = serde_yaml::from_str(front_matter)
                .expect("Please fill in front matter with correct fields.");
            ramens.push((path.with_extension("html"), ramen.clone()));

            let parser = Parser::new(&md);
            let mut html = String::new();
            html::push_html(&mut html, parser);
            (path, templates::ramen(ramen, &html))
        })
        .map(|(path, html)| {
            fs::write(
                Path::new("./build")
                    .join(path.file_name().unwrap())
                    .with_extension("html"),
                html,
            )
        })
        .collect::<io::Result<Vec<_>>>()
        .expect("Failed to write build output.");

    fs::write(
        Path::new("./build/index.html"),
        templates::home(ramens),
    ).unwrap();
}
