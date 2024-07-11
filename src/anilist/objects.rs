use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct Title {
    pub(crate) english: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct CoverImage {
    #[serde(rename = "extraLarge")]
    pub(crate) extra_large: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Episode {
    #[serde(rename = "airingAt")]
    pub(crate) airing_at: u32,

    pub(crate) episode: u16,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Media {
    pub(crate) title: Title,

    #[serde(rename = "coverImage")]
    pub(crate) cover_image: CoverImage,

    pub(crate) season: String,

    #[serde(rename = "seasonYear")]
    pub(crate) season_year: u16,

    pub(crate) episodes: Option<u16>,

    pub(crate) status: String,

    #[serde(rename = "nextAiringEpisode")]
    pub(crate) next_airing_episode: Option<Episode>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Data {
    #[serde(rename = "Media")]
    pub(crate) media: Media,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Response {
    pub(crate) data: Data,
}
