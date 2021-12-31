sixtyfps::sixtyfps!(import { MainWindow } from "src/ui/main.60";);

use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct DiscussionsIncludesAttributes {
    contentType : String,
    contentHtml : String,
}

#[derive(Debug, Deserialize)]
struct DiscussionsIncludes {
    id : String,
    //attributes : HashMap<String, String>
    //attributes : DiscussionsIncludesAttributes
}

#[derive(Debug, Deserialize)]
struct DiscussionsDataAttributes {
    title : String,
    shareUrl : String,
}

#[derive(Debug, Deserialize)]
struct DiscussionsData {
    id : String,
    attributes : DiscussionsDataAttributes
}

#[derive(Debug, Deserialize)]
struct Discussions {
    data : Vec<DiscussionsData>,
    included : Vec<DiscussionsIncludes>
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://project-trains.pl/api/discussions?include=user%2ClastPostedUser%2CfirstPost%2Ctags%2Ctags.parent&filter%5Btag%5D=devlogi&sort=-createdAt&page%5Boffset%5D=0")
        .await?;

    let resp_json = resp.json::<Discussions>().await?;

    println!("{:#?}", resp_json);

    let main_window = MainWindow::new();

    main_window.on_blog_redirect({
        move || { open::that("https://project-trains.pl/").unwrap(); }
    });

    main_window.on_close({
        move || { std::process::exit(0); }
    });

    main_window.run();
    Ok(())
}