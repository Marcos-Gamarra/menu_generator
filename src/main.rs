use std::fs::{self, File};
use std::io::prelude::*;

fn main() {
    let mut output = File::create("output.html").unwrap();
    let mut styles = Vec::new();
    let mut components = Vec::new();
    let navbar = fs::read_to_string("components/navbar/navbar.html").unwrap();
    styles.push(String::from("components/navbar/navbar.css"));
    styles.push(String::from("output.css"));
    components.push(navbar);
    let head = generate_head(String::from("Title"), styles);
    let body = generate_body(components);

    output.write_all(head.as_bytes()).unwrap();
    output.write_all(body.as_bytes()).unwrap();
}

fn add_style(style_path: &str) -> String {
    let link = format!("<link rel=\"stylesheet\" href=\"{}\">", style_path);
    link
}

fn generate_head(title: String, styles: Vec<String>) -> String {
    let mut head = String::from("<head>");
    let title = format!("<title>{}</title>", title);
    let styles = styles
        .iter()
        .map(|style| add_style(style))
        .collect::<Vec<String>>();

    let styles = styles.join("");

    head = head + &title + &styles + "</head>";

    head
}

fn generate_body(components: Vec<String>) -> String {
    let mut body = String::from("<body>");
    let components = components.join("");
    body = body + &components + "</body>";
    body
}
