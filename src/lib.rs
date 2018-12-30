//! # rust-anitomy
//! *rust-anitomy* is a Rust binding for [Anitomy](https://github.com/erengy/anitomy) a C++ library for parsing anime
//! video filenames.
//!
//! ## Installation
//! Add this to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! anitomy = "0.1"
//! ```
//!
//! ### Requirements
//! As this crate depends on *anitomy-sys* it's requirements also apply, see [Xtansia/rust-anitomy-sys](https://github.com/Xtansia/rust-anitomy-sys) for information about *anitomy-sys*.
//!
//! ## Example
//! ```no_run
//! extern crate anitomy;
//!
//! use anitomy::{Anitomy, ElementCategory};
//!
//! fn main() {
//!     let mut anitomy = Anitomy::new();
//!     match anitomy.parse("[TaigaSubs]_Toradora!_(2008)_-_01v2_-_Tiger_and_Dragon_[1280x720_H.264_FLAC][1234ABCD].mkv") {
//!     Ok(ref elements) => {
//!         println!("SUCCESS: Parsed the filename successfully!");
//!         println!(
//!             "It is: {} #{} by {}",
//!             elements.get(ElementCategory::AnimeTitle).expect("anime title"),
//!             elements.get(ElementCategory::EpisodeNumber).expect("episode number"),
//!             elements.get(ElementCategory::ReleaseGroup).expect("release group")
//!         );
//!         println!("And extracted the following elements: {:#?}", &**elements);
//!     },
//!     Err(ref elements) => {
//!         println!("ERROR: Couldn't parse the filename successfully!");
//!         println!("But we managed to extract these elements: {:#?}", &**elements);
//!     },
//!   }
//! }
//! ```

#[doc(hidden)]
pub extern crate anitomy_sys as sys;

use std::ffi::CString;

pub use sys::Element;
pub use sys::ElementCategory;

/// The options used by Anitomy to determine how to parse a filename.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Options {
    pub allowed_delimiters: Vec<char>,
    pub ignored_strings: Vec<String>,
    pub parse_episode_number: bool,
    pub parse_episode_title: bool,
    pub parse_file_extension: bool,
    pub parse_release_group: bool,
}

impl Options {
    /// Construct a new instance of Options with the given settings.
    pub fn new<S: AsRef<str>>(
        allowed_delimiters: &[char],
        ignored_strings: &[S],
        parse_episode_number: bool,
        parse_episode_title: bool,
        parse_file_extension: bool,
        parse_release_group: bool,
    ) -> Self {
        Self {
            allowed_delimiters: allowed_delimiters.iter().cloned().collect(),
            ignored_strings: ignored_strings
                .iter()
                .map(AsRef::as_ref)
                .map(str::to_owned)
                .collect(),
            parse_episode_number,
            parse_episode_title,
            parse_file_extension,
            parse_release_group,
        }
    }

    /// Adds a delimiter char to the list of allowed delimiters.
    pub fn allow_delimiter(&mut self, delimiter: char) -> &mut Self {
        self.allowed_delimiters.push(delimiter);
        self
    }

    /// Replaces the list of allowed delimiters.
    pub fn allow_delimiters(&mut self, delimiters: &[char]) -> &mut Self {
        self.allowed_delimiters = delimiters.iter().cloned().collect();
        self
    }

    /// Adds a string to the list of ignored strings.
    pub fn ignore_string<S: AsRef<str>>(&mut self, string: S) -> &mut Self {
        self.ignored_strings.push(string.as_ref().into());
        self
    }

    /// Replaces the list of ignored strings.
    pub fn ignore_strings<S: AsRef<str>>(&mut self, strings: &[S]) -> &mut Self {
        self.ignored_strings = strings
            .iter()
            .map(AsRef::as_ref)
            .map(str::to_owned)
            .collect();
        self
    }

    /// Sets whether Anitomy should attempt to parse the episode number.
    pub fn parse_episode_number(&mut self, parse: bool) -> &mut Self {
        self.parse_episode_number = parse;
        self
    }

    /// Sets whether Anitomy should attempt to parse the episode title.
    pub fn parse_episode_title(&mut self, parse: bool) -> &mut Self {
        self.parse_episode_title = parse;
        self
    }

    /// Sets whether Anitomy should attempt to parse the file extension.
    pub fn parse_file_extension(&mut self, parse: bool) -> &mut Self {
        self.parse_file_extension = parse;
        self
    }

    /// Sets whether Anitomy should attempt to parse the release group.
    pub fn parse_release_group(&mut self, parse: bool) -> &mut Self {
        self.parse_release_group = parse;
        self
    }
}

impl Default for Options {
    /// Constructs a new instance of Options with the Anitomy defaults.
    ///
    /// Equivalent to:
    /// ```ignore
    /// Options {
    ///     allowed_delimiters: vec![' ', '_', '.', '&', '+', ',', '|'],
    ///     ignored_strings: vec![],
    ///     parse_episode_number: true,
    ///     parse_episode_title: true,
    ///     parse_file_extension: true,
    ///     parse_release_group: true,
    /// }
    /// ```
    fn default() -> Self {
        Self::new::<&str>(
            &[' ', '_', '.', '&', '+', ',', '|'],
            &[],
            true,
            true,
            true,
            true,
        )
    }
}

/// Collection of [`Element`](::Element) instances parsed from a filename, as a result of calling [`Anitomy.parse`](::Anitomy::parse).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Elements {
    elements: Vec<Element>,
}

impl Elements {
    fn new(elements: &sys::Elements) -> Self {
        Self {
            elements: unsafe {
                (0..elements.count(None))
                    .flat_map(|i| elements.at(i))
                    .collect()
            },
        }
    }

    /// Determines whether there are no elements of a given category.
    ///
    /// Passing `None` will check for any elements at all.
    pub fn is_empty<C: Into<Option<ElementCategory>>>(&self, category: C) -> bool {
        match category.into() {
            Some(category) => !self.elements.iter().any(|elem| elem.category == category),
            None => self.elements.is_empty(),
        }
    }

    /// Counts the number of elements of a given category.
    ///
    /// Passing `None` will count all elements.
    pub fn count<C: Into<Option<ElementCategory>>>(&self, category: C) -> usize {
        match category.into() {
            Some(category) => self
                .elements
                .iter()
                .filter(|elem| elem.category == category)
                .count(),
            None => self.elements.len(),
        }
    }

    /// Gets the first element of a category if one exists.
    pub fn get(&self, category: ElementCategory) -> Option<&str> {
        self.elements
            .iter()
            .find(|elem| elem.category == category)
            .map(|elem| elem.value.as_str())
    }

    /// Gets all elements of a category.
    pub fn get_all(&self, category: ElementCategory) -> Vec<&str> {
        self.elements
            .iter()
            .filter(|elem| elem.category == category)
            .map(|elem| elem.value.as_str())
            .collect()
    }
}

impl std::ops::Deref for Elements {
    type Target = Vec<Element>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.elements
    }
}

/// An Anitomy parser instance.
pub struct Anitomy {
    anitomy: sys::Anitomy,
}

impl Anitomy {
    /// Construct a new Anitomy instance.
    pub fn new() -> Self {
        Self {
            anitomy: unsafe { sys::Anitomy::new() },
        }
    }

    /// Parses a filename.
    ///
    /// The `Ok` and `Err` variants correspond to what Anitomy classifies as succeeding or failing in parsing a filename.
    /// Such as an [`AnimeTitle`](::ElementCategory::AnimeTitle) element being found.
    /// Regardless the parsed elements are returned either way.
    pub fn parse<S: AsRef<str>>(&mut self, filename: S) -> Result<Elements, Elements> {
        unsafe {
            // TODO: Better handle the CString creation here?
            let filename = CString::new(filename.as_ref()).expect("no nul chars in filename");
            if self.anitomy.parse(&filename) {
                Ok(Elements::new(self.anitomy.elements()))
            } else {
                Err(Elements::new(self.anitomy.elements()))
            }
        }
    }

    /// Sets the options to be used by Anitomy when parsing filenames.
    pub fn set_options(&mut self, options: &Options) {
        unsafe {
            // TODO: Better handle the CString creation here?
            let opts = self.anitomy.options();
            let allowed_delimiters = CString::new(
                options
                    .allowed_delimiters
                    .iter()
                    .filter(|&&c| c != '\0')
                    .collect::<String>(),
            ).expect("no nul chars in allowed delimiters");
            opts.allowed_delimiters(&allowed_delimiters);
            let ignored_strings = options
                .ignored_strings
                .iter()
                .map(|s| CString::new(s.as_str()))
                .collect::<Result<Vec<_>, _>>()
                .expect("no nul chars in ignored strings");
            opts.ignored_strings(&ignored_strings);
            opts.parse_episode_number(options.parse_episode_number);
            opts.parse_episode_title(options.parse_episode_title);
            opts.parse_file_extension(options.parse_file_extension);
            opts.parse_release_group(options.parse_release_group);
        }
    }
}

impl Drop for Anitomy {
    fn drop(&mut self) {
        unsafe { self.anitomy.destroy() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BLACK_BULLET_FILENAME: &'static str =
        "[異域字幕組][漆黑的子彈][Black Bullet][11-12][1280x720][繁体].mp4";
    const TORADORA_FILENAME: &'static str = "[TaigaSubs]_Toradora!_(2008)_-_01v2_-_Tiger_and_Dragon_[1280x720_H.264_FLAC][1234ABCD].mkv";

    #[test]
    fn anitomy_new_destroy() {
        Anitomy::new();
    }

    #[test]
    fn anitomy_parse_good_input() {
        assert!(Anitomy::new().parse(BLACK_BULLET_FILENAME).is_ok());
    }

    #[test]
    fn anitomy_parse_bad_input() {
        assert!(!Anitomy::new().parse("").is_ok());
    }

    #[test]
    fn anitomy_elements_empty_good_input() {
        let elems = Anitomy::new()
            .parse(BLACK_BULLET_FILENAME)
            .expect("successfully parse filename");
        assert!(!elems.is_empty(None));
        assert!(!elems.is_empty(ElementCategory::AnimeTitle));
        assert!(elems.count(None) > 0);
        assert!(elems.count(ElementCategory::AnimeTitle) == 1);
    }

    #[test]
    fn anitomy_elements_empty_bad_input() {
        let elems = Anitomy::new()
            .parse("")
            .expect_err("fail to parse filename");
        assert!(elems.is_empty(None));
        assert!(elems.is_empty(ElementCategory::AnimeTitle));
        assert!(elems.count(None) == 0);
        assert!(elems.count(ElementCategory::AnimeTitle) == 0);
    }

    #[test]
    fn anitomy_elements_get_good_input() {
        let elems = Anitomy::new()
            .parse(BLACK_BULLET_FILENAME)
            .expect("successfully parse filename");
        assert!(elems.count(ElementCategory::AnimeTitle) == 1);
        assert_eq!(elems.get(ElementCategory::AnimeTitle), Some("Black Bullet"));
    }

    #[test]
    fn anitomy_elements_get_bad_input() {
        let elems = Anitomy::new()
            .parse("")
            .expect_err("fail to parse filename");
        assert!(elems.count(ElementCategory::AnimeTitle) == 0);
        assert_eq!(elems.get(ElementCategory::AnimeTitle), None);
    }

    #[test]
    fn anitomy_elements_get_all_good_input() {
        let elems = Anitomy::new()
            .parse(BLACK_BULLET_FILENAME)
            .expect("successfully parse filename");
        assert!(elems.count(ElementCategory::EpisodeNumber) == 2);
        assert_eq!(elems.get_all(ElementCategory::EpisodeNumber), ["11", "12"]);
    }

    #[test]
    fn anitomy_elements_get_all_bad_input() {
        let elems = Anitomy::new()
            .parse("")
            .expect_err("fail to parse filename");
        assert!(elems.count(ElementCategory::EpisodeNumber) == 0);
        assert_eq!(
            elems.get_all(ElementCategory::EpisodeNumber),
            Vec::<&str>::new()
        );
    }

    #[test]
    fn anitomy_elements_at() {
        let elems = Anitomy::new()
            .parse(BLACK_BULLET_FILENAME)
            .expect("successfully parse filename");
        let pair = &elems[0];
        assert_eq!(pair.category, ElementCategory::FileExtension);
        assert_eq!(pair.value, "mp4");
    }

    #[test]
    fn anitomy_options_allowed_delimiters() {
        let mut ani = Anitomy::new();
        let elems = ani
            .parse(TORADORA_FILENAME)
            .expect("successfully parse filename");
        assert!(elems.count(ElementCategory::AnimeTitle) == 1);
        assert_eq!(elems.get(ElementCategory::AnimeTitle), Some("Toradora!"));

        ani.set_options(Options::default().allow_delimiters(&[]));

        let elems = ani
            .parse(TORADORA_FILENAME)
            .expect("successfully parse filename");
        assert!(elems.count(ElementCategory::AnimeTitle) == 1);
        assert_eq!(elems.get(ElementCategory::AnimeTitle), Some("_Toradora!_"));
    }

    #[test]
    fn anitomy_options_ignored_strings() {
        let mut ani = Anitomy::new();
        let elems = ani
            .parse(TORADORA_FILENAME)
            .expect("successfully parse filename");
        assert!(elems.count(ElementCategory::EpisodeTitle) == 1);
        assert_eq!(
            elems.get(ElementCategory::EpisodeTitle),
            Some("Tiger and Dragon")
        );

        ani.set_options(Options::default().ignore_string("Dragon"));

        let elems = ani
            .parse(TORADORA_FILENAME)
            .expect("successfully parse filename");
        assert!(elems.count(ElementCategory::EpisodeTitle) == 1);
        assert_eq!(elems.get(ElementCategory::EpisodeTitle), Some("Tiger and"));
    }

    #[test]
    fn anitomy_options_parse_episode_number() {
        let mut ani = Anitomy::new();
        let elems = ani
            .parse(TORADORA_FILENAME)
            .expect("successfully parse filename");
        assert!(elems.count(ElementCategory::EpisodeNumber) == 1);

        ani.set_options(Options::default().parse_episode_number(false));

        let elems = ani
            .parse(TORADORA_FILENAME)
            .expect("successfully parse filename");
        assert!(elems.count(ElementCategory::EpisodeNumber) == 0);
    }

    #[test]
    fn anitomy_options_parse_episode_title() {
        let mut ani = Anitomy::new();
        let elems = ani
            .parse(TORADORA_FILENAME)
            .expect("successfully parse filename");
        assert!(elems.count(ElementCategory::EpisodeTitle) == 1);

        ani.set_options(Options::default().parse_episode_title(false));

        let elems = ani
            .parse(TORADORA_FILENAME)
            .expect("successfully parse filename");
        assert!(elems.count(ElementCategory::EpisodeTitle) == 0);
    }

    #[test]
    fn anitomy_options_parse_file_extension() {
        let mut ani = Anitomy::new();
        let elems = ani
            .parse(TORADORA_FILENAME)
            .expect("successfully parse filename");
        assert!(elems.count(ElementCategory::FileExtension) == 1);

        ani.set_options(Options::default().parse_file_extension(false));

        let elems = ani
            .parse(TORADORA_FILENAME)
            .expect("successfully parse filename");
        assert!(elems.count(ElementCategory::FileExtension) == 0);
    }

    #[test]
    fn anitomy_options_parse_release_group() {
        let mut ani = Anitomy::new();
        let elems = ani
            .parse(TORADORA_FILENAME)
            .expect("successfully parse filename");
        assert!(elems.count(ElementCategory::ReleaseGroup) == 1);

        ani.set_options(Options::default().parse_release_group(false));

        let elems = ani
            .parse(TORADORA_FILENAME)
            .expect("successfully parse filename");
        assert!(elems.count(ElementCategory::ReleaseGroup) == 0);
    }
}
