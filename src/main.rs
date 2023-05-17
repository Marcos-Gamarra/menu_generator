use std::io::prelude::*;

fn main() {
    let menu_id = "test_menu".to_string();
    let categories = vec!["one".to_string(), "two".to_string(), "three".to_string()];
    let title = "Test Menu".to_string();
    generate_menu(menu_id, categories, title);
}

fn generate_menu(menu_id: String, categories: Vec<String>, title: String) {
    if let Err(e) = std::fs::create_dir_all("menus/".to_string() + &menu_id) {
        println!("Error creating directory: {}", e);
    }

    generate_categories_files(categories, menu_id, title);
}

fn generate_categories_files(categories: Vec<String>, menu_id: String, title: String) {
    let navbar = generate_navbar(&categories);
    let head = generate_head(&title);
    let content = "content".to_string();
    for category in categories {
        if let Ok(mut file) = std::fs::File::create(format!("menus/{menu_id}/{category}.html")) {
            let page = generate_page(&navbar, &head, &content);
            file.write_all(page.as_bytes()).unwrap();
        }
    }
}

fn generate_head(title: &str) -> String {
    format!(
        "<head>
            <title>{title}</title>
            <link rel=\"stylesheet\" href=\"../../style.css\">
        </head>"
    )
}

fn generate_navbar(categories: &Vec<String>) -> String {
    let mut navbar = "<nav class=\"navbar\">\n".to_string();
    for category in categories {
        navbar.push_str(&generate_link(category));
    }
    navbar.push_str("</nav>");
    navbar
}

fn generate_link(category: &str) -> String {
    format!("<a href=\"{category}.html\"><button class=\"nav-button\">{category}</button></a>")
}

fn generate_page(navbar: &str, head: &str, content: &str) -> String {
    format!(
        "<!DOCTYPE html>
        <html lang=\"en\">
        {head}
        <body>
            {navbar}
            {content}
        </body>
        </html>"
    )
}
