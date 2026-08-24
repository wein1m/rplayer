use std::{fs, path::Path};

use walkdir::{WalkDir, DirEntry};

pub fn scan_music() -> Result<(), Box<dyn std::error::Error>> {
    let path = "/home/wein/rplayer-test/";

    let walker = WalkDir::new(path).into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        match entry {
            Ok(e) => {
                let path = e.path();

                if path.is_file() && is_music(path) {
                    println!("{}", e.path().display())
                };
            },
            Err(e) => eprintln!("Error: {}", e),
        }
    }
        
    Ok(())
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
