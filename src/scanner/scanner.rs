use std::{fs, path::Path};

use lofty::{file::AudioFile, read_from_path};
use lofty::file::TaggedFileExt;
use lofty::tag::Accessor;

use walkdir::{WalkDir, DirEntry};

#[derive(Debug)]
pub struct TrackAudio {
    pub path: String,
    pub track_title: String,
    pub track_artist: String,
    pub track_duration: f32,
}

pub fn scan_music() -> Result<Vec<TrackAudio>, Box<dyn std::error::Error>> {
    let path = "assets";

    let mut tracks = Vec::new();

    let walker = WalkDir::new(path).into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        match entry {
            Ok(e) => {
                let path = e.path();

                if path.is_file() && is_music(path) {
                    if let Some(track) = create_track(path) {
                        tracks.push(track);
                    }
                };
            },
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    Ok(tracks)
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn is_music(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        let ext_str = ext.to_string_lossy().to_lowercase();
        matches!(ext_str.as_str(), "mp3" | "wav" | "flac" | "ogg" | "m4a")
    } else {
        false
    }
}

fn create_track(path: &Path) -> Option<TrackAudio> {
    let abs_path = fs::canonicalize(path).ok()?;
    let mut track_title = String::new();
    let mut track_artist = String::new();

    let tagged_file = match read_from_path(path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to read {}: {}", path.display(), e);
            return None
        }
    };

    if let Some(tag) = tagged_file.primary_tag() {
        if let Some(title) = tag.title() {
            track_title = title.to_string();
            
            if track_title.is_empty() {
                track_title = path.file_stem().unwrap().to_string_lossy().into();
            }
        }
        if let Some(artist) = tag.artist() {
            track_artist = artist.to_string();

            if track_artist.is_empty() {
                track_artist = String::from("Unknown Artist");
            }
        }
    }

    let track_duration = tagged_file.properties().duration().as_secs_f32();

    Some(TrackAudio {
        path: abs_path.to_string_lossy().to_string(),
        track_title,
        track_artist,
        track_duration
    })
}
