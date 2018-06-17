# anitomy-sys
*anitomy-sys* is a low-level Rust binding for [Anitomy](https://github.com/erengy/anitomy) a C++ library for parsing anime
video filenames.

Makes use of [anitomy-c](https://github.com/Xtansia/anitomy-c) a C ABI wrapper for Anitomy. 

## Installation
Add this to your `Cargo.toml`:
```toml
[dependencies]
anitomy-sys = "0.1"
```

*anitomy-sys* will compile and statically link *anitomy-c* and *Anitomy* at build time, as such a compatible compiler is required.

### Requirements
* A C++14 compatible compiler
  - GCC >= 5
  - Clang >= 3.4 (According to the [Clang CXX status page](https://clang.llvm.org/cxx_status.html))
  - [Visual Studio 2017](https://www.visualstudio.com/downloads/) 
    OR [Build Tools for Visual Studio 2017](https://aka.ms/BuildTools)

## Example
```rust
extern crate anitomy_sys;

use anitomy_sys::{Anitomy, ElementCategory};
use std::ffi::CString;

fn main() {
    let mut anitomy = unsafe { Anitomy::new() };
    let filename = CString::new("[TaigaSubs]_Toradora!_(2008)_-_01v2_-_Tiger_and_Dragon_[1280x720_H.264_FLAC][1234ABCD].mkv").expect("no nul chars in filename");
    let success = unsafe { anitomy.parse(&filename) };
    println!("Success? {}", success);
    unsafe {
        let elements = anitomy.elements();
        println!(
            "It is: {} #{} by {}",
            elements.get(ElementCategory::AnimeTitle),
            elements.get(ElementCategory::EpisodeNumber),
            elements.get(ElementCategory::ReleaseGroup)
        );
        (0..elements.count(None))
            .flat_map(|i| elements.at(i))
            .for_each(|e| println!("{:?}: {:?}", e.category, e.value));
    }
    unsafe { anitomy.destroy() };
}
```

Which outputs:
```
Success? true
It is: Toradora! #01 by TaigaSubs
FileExtension: "mkv"
FileName: "[TaigaSubs]_Toradora!_(2008)_-_01v2_-_Tiger_and_Dragon_[1280x720_H.264_FLAC][1234ABCD]"
VideoTerm: "H.264"
VideoResolution: "1280x720"
AudioTerm: "FLAC"
FileChecksum: "1234ABCD"
AnimeYear: "2008"
EpisodeNumber: "01"
ReleaseVersion: "2"
AnimeTitle: "Toradora!"
ReleaseGroup: "TaigaSubs"
EpisodeTitle: "Tiger and Dragon"
```