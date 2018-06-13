#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use std::ffi::CStr;
use std::os::raw::c_char;

pub unsafe fn raw_into_string(raw_string: *const c_char) -> String {
    CStr::from_ptr(raw_string).to_string_lossy().into_owned()
}

pub type element_category_t = i32;
pub const kElementIterateFirst: element_category_t = 0;
pub const kElementAnimeSeason: element_category_t = kElementIterateFirst;
pub const kElementAnimeSeasonPrefix: element_category_t = 1;
pub const kElementAnimeTitle: element_category_t = 2;
pub const kElementAnimeType: element_category_t = 3;
pub const kElementAnimeYear: element_category_t = 4;
pub const kElementAudioTerm: element_category_t = 5;
pub const kElementDeviceCompatibility: element_category_t = 6;
pub const kElementEpisodeNumber: element_category_t = 7;
pub const kElementEpisodeNumberAlt: element_category_t = 8;
pub const kElementEpisodePrefix: element_category_t = 9;
pub const kElementEpisodeTitle: element_category_t = 10;
pub const kElementFileChecksum: element_category_t = 11;
pub const kElementFileExtension: element_category_t = 12;
pub const kElementFileName: element_category_t = 13;
pub const kElementLanguage: element_category_t = 14;
pub const kElementOther: element_category_t = 15;
pub const kElementReleaseGroup: element_category_t = 16;
pub const kElementReleaseInformation: element_category_t = 17;
pub const kElementReleaseVersion: element_category_t = 18;
pub const kElementSource: element_category_t = 19;
pub const kElementSubtitles: element_category_t = 20;
pub const kElementVideoResolution: element_category_t = 21;
pub const kElementVideoTerm: element_category_t = 22;
pub const kElementVolumeNumber: element_category_t = 23;
pub const kElementVolumePrefix: element_category_t = 24;
pub const kElementIterateLast: element_category_t = 25;
pub const kElementUnknown: element_category_t = kElementIterateLast;

extern "C" {
    pub fn string_free(string: *mut c_char);
}

#[repr(C)]
pub struct string_array_t {
    _unused: [u8; 0],
}

extern "C" {
    pub fn string_array_size(array: *const string_array_t) -> usize;
    pub fn string_array_at(array: *const string_array_t, pos: usize) -> *const c_char;
    pub fn string_array_free(array: *mut string_array_t);
}

#[repr(C)]
pub struct element_pair_t {
    pub category: element_category_t,
    pub value: *mut c_char,
}

#[repr(C)]
pub struct elements_t {
    _unused: [u8; 0],
}

extern "C" {
    pub fn elements_empty(elements: *const elements_t) -> bool;
    pub fn elements_empty_category(
        elements: *const elements_t,
        category: element_category_t,
    ) -> bool;
    pub fn elements_count(elements: *const elements_t) -> usize;
    pub fn elements_count_category(
        elements: *const elements_t,
        category: element_category_t,
    ) -> usize;
    pub fn elements_at(elements: *const elements_t, pos: usize) -> element_pair_t;
    pub fn elements_get(elements: *const elements_t, category: element_category_t) -> *mut c_char;
    pub fn elements_get_all(
        elements: *const elements_t,
        category: element_category_t,
    ) -> *mut string_array_t;
}

#[repr(C)]
pub struct anitomy_t {
    _unused: [u8; 0],
}

extern "C" {
    pub fn anitomy_new() -> *mut anitomy_t;
    pub fn anitomy_parse(anitomy: *mut anitomy_t, filename: *const c_char) -> bool;
    pub fn anitomy_elements(anitomy: *const anitomy_t) -> *const elements_t;
    pub fn anitomy_destroy(anitomy: *mut anitomy_t);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    const BLACK_BULLET_FILENAME: &'static str =
        "[異域字幕組][漆黑的子彈][Black Bullet][11-12][1280x720][繁体].mp4";

    unsafe fn get_element(elems: *const elements_t, cat: element_category_t) -> String {
        let cstr = elements_get(elems, cat);
        let val = raw_into_string(cstr);
        string_free(cstr);
        val
    }

    #[test]
    fn anitomy_new_destroy() {
        unsafe {
            let ani = anitomy_new();
            assert!(!ani.is_null());
            anitomy_destroy(ani);
        }
    }

    #[test]
    fn anitomy_parse_good_input() {
        unsafe {
            let filename = CString::new(BLACK_BULLET_FILENAME).unwrap();
            let ani = anitomy_new();
            assert!(!ani.is_null());

            assert!(anitomy_parse(ani, filename.as_ptr()));

            anitomy_destroy(ani);
        }
    }

    #[test]
    fn anitomy_parse_bad_input() {
        unsafe {
            let filename = CString::new("").unwrap();
            let ani = anitomy_new();
            assert!(!ani.is_null());

            assert!(!anitomy_parse(ani, filename.as_ptr()));

            anitomy_destroy(ani);
        }
    }

    #[test]
    fn anitomy_elements_not_null() {
        unsafe {
            let ani = anitomy_new();
            assert!(!ani.is_null());

            assert!(!anitomy_elements(ani).is_null());

            anitomy_destroy(ani);
        }
    }

    #[test]
    fn anitomy_elements_empty_good_input() {
        unsafe {
            let filename = CString::new(BLACK_BULLET_FILENAME).unwrap();
            let ani = anitomy_new();
            assert!(!ani.is_null());

            assert!(anitomy_parse(ani, filename.as_ptr()));
            {
                let elems = anitomy_elements(ani);
                assert!(!elems.is_null());
                assert!(!elements_empty(elems));
                assert!(!elements_empty_category(elems, kElementAnimeTitle));
                assert!(elements_count(elems) > 0);
                assert!(elements_count_category(elems, kElementAnimeTitle) == 1);
            }

            anitomy_destroy(ani);
        }
    }

    #[test]
    fn anitomy_elements_empty_bad_input() {
        unsafe {
            let filename = CString::new("").unwrap();
            let ani = anitomy_new();
            assert!(!ani.is_null());

            assert!(!anitomy_parse(ani, filename.as_ptr()));
            {
                let elems = anitomy_elements(ani);
                assert!(!elems.is_null());
                assert!(elements_empty(elems));
                assert!(elements_empty_category(elems, kElementAnimeTitle));
                assert!(elements_count(elems) == 0);
                assert!(elements_count_category(elems, kElementAnimeTitle) == 0);
            }

            anitomy_destroy(ani);
        }
    }

    #[test]
    fn anitomy_elements_get_good_input() {
        unsafe {
            let filename = CString::new(BLACK_BULLET_FILENAME).unwrap();
            let ani = anitomy_new();
            assert!(!ani.is_null());

            assert!(anitomy_parse(ani, filename.as_ptr()));
            {
                let elems = anitomy_elements(ani);
                assert!(!elems.is_null());
                assert!(elements_count_category(elems, kElementAnimeTitle) == 1);
                assert_eq!(get_element(elems, kElementAnimeTitle), "Black Bullet");
            }

            anitomy_destroy(ani);
        }
    }

    #[test]
    fn anitomy_elements_get_bad_input() {
        unsafe {
            let filename = CString::new("").unwrap();
            let ani = anitomy_new();
            assert!(!ani.is_null());

            assert!(!anitomy_parse(ani, filename.as_ptr()));
            {
                let elems = anitomy_elements(ani);
                assert!(!elems.is_null());
                assert!(elements_count_category(elems, kElementAnimeTitle) == 0);
                assert_eq!(get_element(elems, kElementAnimeTitle), "");
            }

            anitomy_destroy(ani);
        }
    }

    #[test]
    fn anitomy_elements_get_all_good_input() {
        unsafe {
            let filename = CString::new(BLACK_BULLET_FILENAME).unwrap();
            let ani = anitomy_new();
            assert!(!ani.is_null());

            assert!(anitomy_parse(ani, filename.as_ptr()));
            {
                let elems = anitomy_elements(ani);
                assert!(!elems.is_null());
                assert!(elements_count_category(elems, kElementEpisodeNumber) == 2);
                assert_eq!(
                    {
                        let array = elements_get_all(elems, kElementEpisodeNumber);
                        assert!(!array.is_null());
                        let size = string_array_size(array);
                        assert!(size == 2);
                        let vals: Vec<_> = (0..size)
                            .map(|i| raw_into_string(string_array_at(array, i)))
                            .collect();
                        string_array_free(array);
                        vals
                    },
                    ["11", "12"]
                );
            }

            anitomy_destroy(ani);
        }
    }

    #[test]
    fn anitomy_elements_get_all_bad_input() {
        unsafe {
            let filename = CString::new("").unwrap();
            let ani = anitomy_new();
            assert!(!ani.is_null());

            assert!(!anitomy_parse(ani, filename.as_ptr()));
            {
                let elems = anitomy_elements(ani);
                assert!(!elems.is_null());
                assert!(elements_count_category(elems, kElementEpisodeNumber) == 0);
                let epnums = elements_get_all(elems, kElementEpisodeNumber);
                assert!(!epnums.is_null());
                assert!(string_array_size(epnums) == 0);
                string_array_free(epnums);
            }

            anitomy_destroy(ani);
        }
    }

    #[test]
    fn anitomy_elements_at() {
        unsafe {
            let filename = CString::new(BLACK_BULLET_FILENAME).unwrap();
            let ani = anitomy_new();
            assert!(!ani.is_null());

            assert!(anitomy_parse(ani, filename.as_ptr()));
            {
                let elems = anitomy_elements(ani);
                assert!(!elems.is_null());
                assert!(elements_count(elems) > 0);
                let pair = elements_at(elems, 0);
                assert_eq!(pair.category, kElementFileExtension);
                assert_eq!(
                    {
                        let value = raw_into_string(pair.value);
                        string_free(pair.value);
                        value
                    },
                    "mp4"
                );
            }

            anitomy_destroy(ani);
        }
    }
}
