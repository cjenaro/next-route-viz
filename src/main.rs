use clap::{Arg, Command};
use dialoguer::{theme::ColorfulTheme, Select};
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    let matches = Command::new("nextjs_crawler")
        .version("1.0")
        .about("Crawls a Next.js repo and prints out all routes")
        .arg(
            Arg::new("path")
                .help("Specifies the path to the Next.js repo")
                .short('p')
                .long("path")
        )
        .get_matches();

    let default_path = ".".to_string();
    let repo_path = matches.get_one::<String>("path").unwrap_or(&default_path);

    let router_options = vec!["app", "pages"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Which router are you using?")
        .default(0)
        .items(&router_options)
        .interact()
        .unwrap();

    let router = router_options[selection];

    match router {
        "app" => crawl_app_router(repo_path),
        "pages" => crawl_pages_router(repo_path),
        _ => eprintln!("Invalid selection. Please choose 'app' or 'pages'."),
    }
}

fn crawl_app_router(repo_path: &str) {
    let app_path = Path::new(repo_path).join("app");
    if !app_path.exists() {
        eprintln!("The 'app' folder was not found in the specified path.");
        return;
    }

    for entry in WalkDir::new(app_path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry.path();
            if path.file_name().unwrap() == "page.js" || path.file_name().unwrap() == "route.js" {
                println!("{}", path.display());
            }
        }
    }
}

fn crawl_pages_router(repo_path: &str) {
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
                    println!("{}", path.display());
                }
            }
        }
    }
}

