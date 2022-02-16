slint::slint!(import { MainWindow } from "src/ui/main.slint";);
slint::slint!(import { NewsData } from "src/ui/newsdata.slint";);

//use std::collections::HashMap;
use pt_launcher::{load_img, parse_img};
use serde::Deserialize;
use slint::SharedString;
use std::time::Duration;

#[derive(Debug, Deserialize)]
struct DiscussionsIncludesAttributes {
    featuredImage : String,
}

#[derive(Debug, Deserialize)]
struct DiscussionsIncludes {
    id : String,
    attributes : DiscussionsIncludesAttributes,
}

#[derive(Debug, Deserialize)]
struct DiscussionsDataAttributes {
    title : String,
    shareUrl : String,
}

#[derive(Debug, Deserialize)]
struct DiscussionsData {
    id : String,
    attributes : DiscussionsDataAttributes,
}

#[derive(Debug, Deserialize)]
struct Discussions {
    data : Vec<DiscussionsData>,
    included : Vec<DiscussionsIncludes>
}

struct Discussion {
    title : String,
    url : String,
    image_buffer : Vec<u8>,
    width: u32,
    height: u32
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let main_window = MainWindow::new();
    let handle_weak = main_window.as_weak();
    let mut discussions: Vec<Discussion> = vec![];

    main_window.on_url_redirect({
        move |url| {
            open::that(url.as_str()).unwrap();
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

        let disc_resp = reqwest::get("https://project-trains.pl/api/discussions?include=blogMeta&filter[q]=is:blog+tag:devlogi&sort=-createdAt").await;
        let disc_json = disc_resp.unwrap().json::<Discussions>().await.unwrap();
        println!("{:#?}", disc_json);

        for i in 0..disc_json.data.len() {
            let img_url : &str = &disc_json.included[i].attributes.featuredImage;
            let (img_buffer, w, h) = load_img(img_url).await;
            // let (img_buffer, w, h) = load_img("https://picsum.photos/800/450").await;

            let disc = Discussion {
                title: disc_json.data[i].attributes.title.clone(),
                url: disc_json.data[i].attributes.shareUrl.clone(),
                image_buffer: img_buffer,
                width: w,
                height: h
            };

            discussions.push(disc);
        }

        update_discussions(handle_weak.clone(), discussions);

        let (img, w, h) = load_img("https://picsum.photos/800/450").await;
        update_featured(handle_weak.clone(), img, w, h);
    });

    main_window.run();
    Ok(())
}

fn update_discussions(handle: slint::Weak<MainWindow>, discussions: Vec<Discussion>) {
    handle.upgrade_in_event_loop(move |handle| {
        let mut news_data: Vec<NewsData> = vec![];

        for i in 0..discussions.len() {
            let news = NewsData {
                title: SharedString::from(discussions[i].title.clone()),
                cover: parse_img(discussions[i].image_buffer.to_vec(), discussions[i].width, discussions[i].height),
                url: SharedString::from(discussions[i].url.clone()),
            };

            news_data.push(news);
        }

        let news_model = std::rc::Rc::new(slint::VecModel::from(news_data));
        handle.set_news(slint::ModelRc::from(news_model.clone()));
    });
}

fn update_featured(handle: slint::Weak<MainWindow>, image: Vec<u8>, w: u32, h: u32) {
    handle.upgrade_in_event_loop(move |handle| {
        let img = parse_img(image, w, h);
        handle.set_featured(img);
    });
}