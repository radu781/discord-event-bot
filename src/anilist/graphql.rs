use crate::anilist::objects::Response;
use reqwest::header::CONTENT_TYPE;
use serde_json::json;
use std::path;

pub(crate) async fn anime_info(path: &str) -> Response {
    let title = anime_name(path);
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
    let ignore_dirs = ["season", "movies", "music"];
    title
        .to_owned()
        .split(path::MAIN_SEPARATOR_STR)
        .collect::<Vec<&str>>()
        .into_iter()
        .rev()
        .find(|s| {
            !ignore_dirs
                .iter()
                .any(|dir| s.to_lowercase().contains(&dir.to_lowercase()) && s.len() < 10)
        })
        .unwrap_or(title)
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_season() {
        assert_eq!(
            anime_name("E:\\anime\\name_here\\Season 1"),
            "name_here".to_string()
        );
        assert_eq!(
            anime_name("E:\\anime\\name_here\\movies"),
            "name_here".to_string()
        );
        assert_eq!(
            anime_name("E:\\anime\\name_here\\music"),
            "name_here".to_string()
        );
    }

    #[test]
    fn with_2_seasons() {
        assert_eq!(
            anime_name("E:\\anime\\name_here\\Season 1\\Season 2"),
            "name_here".to_string()
        );
    }

    #[test]
    fn with_interlaced_season() {
        assert_eq!(
            anime_name("E:\\anime\\Season 1\\name_here\\Season 1"),
            "name_here".to_string()
        );
    }

    #[test]
    fn no_season() {
        assert_eq!(anime_name("E:\\anime\\name_here"), "name_here".to_string());
    }

    #[test]
    fn season_word_in_directory_name() {
        assert_eq!(
            anime_name("E:\\anime\\name (Season 1)"),
            "name (Season 1)".to_string()
        );
    }

    #[test]
    fn with_movie() {
        assert_eq!(
            anime_name("E:\\anime\\name_here\\Movies"),
            "name_here".to_string()
        );
    }

    #[test]
    fn with_typo() {
        assert_ne!(
            anime_name("E:\\anime\\name_here\\movieeeee"),
            "name_here".to_string()
        );
    }
    #[test]
    fn just_name() {
        assert_eq!(anime_name("name here"), "name here".to_string());
    }
}
