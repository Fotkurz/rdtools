use std::fs;

use notify_rust::Notification;


fn main() {
    let contents = fs::read_to_string("repos.txt").expect("Failed to read repos.txt");

    for line in contents.lines() {
        println!("{}", line);
    }
    
    let url = format!("https://api.github.com/repos/{}/pulls/{}", "Fotkurz/Fotkurz", "150");
    
    let resp = reqwest::blocking::Client::new()
        .get(url)
        .bearer_auth("")
        .send()
        .expect("Failed to request github api");

    println!("{:#?}", resp);
    
    let _ = Notification::new()
        .summary("Github")
        .body("Your pull-request was approved.")
        .show();

}
