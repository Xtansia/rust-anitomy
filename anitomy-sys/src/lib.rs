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

#[derive(Debug)]
pub struct Elements {
    elements: *const ffi::elements_t,
}

impl Elements {
    pub unsafe fn empty(&self, category: Option<ElementCategory>) -> bool {
        match category {
            Some(cat) => {
                ffi::elements_empty_category(self.elements, cat as ffi::element_category_t)
            }
            None => ffi::elements_empty(self.elements),
        }
    }

    pub unsafe fn count(&self, category: Option<ElementCategory>) -> usize {
        match category {
            Some(cat) => {
                ffi::elements_count_category(self.elements, cat as ffi::element_category_t)
            }
            None => ffi::elements_count(self.elements),
        }
    }
}

#[derive(Debug)]
pub struct Anitomy {
    anitomy: *mut ffi::anitomy_t,
    elements: Elements,
}

impl Anitomy {
    pub unsafe fn new() -> Result<Self, ()> {
        let ani = ffi::anitomy_new();

        if !ani.is_null() {
            let elems = ffi::anitomy_elements(ani);

            if !elems.is_null() {
                return Ok(Self {
                    anitomy: ani,
                    elements: Elements { elements: elems },
                });
            }
        }

        Err(())
    }

    pub unsafe fn parse(&mut self, filename: &str) -> Result<bool, NulError> {
        let filename = CString::new(filename)?;
        Ok(ffi::anitomy_parse(self.anitomy, filename.as_ptr()))
    }

    pub unsafe fn elements(&self) -> &Elements {
        &self.elements
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
}
