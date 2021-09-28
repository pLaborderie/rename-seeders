use std::io;
use git2::Repository;
use std::fs;

fn main() {
    println!("Enter path to repo:");
    let mut path = String::new();
    io::stdin().read_line(&mut path).unwrap();
    let path = path.trim();
    
    // Open the repository
    let repo = match Repository::open(path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open repository: {}", e),
    };

    let statuses = repo.statuses(None).unwrap();
    for entry in statuses.iter() {
        let status = entry.status();
        if status.is_wt_modified() {
            let file_path = entry.path();
            if file_path.is_some() {
                let file_path = format!("{}/{}", path, file_path.unwrap());
                println!("Previous file path: {}", file_path);
                let new_path = get_updated_path(&file_path);
                println!("New file path: {}", new_path);
                fs::rename(file_path, new_path).unwrap();
            }
        }
    }
}

fn get_updated_path(path: &str) -> String {
    if path.contains("updated") {
        let index = path.find("updated").unwrap();
        let number = path.get(index + 7..index + 8).unwrap();
        println!("number: {}", number);
        let is_number = number.parse::<i32>().is_ok();
        if is_number {
            let old_updated = format!("updated{}", number);
            let new_updated = format!("updated{}", (number.parse::<i32>().unwrap() + 1).to_string());
            path.replace(&old_updated, &new_updated)
        } else {
            path.replace("updated", "updated2")
        }
    } else {
        path.replace(".", "updated.")
    }
}
