slint::slint!(import { MainWindow } from "src/ui/main.slint";);
slint::slint!(import { NewsData } from "src/ui/newsdata.slint";);

use pt_launcher::{parse_img, post::Post};
use slint::SharedString;

#[tokio::main]
async fn main() {
    let main_window = MainWindow::new();
    let handle_weak = main_window.as_weak();

    main_window.on_url_redirect(move |url| {
        open::that(url.as_str()).unwrap();
    });

    main_window.on_close(move || {
        std::process::exit(0);
    });

    tokio::spawn(async move {
        let posts = Post::fetch().await;
        update_discussions(handle_weak.clone(), posts);
    });

    main_window.run();
}

fn update_discussions(handle: slint::Weak<MainWindow>, posts: Vec<Post>) {
    handle.upgrade_in_event_loop(move |handle| {
        let mut news_data: Vec<NewsData> = vec![];

        for i in 0..posts.len() {
            let news = NewsData {
                title: SharedString::from(posts[i].title.clone()),
                excerpt: SharedString::from(posts[i].excerpt.clone()),
                cover: parse_img(
                    posts[i].image_buffer.to_vec(),
                    posts[i].width,
                    posts[i].height,
                ),
                url: SharedString::from(posts[i].url.clone()),
            };

            news_data.push(news);
        }

        let news_model = std::rc::Rc::new(slint::VecModel::from(news_data));
        handle.set_news(slint::ModelRc::from(news_model.clone()));
    });
}
