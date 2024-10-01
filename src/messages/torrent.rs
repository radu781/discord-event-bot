use super::utils::add_embed;
use crate::anilist::graphql::anime_info;
use clap::Args;
use serenity::all::{Color, Colour, CreateEmbed, CreateMessage};
use std::{fmt::Display, str::FromStr};

#[derive(Args, Debug)]
pub(crate) struct TorrentMessage {
    #[arg(long)]
    pub(crate) name: Option<String>,

    #[arg(long)]
    pub(crate) category: Option<Category>,

    #[arg(long)]
    pub(crate) tags: Option<String>,

    #[arg(long)]
    pub(crate) content_path: Option<String>,

    #[arg(long)]
    pub(crate) root_path: Option<String>,

    #[arg(long)]
    pub(crate) save_path: Option<String>,

    #[arg(long)]
    pub(crate) files: Option<String>,

    #[arg(long)]
    pub(crate) byte_size: Option<String>,

    #[arg(long)]
    pub(crate) tracker: Option<String>,

    #[arg(long)]
    pub(crate) info_hash_v1: Option<String>,

    #[arg(long)]
    pub(crate) info_hash_v2: Option<String>,

    #[arg(long)]
    pub(crate) id: Option<String>,
}

impl TorrentMessage {
    pub(crate) async fn message(&self) -> CreateMessage {
        let mut embed = CreateEmbed::new()
            .title("Download done")
            .color(Self::category_to_color(&self.category));

        if let Some(category) = &self.category {
            embed = category.extra_fields(embed, &self.save_path).await
        }
        embed = add_embed(embed, "Name", &self.name);
        embed = add_embed(embed, "Category", &self.category);
        embed = add_embed(embed, "Tags", &self.tags);
        // embed = add_embed(embed, "Content path", &self.content_path);
        embed = add_embed(embed, "Root path", &self.root_path);
        embed = add_embed(embed, "Folder", &self.save_path);
        embed = add_embed(embed, "Files", &self.files);
        embed = add_embed(embed, "Size", &Self::byte_size_readable(&self.byte_size));
        // embed = add_embed(embed, "Tracker", &self.tracker);
        embed = add_embed(embed, "Info hash v1", &self.info_hash_v1);
        embed = add_embed(embed, "Info hash v2", &self.info_hash_v2);
        embed = add_embed(embed, "Id", &self.id);
        CreateMessage::new().add_embed(embed)
    }

    fn category_to_color(category: &Option<Category>) -> Colour {
        match category {
            Some(Category::Anime) => Color::from_rgb(116, 105, 182),
            _ => Color::from_rgb(123, 211, 234),
        }
    }

    fn byte_size_readable(byte_size: &Option<String>) -> Option<String> {
        match byte_size {
            Some(bytes) => match bytes.parse::<u64>() {
                Ok(b) => {
                    let units = ["B", "KB", "MB", "GB", "TB"];
                    let mut idx = 0;
                    let mut b = b as f64;
                    while b > 1024.0 && idx < units.len() - 1 {
                        b /= 1024.0;
                        idx += 1;
                    }
                    Some(format!("{:.2} {}", b, units[idx]))
                }
                Err(_) => Some("?".to_owned()),
            },
            None => None,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Category {
    Anime,
    Game,
    Other(String),
}

impl Category {
    async fn extra_fields(
        &self,
        mut embed: CreateEmbed,
        save_path: &Option<String>,
    ) -> CreateEmbed {
        match self {
            Category::Anime => match save_path {
                Some(path) => {
                    let info = anime_info(path).await;
                    let media_title = info.data.media.title;
                    let title = media_title.english.unwrap_or(media_title.romaji.unwrap_or(
                        media_title.native.unwrap_or_else(|| {
                            eprintln!("No title language found");
                            "???".to_owned()
                        }),
                    ));

                    embed = embed
                        .image(info.data.media.cover_image.extra_large)
                        .title(format!("{title} download done"))
                        .field(
                            "Airing season",
                            format!(
                                "{} {}",
                                info.data.media.season_year,
                                info.data.media.season.to_lowercase()
                            ),
                            false,
                        );
                    if let Some(next) = info.data.media.next_airing_episode {
                        embed = embed.field(
                            "Next episode",
                            format!("<t:{}:R> <t:{}:F>", next.airing_at, next.airing_at),
                            false,
                        );
                    }
                    embed
                }
                None => embed,
            },
            Category::Game => embed,
            Category::Other(_) => embed,
        }
    }
}

impl FromStr for Category {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "anime" => Ok(Self::Anime),
            "game" => Ok(Self::Game),
            other => Ok(Self::Other(other.to_owned())),
        }
    }
}

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Category::Anime => write!(f, "anime"),
            Category::Game => write!(f, "game"),
            Category::Other(other) => write!(f, "{other}"),
        }
    }
}
