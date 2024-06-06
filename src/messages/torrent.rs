use clap::Args;
use serenity::all::{Color, Colour, CreateEmbed, CreateMessage};

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
    pub(crate) fn message(&self) -> CreateMessage {
        let mut embed = CreateEmbed::new()
            .title("Download done")
            .color(Self::category_to_color(&self.category));
        embed = add_embed(embed, "Name", &self.name);
        embed = add_embed(embed, "Category", &self.category);
        embed = add_embed(embed, "Tags", &self.tags);
        embed = add_embed(embed, "Content path", &self.content_path);
        embed = add_embed(embed, "Root path", &self.root_path);
        embed = add_embed(embed, "Save path", &self.save_path);
        embed = add_embed(embed, "Files", &self.files);
        embed = add_embed(embed, "Byte size", &self.byte_size);
        embed = add_embed(embed, "Tracker", &self.tracker);
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
}
