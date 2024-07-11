use reqwest::header::CONTENT_TYPE;
use serde_json::json;

use crate::anilist::objects::Response;

pub(crate) async fn anime_info(title: &str) -> Response {
    println!("title is: {:?}", title);
    let title = anime_name(title);
    println!("title after is: {:?}", title);
    let query = json!({"query": r#"
        {
    Media(type: ANIME, search: "My-Hero-Academia-Season-7") {
        coverImage {
            extraLarge
        }
        title {
            english
        }
        season
        seasonYear
        episodes
        status
        nextAiringEpisode {
            airingAt
            episode
        }
    }
}"#});

    // let variables = json!({"search": title});
    // let body = json!({"query": query, "variables": variables});
    println!("{:?}", query);
    // let body = json!({"query": query});
    reqwest::Client::new()
        .post("https://graphql.anilist.co")
        .json(&query)
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await
        .expect("Send failed")
        .json::<Response>()
        .await
        .expect("Parse failed")
}

fn anime_name(title: &str) -> &str {
    if let Some(idx) = title.rfind('\\') {
        title.split_at(idx).1
    } else {
        title
    }
}
