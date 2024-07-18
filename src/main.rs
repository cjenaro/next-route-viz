use clap::{Arg, Command};
use dialoguer::{theme::ColorfulTheme, Select};
use std::collections::HashMap;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug)]
struct RouteNode {
    name: String,
    children: HashMap<String, RouteNode>,
}

impl RouteNode {
    fn new(name: &str) -> Self {
        RouteNode {
            name: name.to_string(),
            children: HashMap::new(),
        }
    }

    fn add_route(&mut self, parts: &[String]) {
        if parts.is_empty() {
            return;
        }

        let first = parts[0].replace(".page", "").replace(".route", "");
        let first = first.split('.').next().unwrap_or(&first).to_string(); // Remove file extension
        let rest = &parts[1..];

        let child = self
            .children
            .entry(first.clone())
            .or_insert_with(|| RouteNode::new(&first));
        child.add_route(rest);
    }

    fn print(&self, prefix: &str, is_last: bool) {
        println!(
            "{}{}{}",
            prefix,
            if is_last { "└── " } else { "├── " },
            self.name
        );
        let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });

        let mut children_iter = self.children.values().peekable();
        while let Some(child) = children_iter.next() {
            child.print(&new_prefix, children_iter.peek().is_none());
        }
    }
}

fn main() {
    let matches = Command::new("nextjs_crawler")
        .version("1.0")
        .about("Crawls a Next.js repo and prints out all routes")
        .arg(
            Arg::new("path")
                .help("Specifies the path to the Next.js repo")
                .short('p')
                .long("path"),
        )
        .get_matches();

    let default_path = &".".to_string();
    let repo_path = matches.get_one::<String>("path").unwrap_or(default_path);

    let router_options = vec!["app", "pages"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Which router are you using?")
        .default(0)
        .items(&router_options)
        .interact()
        .unwrap();

    let router = router_options[selection];

    let mut root = RouteNode::new("root");

    match router {
        "app" => crawl_app_router(repo_path, &mut root),
        "pages" => crawl_pages_router(repo_path, &mut root),
        _ => eprintln!("Invalid selection. Please choose 'app' or 'pages'."),
    }

    root.print("", true);
}

fn parse_route_name(relative_path: &Path) -> Vec<String> {
    relative_path
        .components()
        .filter_map(|c| c.as_os_str().to_str().map(|s| s.to_string()))
        .map(|s| s.replace(".page", "").replace(".route", ""))
        .collect()
}

fn crawl_app_router(repo_path: &str, root: &mut RouteNode) {
    let app_path = Path::new(repo_path).join("app");
    if !app_path.exists() {
        eprintln!("The 'app' folder was not found in the specified path.");
        return;
    }

    for entry in WalkDir::new(app_path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry.path();
            if path.file_name().unwrap() == "page.js" || path.file_name().unwrap() == "route.js" {
                let relative_path = path.strip_prefix(repo_path).unwrap();
                let parts: Vec<String> = parse_route_name(relative_path);
                root.add_route(&parts);
            }
        }
    }
}

fn crawl_pages_router(repo_path: &str, root: &mut RouteNode) {
    let pages_path = Path::new(repo_path).join("pages");
    if !pages_path.exists() {
        eprintln!("The 'pages' folder was not found in the specified path.");
        return;
    }

    for entry in WalkDir::new(pages_path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry.path();
            if let Some(extension) = path.extension() {
                if extension == "js" || extension == "tsx" || extension == "jsx" {
                    let relative_path = path.strip_prefix(repo_path).unwrap();
                    let parts: Vec<String> = parse_route_name(relative_path);
                    root.add_route(&parts);
                }
            }
        }
    }
}
