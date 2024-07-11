use serenity::all::CreateEmbed;

pub(crate) fn add_embed(embed: CreateEmbed, name: &str, value: &Option<String>) -> CreateEmbed {
    if let Some(value) = value {
        if !value.is_empty() {
            embed.field(name, value, false)
        } else {
            embed
        }
    } else {
        embed
    }
}
