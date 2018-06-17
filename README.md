# anitomy-rs
*anitomy-rs* is a Rust binding for [Anitomy](https://github.com/erengy/anitomy) a C++ library for parsing anime
video filenames.

## Example
```rust
extern crate anitomy;

use anitomy::{Anitomy, ElementCategory};

fn main() {
    let mut anitomy = Anitomy::new();
    match anitomy.parse("[TaigaSubs]_Toradora!_(2008)_-_01v2_-_Tiger_and_Dragon_[1280x720_H.264_FLAC][1234ABCD].mkv") {
    Ok(ref elements) => {
        println!("SUCCESS: Parsed the filename successfully!");
        println!(
            "It is: {} #{} by {}", 
            elements.get(ElementCategory::AnimeTitle).expect("anime title"), 
            elements.get(ElementCategory::EpisodeNumber).expect("episode number"), 
            elements.get(ElementCategory::ReleaseGroup).expect("release group")
        );
        println!("And extracted the following elements: {:#?}", &**elements);
    },
    Err(ref elements) => {
        println!("ERROR: Couldn't parse the filename successfully!");
        println!("But we managed to extract these elements: {:#?}", &**elements);
    },
  }
}
```

Which outputs:
```
SUCCESS: Parsed the filename successfully!
It is: Toradora! #01 by TaigaSubs
And extracted the following elements: [
    Element {
        category: FileExtension,
        value: "mkv"
    },
    Element {
        category: FileName,
        value: "[TaigaSubs]_Toradora!_(2008)_-_01v2_-_Tiger_and_Dragon_[1280x720_H.264_FLAC][1234ABCD]"
    },
    Element {
        category: VideoTerm,
        value: "H.264"
    },
    Element {
        category: VideoResolution,
        value: "1280x720"
    },
    Element {
        category: AudioTerm,
        value: "FLAC"
    },
    Element {
        category: FileChecksum,
        value: "1234ABCD"
    },
    Element {
        category: AnimeYear,
        value: "2008"
    },
    Element {
        category: EpisodeNumber,
        value: "01"
    },
    Element {
        category: ReleaseVersion,
        value: "2"
    },
    Element {
        category: AnimeTitle,
        value: "Toradora!"
    },
    Element {
        category: ReleaseGroup,
        value: "TaigaSubs"
    },
    Element {
        category: EpisodeTitle,
        value: "Tiger and Dragon"
    }
]
```