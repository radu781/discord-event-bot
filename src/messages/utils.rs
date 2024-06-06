use serenity::all::CreateEmbed;

pub(crate) fn add_embed(embed: CreateEmbed, name: &str, value: &Option<String>) -> CreateEmbed {
    if let Some(value) = value {
        embed.field(name, value, false)
    } else {
        embed
    }
}
