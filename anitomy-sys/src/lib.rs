//! # anitomy-sys
//! *anitomy-sys* is a low-level Rust binding for [Anitomy](https://github.com/erengy/anitomy) a C++ library for parsing anime
//! video filenames.
//! 
//! Makes use of [anitomy-c](https://github.com/Xtansia/anitomy-c) a C ABI wrapper for Anitomy. 
//! 
//! ## Installation
//! Add this to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! anitomy-sys = "0.1"
//! ```
//! 
//! *anitomy-sys* will compile and statically link *anitomy-c* and *Anitomy* at build time, as such a compatible compiler is required.
//! 
//! ### Requirements
//! * A C++14 compatible compiler
//!   - GCC >= 5
//!   - Clang >= 3.4 (According to the [Clang CXX status page](https://clang.llvm.org/cxx_status.html))
//!   - [Visual Studio 2017](https://www.visualstudio.com/downloads/) 
//!     OR [Build Tools for Visual Studio 2017](https://aka.ms/BuildTools)
//! 
//! ## Example
//! ```no_run
//! extern crate anitomy_sys;
//! 
//! use anitomy_sys::{Anitomy, ElementCategory};
//! use std::ffi::CString;
//! 
//! fn main() {
//!     let mut anitomy = unsafe { Anitomy::new() };
//!     let filename = CString::new("[TaigaSubs]_Toradora!_(2008)_-_01v2_-_Tiger_and_Dragon_[1280x720_H.264_FLAC][1234ABCD].mkv").expect("no nul chars in filename");
//!     let success = unsafe { anitomy.parse(&filename) };
//!     println!("Success? {}", success);
//!     unsafe {
//!         let elements = anitomy.elements();
//!         println!(
//!             "It is: {} #{} by {}",
//!             elements.get(ElementCategory::AnimeTitle),
//!             elements.get(ElementCategory::EpisodeNumber),
//!             elements.get(ElementCategory::ReleaseGroup)
//!         );
//!         (0..elements.count(None))
//!             .flat_map(|i| elements.at(i))
//!             .for_each(|e| println!("{:?}: {:?}", e.category, e.value));
//!     }
//!     unsafe { anitomy.destroy() };
//! }
//! ```

pub mod ffi;

use std::ffi::CStr;

/// The options used by Anitomy to determine how to parse a filename.
#[repr(C)]
pub struct Options {
    options: ffi::options_t,
}

impl Options {
    /// Set the allowed delimiters.
    pub unsafe fn allowed_delimiters<S: AsRef<CStr>>(&mut self, allowed_delimiters: S) {
        ffi::options_allowed_delimiters(&mut self.options, allowed_delimiters.as_ref().as_ptr())
    }

    /// Set the strings to ignore.
    pub unsafe fn ignored_strings<S: AsRef<CStr>>(&mut self, ignored_strings: &[S]) {
        let array = ffi::string_array_new();
        ignored_strings
            .iter()
            .for_each(|cstr| ffi::string_array_add(array, cstr.as_ref().as_ptr()));
        ffi::options_ignored_strings(&mut self.options, array);
        ffi::string_array_free(array);
    }

    /// Set whether to attempt to parse the episode number.
    pub unsafe fn parse_episode_number(&mut self, parse_episode_number: bool) {
        ffi::options_parse_episode_number(&mut self.options, parse_episode_number)
    }

    /// Set whether to attempt to parse the episode title.
    pub unsafe fn parse_episode_title(&mut self, parse_episode_title: bool) {
        ffi::options_parse_episode_title(&mut self.options, parse_episode_title)
    }

    /// Set whether to attempt to parse the file extension.
    pub unsafe fn parse_file_extension(&mut self, parse_file_extension: bool) {
        ffi::options_parse_file_extension(&mut self.options, parse_file_extension)
    }

    /// Set whether to attempt to parse the release group.
    pub unsafe fn parse_release_group(&mut self, parse_release_group: bool) {
        ffi::options_parse_release_group(&mut self.options, parse_release_group)
    }
}

/// The category of an [`Element`](::Element).
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ElementCategory {
    AnimeSeason = ffi::kElementAnimeSeason,
    AnimeSeasonPrefix = ffi::kElementAnimeSeasonPrefix,
    AnimeTitle = ffi::kElementAnimeTitle,
    AnimeType = ffi::kElementAnimeType,
    AnimeYear = ffi::kElementAnimeYear,
    AudioTerm = ffi::kElementAudioTerm,
    DeviceCompatibility = ffi::kElementDeviceCompatibility,
    EpisodeNumber = ffi::kElementEpisodeNumber,
    EpisodeNumberAlt = ffi::kElementEpisodeNumberAlt,
    EpisodePrefix = ffi::kElementEpisodePrefix,
    EpisodeTitle = ffi::kElementEpisodeTitle,
    FileChecksum = ffi::kElementFileChecksum,
    FileExtension = ffi::kElementFileExtension,
    FileName = ffi::kElementFileName,
    Language = ffi::kElementLanguage,
    Other = ffi::kElementOther,
    ReleaseGroup = ffi::kElementReleaseGroup,
    ReleaseInformation = ffi::kElementReleaseInformation,
    ReleaseVersion = ffi::kElementReleaseVersion,
    Source = ffi::kElementSource,
    Subtitles = ffi::kElementSubtitles,
    VideoResolution = ffi::kElementVideoResolution,
    VideoTerm = ffi::kElementVideoTerm,
    VolumeNumber = ffi::kElementVolumeNumber,
    VolumePrefix = ffi::kElementVolumePrefix,
    Unknown = ffi::kElementUnknown,
}

impl From<ffi::element_category_t> for ElementCategory {
    fn from(val: ffi::element_category_t) -> ElementCategory {
        match val {
            ffi::kElementAnimeSeason => ElementCategory::AnimeSeason,
            ffi::kElementAnimeSeasonPrefix => ElementCategory::AnimeSeasonPrefix,
            ffi::kElementAnimeTitle => ElementCategory::AnimeTitle,
            ffi::kElementAnimeType => ElementCategory::AnimeType,
            ffi::kElementAnimeYear => ElementCategory::AnimeYear,
            ffi::kElementAudioTerm => ElementCategory::AudioTerm,
            ffi::kElementDeviceCompatibility => ElementCategory::DeviceCompatibility,
            ffi::kElementEpisodeNumber => ElementCategory::EpisodeNumber,
            ffi::kElementEpisodeNumberAlt => ElementCategory::EpisodeNumberAlt,
            ffi::kElementEpisodePrefix => ElementCategory::EpisodePrefix,
            ffi::kElementEpisodeTitle => ElementCategory::EpisodeTitle,
            ffi::kElementFileChecksum => ElementCategory::FileChecksum,
            ffi::kElementFileExtension => ElementCategory::FileExtension,
            ffi::kElementFileName => ElementCategory::FileName,
            ffi::kElementLanguage => ElementCategory::Language,
            ffi::kElementOther => ElementCategory::Other,
            ffi::kElementReleaseGroup => ElementCategory::ReleaseGroup,
            ffi::kElementReleaseInformation => ElementCategory::ReleaseInformation,
            ffi::kElementReleaseVersion => ElementCategory::ReleaseVersion,
            ffi::kElementSource => ElementCategory::Source,
            ffi::kElementSubtitles => ElementCategory::Subtitles,
            ffi::kElementVideoResolution => ElementCategory::VideoResolution,
            ffi::kElementVideoTerm => ElementCategory::VideoTerm,
            ffi::kElementVolumeNumber => ElementCategory::VolumeNumber,
            ffi::kElementVolumePrefix => ElementCategory::VolumePrefix,
            _ => ElementCategory::Unknown,
        }
    }
}

/// An element parsed from a filename by Anitomy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Element {
    /// The category of the element.
    pub category: ElementCategory,
    /// The value of the element.
    pub value: String,
}

/// The collection of elements parsed from a filename by Anitomy.
#[repr(C)]
pub struct Elements {
    elements: ffi::elements_t,
}

impl Elements {
    /// Determines whether there are no elements of a given category.
    /// 
    /// Passing `None` will check for any elements at all.
    pub unsafe fn empty<C: Into<Option<ElementCategory>>>(&self, category: C) -> bool {
        match category.into() {
            Some(cat) => {
                ffi::elements_empty_category(&self.elements, cat as ffi::element_category_t)
            }
            None => ffi::elements_empty(&self.elements),
        }
    }

    /// Counts the number of elements of a given category.
    /// 
    /// Passing `None` will count all elements.
    pub unsafe fn count<C: Into<Option<ElementCategory>>>(&self, category: C) -> usize {
        match category.into() {
            Some(cat) => {
                ffi::elements_count_category(&self.elements, cat as ffi::element_category_t)
            }
            None => ffi::elements_count(&self.elements),
        }
    }

    /// Get the element at the given position if one exists.
    pub unsafe fn at(&self, pos: usize) -> Option<Element> {
        if pos < self.count(None) {
            let pair = ffi::elements_at(&self.elements, pos);
            let value = ffi::raw_into_string(pair.value);
            ffi::string_free(pair.value);
            Some(Element {
                category: ElementCategory::from(pair.category),
                value: value,
            })
        } else {
            None
        }
    }

    /// Gets the first element of a category if one exists, otherwise returns an empty string.
    pub unsafe fn get(&self, category: ElementCategory) -> String {
        let rawval = ffi::elements_get(&self.elements, category as ffi::element_category_t);
        let val = ffi::raw_into_string(rawval);
        ffi::string_free(rawval);
        val
    }

    /// Gets all elements of a category.
    pub unsafe fn get_all(&self, category: ElementCategory) -> Vec<String> {
        let rawvals = ffi::elements_get_all(&self.elements, category as ffi::element_category_t);
        let size = ffi::string_array_size(rawvals);
        let vals = (0..size)
            .map(|i| ffi::raw_into_string(ffi::string_array_at(rawvals, i)))
            .collect();
        ffi::string_array_free(rawvals);
        vals
    }
}

/// An Anitomy parser instance.
#[derive(Debug)]
pub struct Anitomy {
    anitomy: *mut ffi::anitomy_t,
}

impl Anitomy {
    /// Construct a new Anitomy instance.
    pub unsafe fn new() -> Self {
        Self {
            anitomy: ffi::anitomy_new(),
        }
    }

    /// Parses a filename.
    /// 
    /// `true` and `false` return values correspond to what Anitomy classifies as succeeding or failing in parsing a filename.
    /// Such as an [`AnimeTitle`](::ElementCategory::AnimeTitle) element being found.
    pub unsafe fn parse<S: AsRef<CStr>>(&mut self, filename: S) -> bool {
        ffi::anitomy_parse(self.anitomy, filename.as_ref().as_ptr())
    }

    /// Get the [`Elements`](::Elements) container of this Anitomy instance.
    pub unsafe fn elements(&self) -> &Elements {
        &*(ffi::anitomy_elements(self.anitomy) as *const Elements)
    }

    /// Get the [`Options`](::Options) of this Anitomy instance.
    pub unsafe fn options(&mut self) -> &mut Options {
        &mut *(ffi::anitomy_options(self.anitomy) as *mut Options)
    }

    /// Destroy this instance.
    /// 
    /// This should always be called to free the associated C++ Anitomy instance and resources.
    pub unsafe fn destroy(&mut self) {
        ffi::anitomy_destroy(self.anitomy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    const BLACK_BULLET_FILENAME: &'static str =
        "[異域字幕組][漆黑的子彈][Black Bullet][11-12][1280x720][繁体].mp4";
    const TORADORA_FILENAME: &'static str = "[TaigaSubs]_Toradora!_(2008)_-_01v2_-_Tiger_and_Dragon_[1280x720_H.264_FLAC][1234ABCD].mkv";

    #[test]
    fn anitomy_new_destroy() {
        unsafe {
            let mut ani = Anitomy::new();
            ani.destroy();
        }
    }

    #[test]
    fn anitomy_parse_good_input() {
        unsafe {
            let filename = CString::new(BLACK_BULLET_FILENAME).unwrap();
            let mut ani = Anitomy::new();

            assert!(ani.parse(&filename));

            ani.destroy();
        }
    }

    #[test]
    fn anitomy_parse_bad_input() {
        unsafe {
            let filename = CString::new("").unwrap();
            let mut ani = Anitomy::new();

            assert!(!ani.parse(&filename));

            ani.destroy();
        }
    }

    #[test]
    fn anitomy_elements_empty_good_input() {
        unsafe {
            let filename = CString::new(BLACK_BULLET_FILENAME).unwrap();
            let mut ani = Anitomy::new();

            assert!(ani.parse(&filename));
            {
                let elems = ani.elements();
                assert!(!elems.empty(None));
                assert!(!elems.empty(ElementCategory::AnimeTitle));
                assert!(elems.count(None) > 0);
                assert!(elems.count(ElementCategory::AnimeTitle) == 1);
            }

            ani.destroy()
        }
    }

    #[test]
    fn anitomy_elements_empty_bad_input() {
        unsafe {
            let filename = CString::new("").unwrap();
            let mut ani = Anitomy::new();

            assert!(!ani.parse(&filename));
            {
                let elems = ani.elements();
                assert!(elems.empty(None));
                assert!(elems.empty(ElementCategory::AnimeTitle));
                assert!(elems.count(None) == 0);
                assert!(elems.count(ElementCategory::AnimeTitle) == 0);
            }

            ani.destroy()
        }
    }

    #[test]
    fn anitomy_elements_get_good_input() {
        unsafe {
            let filename = CString::new(BLACK_BULLET_FILENAME).unwrap();
            let mut ani = Anitomy::new();

            assert!(ani.parse(&filename));
            {
                let elems = ani.elements();
                assert!(elems.count(ElementCategory::AnimeTitle) == 1);
                assert_eq!(elems.get(ElementCategory::AnimeTitle), "Black Bullet");
            }

            ani.destroy()
        }
    }

    #[test]
    fn anitomy_elements_get_bad_input() {
        unsafe {
            let filename = CString::new("").unwrap();
            let mut ani = Anitomy::new();

            assert!(!ani.parse(&filename));
            {
                let elems = ani.elements();
                assert!(elems.count(ElementCategory::AnimeTitle) == 0);
                assert_eq!(elems.get(ElementCategory::AnimeTitle), "");
            }

            ani.destroy()
        }
    }

    #[test]
    fn anitomy_elements_get_all_good_input() {
        unsafe {
            let filename = CString::new(BLACK_BULLET_FILENAME).unwrap();
            let mut ani = Anitomy::new();

            assert!(ani.parse(&filename));
            {
                let elems = ani.elements();
                assert!(elems.count(ElementCategory::EpisodeNumber) == 2);
                assert_eq!(elems.get_all(ElementCategory::EpisodeNumber), ["11", "12"]);
            }

            ani.destroy()
        }
    }

    #[test]
    fn anitomy_elements_get_all_bad_input() {
        unsafe {
            let filename = CString::new("").unwrap();
            let mut ani = Anitomy::new();

            assert!(!ani.parse(&filename));
            {
                let elems = ani.elements();
                assert!(elems.count(ElementCategory::EpisodeNumber) == 0);
                assert_eq!(
                    elems.get_all(ElementCategory::EpisodeNumber),
                    Vec::<String>::new()
                );
            }

            ani.destroy()
        }
    }

    #[test]
    fn anitomy_elements_at() {
        unsafe {
            let filename = CString::new(BLACK_BULLET_FILENAME).unwrap();
            let mut ani = Anitomy::new();

            assert!(ani.parse(&filename));
            {
                let elems = ani.elements();
                let pair = elems.at(0).expect("at least one element");
                assert_eq!(pair.category, ElementCategory::FileExtension);
                assert_eq!(pair.value, "mp4");
            }

            ani.destroy();
        }
    }

    #[test]
    fn anitomy_options_allowed_delimiters() {
        unsafe {
            let filename = CString::new(TORADORA_FILENAME).unwrap();
            let mut ani = Anitomy::new();

            assert!(ani.parse(&filename));
            {
                let elems = ani.elements();
                assert!(elems.count(ElementCategory::AnimeTitle) == 1);
                assert_eq!(elems.get(ElementCategory::AnimeTitle), "Toradora!");
            }

            {
                ani.options().allowed_delimiters(&CString::new("").unwrap());
            }

            assert!(ani.parse(&filename));
            {
                let elems = ani.elements();
                assert!(elems.count(ElementCategory::AnimeTitle) == 1);
                assert_eq!(elems.get(ElementCategory::AnimeTitle), "_Toradora!_");
            }

            ani.destroy();
        }
    }

    #[test]
    fn anitomy_options_ignored_strings() {
        unsafe {
            let filename = CString::new(TORADORA_FILENAME).unwrap();
            let mut ani = Anitomy::new();

            assert!(ani.parse(&filename));
            {
                let elems = ani.elements();
                assert!(elems.count(ElementCategory::EpisodeTitle) == 1);
                assert_eq!(elems.get(ElementCategory::EpisodeTitle), "Tiger and Dragon");
            }

            {
                ani.options()
                    .ignored_strings(&[CString::new("Dragon").unwrap()]);
            }

            assert!(ani.parse(&filename));
            {
                let elems = ani.elements();
                assert!(elems.count(ElementCategory::EpisodeTitle) == 1);
                assert_eq!(elems.get(ElementCategory::EpisodeTitle), "Tiger and");
            }

            ani.destroy();
        }
    }

    #[test]
    fn anitomy_options_parse_episode_number() {
        unsafe {
            let filename = CString::new(TORADORA_FILENAME).unwrap();
            let mut ani = Anitomy::new();

            assert!(ani.parse(&filename));
            {
                let elems = ani.elements();
                assert!(elems.count(ElementCategory::EpisodeNumber) == 1);
            }

            {
                ani.options().parse_episode_number(false);
            }

            assert!(ani.parse(&filename));
            {
                let elems = ani.elements();
                assert!(elems.count(ElementCategory::EpisodeNumber) == 0);
            }

            ani.destroy();
        }
    }

    #[test]
    fn anitomy_options_parse_episode_title() {
        unsafe {
            let filename = CString::new(TORADORA_FILENAME).unwrap();
            let mut ani = Anitomy::new();

            assert!(ani.parse(&filename));
            {
                let elems = ani.elements();
                assert!(elems.count(ElementCategory::EpisodeTitle) == 1);
            }

            {
                ani.options().parse_episode_title(false);
            }

            assert!(ani.parse(&filename));
            {
                let elems = ani.elements();
                assert!(elems.count(ElementCategory::EpisodeTitle) == 0);
            }

            ani.destroy();
        }
    }

    #[test]
    fn anitomy_options_parse_file_extension() {
        unsafe {
            let filename = CString::new(TORADORA_FILENAME).unwrap();
            let mut ani = Anitomy::new();

            assert!(ani.parse(&filename));
            {
                let elems = ani.elements();
                assert!(elems.count(ElementCategory::FileExtension) == 1);
            }

            {
                ani.options().parse_file_extension(false);
            }

            assert!(ani.parse(&filename));
            {
                let elems = ani.elements();
                assert!(elems.count(ElementCategory::FileExtension) == 0);
            }

            ani.destroy();
        }
    }

    #[test]
    fn anitomy_options_parse_release_group() {
        unsafe {
            let filename = CString::new(TORADORA_FILENAME).unwrap();
            let mut ani = Anitomy::new();

            assert!(ani.parse(&filename));
            {
                let elems = ani.elements();
                assert!(elems.count(ElementCategory::ReleaseGroup) == 1);
            }

            {
                ani.options().parse_release_group(false);
            }

            assert!(ani.parse(&filename));
            {
                let elems = ani.elements();
                assert!(elems.count(ElementCategory::ReleaseGroup) == 0);
            }

            ani.destroy();
        }
    }
}
