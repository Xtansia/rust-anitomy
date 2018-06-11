pub mod ffi;

use std::ffi::{CStr, CString, NulError};

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

#[repr(C)]
pub struct ElementPair {
    element_pair: ffi::element_pair_t,
}

impl ElementPair {
    pub unsafe fn category(&self) -> ElementCategory {
        ElementCategory::from(ffi::element_pair_category(&self.element_pair))
    }

    pub unsafe fn value(&self) -> String {
        let rawval = ffi::element_pair_value(&self.element_pair);
        let val = CStr::from_ptr(rawval).to_string_lossy().into_owned();
        ffi::string_free(rawval);
        val
    }
}

#[repr(C)]
pub struct Elements {
    elements: ffi::elements_t,
}

impl Elements {
    pub unsafe fn empty(&self, category: Option<ElementCategory>) -> bool {
        match category {
            Some(cat) => {
                ffi::elements_empty_category(&self.elements, cat as ffi::element_category_t)
            }
            None => ffi::elements_empty(&self.elements),
        }
    }

    pub unsafe fn count(&self, category: Option<ElementCategory>) -> usize {
        match category {
            Some(cat) => {
                ffi::elements_count_category(&self.elements, cat as ffi::element_category_t)
            }
            None => ffi::elements_count(&self.elements),
        }
    }

    pub unsafe fn at(&self, pos: usize) -> Option<&ElementPair> {
        if pos < self.count(None) {
            Some(&*(ffi::elements_at(&self.elements, pos) as *const ElementPair))
        } else {
            None
        }
    }

    pub unsafe fn get(&self, category: ElementCategory) -> String {
        let rawval = ffi::elements_get(&self.elements, category as ffi::element_category_t);
        let val = CStr::from_ptr(rawval).to_string_lossy().into_owned();
        ffi::string_free(rawval);
        val
    }

    pub unsafe fn get_all(&self, category: ElementCategory) -> Vec<String> {
        let rawvals = ffi::elements_get_all(&self.elements, category as ffi::element_category_t);
        let vals = (0..rawvals.size)
            .map(|i| *rawvals.data.offset(i as isize))
            .map(|c_str| CStr::from_ptr(c_str).to_string_lossy().into_owned())
            .collect();
        ffi::array_free(rawvals);
        vals
    }
}

#[derive(Debug)]
pub struct Anitomy {
    anitomy: *mut ffi::anitomy_t,
}

impl Anitomy {
    pub unsafe fn new() -> Result<Self, ()> {
        let ani = ffi::anitomy_new();

        if !ani.is_null() {
            Ok(Self { anitomy: ani })
        } else {
            Err(())
        }
    }

    pub unsafe fn parse(&mut self, filename: &str) -> Result<bool, NulError> {
        let filename = CString::new(filename)?;
        Ok(ffi::anitomy_parse(self.anitomy, filename.as_ptr()))
    }

    pub unsafe fn elements(&self) -> &Elements {
        &*(ffi::anitomy_elements(self.anitomy) as *const Elements)
    }

    pub unsafe fn destroy(&mut self) {
        ffi::anitomy_destroy(self.anitomy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BLACK_BULLET_FILENAME: &'static str =
        "[異域字幕組][漆黑的子彈][Black Bullet][11-12][1280x720][繁体].mp4";

    #[test]
    fn anitomy_new_destroy() {
        unsafe {
            let mut ani = Anitomy::new().unwrap();
            ani.destroy();
        }
    }

    #[test]
    fn anitomy_parse_good_input() {
        unsafe {
            let mut ani = Anitomy::new().unwrap();
            let success = ani.parse(BLACK_BULLET_FILENAME).unwrap();
            assert!(success);
            ani.destroy();
        }
    }

    #[test]
    fn anitomy_parse_bad_input() {
        unsafe {
            let mut ani = Anitomy::new().unwrap();
            let success = ani.parse("").unwrap();
            assert!(!success);
            ani.destroy();
        }
    }

    #[test]
    fn anitomy_elements_empty_good_input() {
        unsafe {
            let mut ani = Anitomy::new().unwrap();
            let success = ani.parse(BLACK_BULLET_FILENAME).unwrap();
            assert!(success);
            {
                let elems = ani.elements();
                let empty = elems.empty(None);
                assert!(!empty);
                let anititle_empty = elems.empty(Some(ElementCategory::AnimeTitle));
                assert!(!anititle_empty);
                let size = elems.count(None);
                assert!(size > 0);
                let anititle_count = elems.count(Some(ElementCategory::AnimeTitle));
                assert!(anititle_count == 1);
            }
            ani.destroy()
        }
    }

    #[test]
    fn anitomy_elements_empty_bad_input() {
        unsafe {
            let mut ani = Anitomy::new().unwrap();
            let success = ani.parse("").unwrap();
            assert!(!success);
            {
                let elems = ani.elements();
                let empty = elems.empty(None);
                assert!(empty);
                let anititle_empty = elems.empty(Some(ElementCategory::AnimeTitle));
                assert!(anititle_empty);
                let size = elems.count(None);
                assert!(size == 0);
                let anititle_count = elems.count(Some(ElementCategory::AnimeTitle));
                assert!(anititle_count == 0);
            }
            ani.destroy()
        }
    }

    #[test]
    fn anitomy_elements_get_good_input() {
        unsafe {
            let mut ani = Anitomy::new().unwrap();
            let success = ani.parse(BLACK_BULLET_FILENAME).unwrap();
            assert!(success);
            {
                let elems = ani.elements();
                let empty = elems.empty(None);
                assert!(!empty);
                let anititle_empty = elems.empty(Some(ElementCategory::AnimeTitle));
                assert!(!anititle_empty);
                let size = elems.count(None);
                assert!(size > 0);
                let anititle_count = elems.count(Some(ElementCategory::AnimeTitle));
                assert!(anititle_count == 1);
                let anititle = elems.get(ElementCategory::AnimeTitle);
                assert_eq!(anititle, "Black Bullet");
            }
            ani.destroy()
        }
    }

    #[test]
    fn anitomy_elements_get_bad_input() {
        unsafe {
            let mut ani = Anitomy::new().unwrap();
            let success = ani.parse("").unwrap();
            assert!(!success);
            {
                let elems = ani.elements();
                let empty = elems.empty(None);
                assert!(empty);
                let anititle_empty = elems.empty(Some(ElementCategory::AnimeTitle));
                assert!(anititle_empty);
                let size = elems.count(None);
                assert!(size == 0);
                let anititle_count = elems.count(Some(ElementCategory::AnimeTitle));
                assert!(anititle_count == 0);
                let anititle = elems.get(ElementCategory::AnimeTitle);
                assert_eq!(anititle, "");
            }
            ani.destroy()
        }
    }

    #[test]
    fn anitomy_elements_get_all_good_input() {
        unsafe {
            let mut ani = Anitomy::new().unwrap();
            let success = ani.parse(BLACK_BULLET_FILENAME).unwrap();
            assert!(success);
            {
                let elems = ani.elements();
                let empty = elems.empty(None);
                assert!(!empty);
                let epnums_empty = elems.empty(Some(ElementCategory::EpisodeNumber));
                assert!(!epnums_empty);
                let size = elems.count(None);
                assert!(size > 0);
                let epnums_count = elems.count(Some(ElementCategory::EpisodeNumber));
                assert!(epnums_count == 2);
                let epnums = elems.get_all(ElementCategory::EpisodeNumber);
                assert_eq!(epnums, ["11", "12"]);
            }
            ani.destroy()
        }
    }

    #[test]
    fn anitomy_elements_get_all_bad_input() {
        unsafe {
            let mut ani = Anitomy::new().unwrap();
            let success = ani.parse("").unwrap();
            assert!(!success);
            {
                let elems = ani.elements();
                let empty = elems.empty(None);
                assert!(empty);
                let epnums_empty = elems.empty(Some(ElementCategory::EpisodeNumber));
                assert!(epnums_empty);
                let size = elems.count(None);
                assert!(size == 0);
                let epnums_count = elems.count(Some(ElementCategory::EpisodeNumber));
                assert!(epnums_count == 0);
                let epnums = elems.get_all(ElementCategory::EpisodeNumber);
                assert_eq!(epnums, Vec::<String>::new());
            }
            ani.destroy()
        }
    }

    #[test]
    fn anitomy_elements_at() {
        unsafe {
            let mut ani = Anitomy::new().unwrap();
            let success = ani.parse(BLACK_BULLET_FILENAME).unwrap();
            assert!(success);
            {
                let elems = ani.elements();
                let empty = elems.empty(None);
                assert!(!empty);
                let size = elems.count(None);
                assert!(size > 0);
                let pair = elems.at(0).unwrap();
                let category = pair.category();
                let value = pair.value();
                assert_eq!(category, ElementCategory::FileExtension);
                assert_eq!(value, "mp4");
            }
            ani.destroy();
        }
    }
}
