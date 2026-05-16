use lofty::prelude::*;
use lofty::probe::Probe;
use std::path::Path;
use std::time::Duration;

pub struct TrackInfo {
    pub title: String,
    pub artist: String,
    pub duration: Duration,
}

pub fn read_metadata(path: &Path) -> Option<TrackInfo> {
    let tagged_file = Probe::open(path).ok()?.read().ok()?;

    let (title, artist) = if let Some(tag) = tagged_file.primary_tag().or_else(|| tagged_file.first_tag()) {
        (
            tag.title().unwrap_or_default().to_string(),
            tag.artist().unwrap_or_default().to_string(),
        )
    } else {
        (String::new(), String::new())
    };

    let duration = tagged_file.properties().duration();

    let title = if title.is_empty() {
        path.file_stem()?.to_str()?.to_string()
    } else {
        title
    };

    Some(TrackInfo {
        title,
        artist,
        duration,
    })
}
