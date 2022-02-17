use crate::load_img;

const URL: &str = "https://project-trains.pl/api/discussions?include=blogMeta&filter[q]=is:blog+tag:devlogi&sort=-createdAt";

pub struct Post {
    pub title: String,
    pub url: String,
    pub excerpt: String,
    pub image_buffer: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

impl Post {
    fn new(
        title: Option<&str>,
        excerpt: Option<&str>,
        url: Option<&str>,
        image_buffer: Vec<u8>,
        width: usize,
        height: usize,
    ) -> Self {
        Post {
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

    pub async fn fetch() -> Vec<Post> {
        let res = tinyget::get(URL).send().unwrap();
        let json = json::parse(res.as_str().unwrap()).unwrap();
        let mut posts: Vec<Post> = vec![];

        for i in 0..json["data"].len() {
            let data = &json["data"][i]["attributes"];
            let included = &json["included"][i]["attributes"];
            let img_url: String = Post::get_string(included["featuredImage"].as_str());
            let (img_buffer, w, h) = load_img(img_url.as_str()).await;

            let post = Post::new(
                data["title"].as_str(),
                included["summary"].as_str(),
                data["shareUrl"].as_str(),
                img_buffer,
                w,
                h,
            );

            posts.push(post);
        }

        posts
    }
}
