use std::collections::HashMap;
use std::path::PathBuf;

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Ramen {
    pub name: String,
    pub image: String,
    pub location: String,
}

pub fn ramen(ramen: Ramen, content: &str) -> String {
    html(
        "ja",
        &head(&format!("{} | ラーログ", ramen.name), vec![]),
        &body(&format!(
            r#"
        <h1>{}</h1>
        <img class="ramen-thumbnail" src="{}">
        <div>{}</div>
        {}"#,
            ramen.name, ramen.image, ramen.location, content
        )),
    )
}

pub fn home(ramens: Vec<(PathBuf, Ramen)>) -> String {
    let ramens_by_location: HashMap<String, Vec<(PathBuf, Ramen)>> =
        ramens
            .into_iter()
            .fold(HashMap::new(), |mut map, (path, ramen)| {
                map.entry(ramen.location.clone())
                    .or_default()
                    .push((path, ramen));
                map
            });

    let rendered = ramens_by_location
        .keys()
        .map(|location| {
            format!(
                r#"<h2>{}</h2>
                <div class="ramens-container">
                <div class="ramens">{}</div>
                </div>"#,
                location,
                ramens_by_location[location]
                    .iter()
                    .map(|(path, ramen)| {
                        format!(
                            r#"<a href="./{}">
                                <img class="ramen-thumbnail" src="{}">
                                <div>
                                {}
                                </div>
                            </a>"#,
                            path.file_name().unwrap().to_str().unwrap(),
                            &ramen.image,
                            &ramen.name
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    html(
        "ja",
        &head("ラーログ", vec![]),
        &body(&format!(r#"{}"#, rendered)),
    )
}

fn html(lang: &str, head: &str, body: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="{}">
{}
{}
</html>"#,
        lang, head, body
    )
}

fn head(title: &str, links: Vec<(&str, &str)>) -> String {
    format!(
        r#"<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>{}</title>
    <link rel="stylesheet" href="./style.css">
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bulma@0.9.1/css/bulma.min.css">
{}
</head>"#,
        title,
        links
            .iter()
            .map(|(rel, href)| format!(r#"    <link rel="{}" href="{}">"#, rel, href))
            .collect::<Vec<_>>()
            .join("    \n")
    )
}

fn body(content: &str) -> String {
    format!(
        r#"<body style="overflow-x: hidden">
{}
    <section class="section">
        <div class="container is-max-desktop">
            <div class="content">
{}
            </div>
        </div>
    </section>
</body>"#,
        navbar(),
        content
    )
}

fn navbar() -> String {
    r#"    <nav class="logobar">
        <div class="container is-justify-content-center">
            <a class="logo" href="./index.html">
                <img src="./logo.png" height="52" alt="ラーログ">
            </a>
        </div>
    </nav>
    "#
    .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generate_html() {
        assert_eq!(
            html(
                "ja-jp",
                &head(
                    "ラーログ",
                    vec![(
                        "stylesheet",
                        "https://cdn.jsdelivr.net/npm/bulma@0.9.1/css/bulma.min.css"
                    )]
                ),
                "<body>body</body>"
            )
            .split_whitespace()
            .collect::<String>(),
            r#"
<!DOCTYPE html>
<html lang="ja-jp">
    <head>
        <meta charset="utf-8">
        <meta name="viewport" content="width=device-width, initial-scale=1">
        <title>ra-log</title>
        <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bulma@0.9.1/css/bulma.min.css">
    </head>
    <body>
        body
    </body>
</html>
            "#
            .split_whitespace()
            .collect::<String>(),
        )
    }
}
