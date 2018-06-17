pub extern crate anitomy_sys;

use std::ffi::CString;

pub use anitomy_sys as sys;
pub use sys::ElementCategory;
pub type Element = sys::ElementPair;

#[derive(Debug, Clone)]
pub struct Options {
    allowed_delimiters: Vec<char>,
    ignored_strings: Vec<String>,
    parse_episode_number: bool,
    parse_episode_title: bool,
    parse_file_extension: bool,
    parse_release_group: bool,
}

impl Options {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn allow_delimiter(&mut self, delimiter: char) -> &mut Self {
        self.allowed_delimiters.push(delimiter);
        self
    }

    pub fn allow_delimiters(&mut self, delimiters: &[char]) -> &mut Self {
        self.allowed_delimiters = delimiters.iter().cloned().collect();
        self
    }

    pub fn ignore_string<S: AsRef<str>>(&mut self, string: S) -> &mut Self {
        self.ignored_strings.push(string.as_ref().into());
        self
    }

    pub fn ignore_strings<S: AsRef<str>>(&mut self, strings: &[S]) -> &mut Self {
        self.ignored_strings = strings
            .iter()
            .map(AsRef::as_ref)
            .map(str::to_owned)
            .collect();
        self
    }

    pub fn parse_episode_number(&mut self, parse: bool) -> &mut Self {
        self.parse_episode_number = parse;
        self
    }

    pub fn parse_episode_title(&mut self, parse: bool) -> &mut Self {
        self.parse_episode_title = parse;
        self
    }

    pub fn parse_file_extension(&mut self, parse: bool) -> &mut Self {
        self.parse_file_extension = parse;
        self
    }

    pub fn parse_release_group(&mut self, parse: bool) -> &mut Self {
        self.parse_release_group = parse;
        self
    }
}

impl Default for Options {
    fn default() -> Self {
        Self {
            allowed_delimiters: vec![' ', '_', '.', '&', '+', ',', '|'],
            ignored_strings: Vec::new(),
            parse_episode_number: true,
            parse_episode_title: true,
            parse_file_extension: true,
            parse_release_group: true,
        }
    }
}

#[derive(Debug, Clone)]
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

    pub fn is_empty<C: Into<Option<ElementCategory>>>(&self, category: C) -> bool {
        match category.into() {
            Some(category) => !self.elements.iter().any(|elem| elem.category == category),
            None => self.elements.is_empty(),
        }
    }

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

    pub fn get(&self, category: ElementCategory) -> Option<&str> {
        self.elements
            .iter()
            .find(|elem| elem.category == category)
            .map(|elem| elem.value.as_str())
    }

    pub fn get_all(&self, category: ElementCategory) -> Vec<&str> {
        self.elements
            .iter()
            .filter(|elem| elem.category == category)
            .map(|elem| elem.value.as_str())
            .collect()
    }
}

impl std::ops::Deref for Elements {
    type Target = [Element];

    #[inline]
    fn deref(&self) -> &[Element] {
        &self.elements
    }
}

pub struct Anitomy {
    anitomy: sys::Anitomy,
}

impl Anitomy {
    pub fn new() -> Self {
        Self {
            anitomy: unsafe { sys::Anitomy::new() },
        }
    }

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

        ani.set_options(Options::new().allow_delimiters(&[]));

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

        ani.set_options(Options::new().ignore_string("Dragon"));

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

        ani.set_options(Options::new().parse_episode_number(false));

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

        ani.set_options(Options::new().parse_episode_title(false));

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

        ani.set_options(Options::new().parse_file_extension(false));

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

        ani.set_options(Options::new().parse_release_group(false));

        let elems = ani
            .parse(TORADORA_FILENAME)
            .expect("successfully parse filename");
        assert!(elems.count(ElementCategory::ReleaseGroup) == 0);
    }
}
