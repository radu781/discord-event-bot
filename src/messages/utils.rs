use serenity::all::CreateEmbed;

pub(crate) fn add_embed<Value>(embed: CreateEmbed, name: &str, value: &Option<Value>) -> CreateEmbed
where
    Value: ToString,
{
    if let Some(value) = value {
        let value = value.to_string();
        if !value.is_empty() {
            embed.field(name, value, false)
        } else {
            embed
        }
    } else {
        embed
    }
}
