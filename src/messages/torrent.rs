use clap::Args;
use serenity::all::{Color, Colour, CreateEmbed, CreateMessage};

use crate::anilist::graphql::anime_info;

use super::utils::add_embed;

#[derive(Args)]
pub(crate) struct TorrentMessage {
    #[arg(long)]
    pub(crate) name: Option<String>,

    #[arg(long)]
    pub(crate) category: Option<String>,

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

        // TODO: make more robust
        if let Some(category) = &self.category {
            if category.to_lowercase().as_str() == "anime" {
                embed =
                    Self::extra_anime_fields(embed, self.save_path.clone().unwrap().as_str()).await;
            }
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

    fn category_to_color(category: &Option<String>) -> Colour {
        match category {
            Some(category) => match category.as_str() {
                "Anime" => Color::from_rgb(116, 105, 182),
                _ => Color::from_rgb(123, 211, 234),
            },
            None => Color::from_rgb(123, 211, 234),
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

    async fn extra_anime_fields(mut embed: CreateEmbed, title: &str) -> CreateEmbed {
        let info = anime_info(title).await;
        embed = embed.image(info.data.media.cover_image.extra_large);
        embed = embed.title(format!("{} download done", info.data.media.title.english));
        embed = embed.field(
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
}
