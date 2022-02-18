use json::JsonValue;
use crate::load_img;

const URL: &str = "https://project-trains.pl/api/discussions?include=blogMeta&filter[q]=is:blog+tag:devlogi&sort=-createdAt";

pub struct Post {
    pub title: String,
    pub url: String,
    pub excerpt: String,
    pub image_url: String,
    pub image_buffer: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

impl Post {
    fn new(
        title: Option<&str>,
        excerpt: Option<&str>,
        url: Option<&str>,
        image_url: Option<&str>,
        image_buffer: Vec<u8>,
        width: usize,
        height: usize,
    ) -> Self {
        Post {
            title: Self::get_string(title),
            excerpt: Self::get_string(excerpt),
            url: Self::get_string(url),
            image_url: Self::get_string(image_url),
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

    pub fn fetch() -> JsonValue {
        let res = tinyget::get(URL).send().unwrap();
        let json = json::parse(res.as_str().unwrap()).unwrap();
        json
    }

    pub fn parse_posts(json: &JsonValue) -> Vec<Post> {
        let mut posts: Vec<Post> = vec![];

        for i in 0..json["data"].len() {
            let data = &json["data"][i]["attributes"];
            let included = &json["included"][i]["attributes"];

            let post = Post::new(
                data["title"].as_str(),
                included["summary"].as_str(),
                data["shareUrl"].as_str(),
                included["featuredImage"].as_str(),
                vec![0, 0],
                1,
                1,
            );

            posts.push(post);
        }

        posts
    }

    pub async fn load_feature_image(posts: Vec<Post>) -> Vec<Post> {
        let mut posts = posts;
        let img_url: String = posts[0].image_url.clone();
        let (img_buffer, w, h) = load_img(img_url.as_str()).await;

        posts[0].image_buffer = img_buffer;
        posts[0].width = w;
        posts[0].height = h;

        posts
    }

    pub async fn load_images(posts: Vec<Post>) -> Vec<Post> {
        let mut posts = posts;
        for i in 0..posts.len() {
            let img_url: String = posts[i].image_url.clone();
            let (img_buffer, w, h) = load_img(img_url.as_str()).await;

            posts[i].image_buffer = img_buffer;
            posts[i].width = w;
            posts[i].height = h;
        }

        posts
    }
}
