sixtyfps::sixtyfps!(import { MainWindow } from "src/ui/main.60";);
sixtyfps::sixtyfps!(import { NewsData } from "src/ui/newsdata.60";);

//use std::collections::HashMap;
use pt_launcher::{load_img, parse_img};
use serde::Deserialize;
use sixtyfps::SharedString;
use std::time::Duration;

// #[derive(Debug, Deserialize)]
// struct DiscussionsIncludesAttributes {
//     contentHtml : String,
// }

// #[derive(Debug, Deserialize)]
// struct DiscussionsIncludes {
//     id : String,
//     attributes : DiscussionsIncludesAttributes,
// }

// #[derive(Debug, Deserialize)]
// struct DiscussionsDataRelationshipsFirstPostData {
//     id : String
// }

// #[derive(Debug, Deserialize)]
// struct DiscussionsDataRelationshipsFirstPost {
//     data : DiscussionsDataRelationshipsFirstPostData
// }

// #[derive(Debug, Deserialize)]
// struct DiscussionsDataRelationships {
//     firstPost : DiscussionsDataRelationshipsFirstPost
// }

// #[derive(Debug, Deserialize)]
// struct DiscussionsDataAttributes {
//     title : String,
//     shareUrl : String
// }

// #[derive(Debug, Deserialize)]
// struct DiscussionsData {
//     id : String,
//     attributes : DiscussionsDataAttributes,
//     relationships : DiscussionsDataRelationships
// }

// #[derive(Debug, Deserialize)]
// struct Discussions {
//     data : Vec<DiscussionsData>,
//     included : Vec<DiscussionsIncludes>
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let main_window = MainWindow::new();
    let handle_weak = main_window.as_weak();

    main_window.on_blog_redirect({
        move || {
            open::that("https://project-trains.pl/").unwrap();
        }
    });

    main_window.on_close({
        move || {
            std::process::exit(0);
        }
    });

    tokio::spawn(async move {
        // delay for testing
        std::thread::sleep(Duration::new(2, 0));

        let (img, w, h) = load_img("https://picsum.photos/800/450").await;
        update_featured(handle_weak.clone(), img, w, h);       
    });

    // let resp = reqwest::get("https://project-trains.pl/api/discussions?include=firstPost&filter%5Btag%5D=devlogi&sort=-createdAt&page%5Boffset%5D=0")
    // .await?;
    // let resp_json = resp.json::<Discussions>().await?;
    let mut discussions: Vec<NewsData> = vec![];
    // println!("{:#?}", resp_json);

    // for i in 0..resp_json.data.len() {
    // let mut disc = NewsData {
    // title: SharedString::from(resp_json.data[i].attributes.title.clone()),
    //image: SharedString::from("../assets/devlog.jpg"),
    // };

    // discussions.push(disc);
    // }

    let discussions_model = std::rc::Rc::new(sixtyfps::VecModel::from(discussions));

    main_window.set_news(sixtyfps::ModelHandle::new(discussions_model.clone()));
    main_window.run();

    Ok(())
}

fn update_featured(handle: sixtyfps::Weak<MainWindow>, image: Vec<u8>, w: u32, h: u32) {
    handle.upgrade_in_event_loop(move |handle| {
        let img = parse_img(image, w, h);
        handle.set_image(img);
    });
}