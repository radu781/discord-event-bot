use crate::anilist::objects::Response;
use reqwest::header::CONTENT_TYPE;
use serde_json::json;
use std::path;

pub(crate) async fn anime_info(title: &str) -> Response {
    println!("title is: {:?}", title);
    let title = anime_name(title);
    println!("title after is: {:?}", title);
    let query = json!({"query": format!(r#"
            {{
    Media(type: ANIME, search: "{title}") {{
        coverImage {{
            extraLarge
        }}
        title {{
            english
        }}
        season
        seasonYear
        episodes
        status
        nextAiringEpisode {{
            airingAt
            episode
        }}
    }}
}}"#)});

    let variables = json!({"search": title});
    let body = json!({"query": query, "variables": variables});
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

fn anime_name(title: &str) -> String {
    let ignore_dirs = ["season"];
    title
        .to_owned()
        .split(path::MAIN_SEPARATOR_STR)
        .collect::<Vec<&str>>()
        .into_iter()
        .rev()
        .find(|s| {
            !ignore_dirs
                .iter()
                .any(|dir| s.to_lowercase().contains(&dir.to_lowercase()))
        })
        .unwrap()
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_season() {
        assert_eq!(
            anime_name("E:\\anime\\Makeine\\Season 1"),
            "Makeine".to_string()
        );
    }

    #[test]
    fn with_2_seasons() {
        assert_eq!(
            anime_name("E:\\anime\\Makeine\\Season 1\\Season 2"),
            "Makeine".to_string()
        );
    }
    
    #[test]
    fn with_interlaced_season() {
        assert_eq!(
            anime_name("E:\\anime\\Season 1\\Makeine\\Season 1"),
            "Makeine".to_string()
        );
    }

    #[test]
    fn no_season() {
        assert_eq!(anime_name("E:\\anime\\Makeine"), "Makeine".to_string());
    }
}
