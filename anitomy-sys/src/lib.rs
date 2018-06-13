pub mod ffi;

use std::ffi::{CString, NulError};

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

pub struct ElementPair {
    category: ElementCategory,
    value: String,
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

    pub unsafe fn at(&self, pos: usize) -> Option<ElementPair> {
        if pos < self.count(None) {
            let pair = ffi::elements_at(&self.elements, pos);
            let value = ffi::raw_into_string(pair.value);
            ffi::string_free(pair.value);
            Some(ElementPair {
                category: ElementCategory::from(pair.category),
                value: value,
            })
        } else {
            None
        }
    }

    pub unsafe fn get(&self, category: ElementCategory) -> String {
        let rawval = ffi::elements_get(&self.elements, category as ffi::element_category_t);
        let val = ffi::raw_into_string(rawval);
        ffi::string_free(rawval);
        val
    }

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

            assert!(
                ani.parse(BLACK_BULLET_FILENAME)
                    .expect("no nul chars in filename")
            );

            ani.destroy();
        }
    }

    #[test]
    fn anitomy_parse_bad_input() {
        unsafe {
            let mut ani = Anitomy::new().unwrap();

            assert!(!ani.parse("").expect("no nul chars in filename"));

            ani.destroy();
        }
    }

    #[test]
    fn anitomy_elements_empty_good_input() {
        unsafe {
            let mut ani = Anitomy::new().unwrap();

            assert!(
                ani.parse(BLACK_BULLET_FILENAME)
                    .expect("no nul chars in filename")
            );
            {
                let elems = ani.elements();
                assert!(!elems.empty(None));
                assert!(!elems.empty(Some(ElementCategory::AnimeTitle)));
                assert!(elems.count(None) > 0);
                assert!(elems.count(Some(ElementCategory::AnimeTitle)) == 1);
            }

            ani.destroy()
        }
    }

    #[test]
    fn anitomy_elements_empty_bad_input() {
        unsafe {
            let mut ani = Anitomy::new().unwrap();

            assert!(!ani.parse("").expect("no nul chars in filename"));
            {
                let elems = ani.elements();
                assert!(elems.empty(None));
                assert!(elems.empty(Some(ElementCategory::AnimeTitle)));
                assert!(elems.count(None) == 0);
                assert!(elems.count(Some(ElementCategory::AnimeTitle)) == 0);
            }

            ani.destroy()
        }
    }

    #[test]
    fn anitomy_elements_get_good_input() {
        unsafe {
            let mut ani = Anitomy::new().unwrap();

            assert!(
                ani.parse(BLACK_BULLET_FILENAME)
                    .expect("no nul chars in filename")
            );
            {
                let elems = ani.elements();
                assert!(elems.count(Some(ElementCategory::AnimeTitle)) == 1);
                assert_eq!(elems.get(ElementCategory::AnimeTitle), "Black Bullet");
            }

            ani.destroy()
        }
    }

    #[test]
    fn anitomy_elements_get_bad_input() {
        unsafe {
            let mut ani = Anitomy::new().unwrap();

            assert!(!ani.parse("").expect("no nul chars in filename"));
            {
                let elems = ani.elements();
                assert!(elems.count(Some(ElementCategory::AnimeTitle)) == 0);
                assert_eq!(elems.get(ElementCategory::AnimeTitle), "");
            }

            ani.destroy()
        }
    }

    #[test]
    fn anitomy_elements_get_all_good_input() {
        unsafe {
            let mut ani = Anitomy::new().unwrap();

            assert!(
                ani.parse(BLACK_BULLET_FILENAME)
                    .expect("no nul chars in filename")
            );
            {
                let elems = ani.elements();
                assert!(elems.count(Some(ElementCategory::EpisodeNumber)) == 2);
                assert_eq!(elems.get_all(ElementCategory::EpisodeNumber), ["11", "12"]);
            }

            ani.destroy()
        }
    }

    #[test]
    fn anitomy_elements_get_all_bad_input() {
        unsafe {
            let mut ani = Anitomy::new().unwrap();

            assert!(!ani.parse("").expect("no nul chars in filename"));
            {
                let elems = ani.elements();
                assert!(elems.count(Some(ElementCategory::EpisodeNumber)) == 0);
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
            let mut ani = Anitomy::new().unwrap();

            assert!(
                ani.parse(BLACK_BULLET_FILENAME)
                    .expect("no nul chars in filename")
            );
            {
                let elems = ani.elements();
                let pair = elems.at(0).expect("at least one element");
                assert_eq!(pair.category, ElementCategory::FileExtension);
                assert_eq!(pair.value, "mp4");
            }

            ani.destroy();
        }
    }
}
