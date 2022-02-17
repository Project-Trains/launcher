slint::slint!(import { MainWindow } from "src/ui/main.slint";);
slint::slint!(import { NewsData } from "src/ui/newsdata.slint";);

use pt_launcher::{load_img, parse_img};
use slint::SharedString;
use std::time::Duration;

struct Discussion {
    title: String,
    url: String,
    excerpt: String,
    image_buffer: Vec<u8>,
    width: usize,
    height: usize,
}

impl Discussion {
    fn new(
        title: Option<&str>,
        excerpt: Option<&str>,
        url: Option<&str>,
        image_buffer: Vec<u8>,
        width: usize,
        height: usize,
    ) -> Self {
        Discussion {
            title: Self::get_string(title),
            excerpt: Self::get_string(excerpt),
            url: Self::get_string(url),
            image_buffer,
            width,
            height,
        }
    }

    pub fn get_string(option: Option<&str>) -> String {
        match option {
            Some(v) => v.to_string(),
            None => String::new(),
        }
    }
}

#[tokio::main]
async fn main() {
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

        let disc_resp = tinyget::get("https://project-trains.pl/api/discussions?include=blogMeta&filter[q]=is:blog+tag:devlogi&sort=-createdAt")
            .send()
            .unwrap();

        let disc_json = json::parse(disc_resp.as_str().unwrap()).unwrap();
        //println!("{:#?}", disc_json);

        for i in 0..disc_json["data"].len() {
            let data = &disc_json["data"][i]["attributes"];
            let included = &disc_json["included"][i]["attributes"];
            
            let img_url: String = Discussion::get_string(included["featuredImage"].as_str());
            let (img_buffer, w, h) = load_img(img_url.as_str()).await;
            //let (img_buffer, w, h) = load_img("https://picsum.photos/800/450").await;

            let disc = Discussion::new(
                data["title"].as_str(),
                included["summary"].as_str(),
                data["shareUrl"].as_str(),
                img_buffer,
                w,
                h,
            );

            discussions.push(disc);
        }

        update_discussions(handle_weak.clone(), discussions);
    });

    main_window.run();
}

fn update_discussions(handle: slint::Weak<MainWindow>, discussions: Vec<Discussion>) {
    handle.upgrade_in_event_loop(move |handle| {
        let mut news_data: Vec<NewsData> = vec![];

        for i in 0..discussions.len() {
            let news = NewsData {
                title: SharedString::from(discussions[i].title.clone()),
                excerpt: SharedString::from(discussions[i].excerpt.clone()),
                cover: parse_img(discussions[i].image_buffer.to_vec(), discussions[i].width, discussions[i].height),
                url: SharedString::from(discussions[i].url.clone())
            };

            news_data.push(news);
        }

        let news_model = std::rc::Rc::new(slint::VecModel::from(news_data));
        handle.set_news(slint::ModelRc::from(news_model.clone()));
    });
}
