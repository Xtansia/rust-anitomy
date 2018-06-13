#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use std::ffi::CStr;
use std::os::raw::c_char;

#[inline]
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
    pub fn string_array_new() -> *mut string_array_t;
    pub fn string_array_size(array: *const string_array_t) -> usize;
    pub fn string_array_at(array: *const string_array_t, pos: usize) -> *const c_char;
    pub fn string_array_add(array: *mut string_array_t, value: *const c_char);
    pub fn string_array_free(array: *mut string_array_t);
}

#[repr(C)]
pub struct options_t {
    _unused: [u8; 0],
}

extern "C" {
    pub fn options_allowed_delimiters(options: *mut options_t, allowed_delimiters: *const c_char);
    pub fn options_ignored_strings(options: *mut options_t, ignored_strings: *const string_array_t);
    pub fn options_parse_episode_number(options: *mut options_t, parse_episode_number: bool);
    pub fn options_parse_episode_title(options: *mut options_t, parse_episode_title: bool);
    pub fn options_parse_file_extension(options: *mut options_t, parse_file_extension: bool);
    pub fn options_parse_release_group(option: *mut options_t, parse_release_group: bool);
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
    pub fn anitomy_options(anitomy: *mut anitomy_t) -> *mut options_t;
    pub fn anitomy_destroy(anitomy: *mut anitomy_t);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    const BLACK_BULLET_FILENAME: &'static str =
        "[異域字幕組][漆黑的子彈][Black Bullet][11-12][1280x720][繁体].mp4";
    const TORADORA_FILENAME: &'static str = "[TaigaSubs]_Toradora!_(2008)_-_01v2_-_Tiger_and_Dragon_[1280x720_H.264_FLAC][1234ABCD].mkv";

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

    #[test]
    fn anitomy_options_not_null() {
        unsafe {
            let ani = anitomy_new();
            assert!(!ani.is_null());

            assert!(!anitomy_options(ani).is_null());

            anitomy_destroy(ani);
        }
    }

    #[test]
    fn anitomy_options_allowed_delimiters() {
        unsafe {
            let filename = CString::new(TORADORA_FILENAME).unwrap();
            let ani = anitomy_new();
            assert!(!ani.is_null());

            assert!(anitomy_parse(ani, filename.as_ptr()));
            {
                let elems = anitomy_elements(ani);
                assert!(!elems.is_null());
                assert!(elements_count_category(elems, kElementAnimeTitle) == 1);
                assert_eq!(get_element(elems, kElementAnimeTitle), "Toradora!");
            }

            {
                let opts = anitomy_options(ani);
                assert!(!opts.is_null());
                let allowed_delimiters = CString::new("").unwrap();
                options_allowed_delimiters(opts, allowed_delimiters.as_ptr());
            }

            assert!(anitomy_parse(ani, filename.as_ptr()));
            {
                let elems = anitomy_elements(ani);
                assert!(!elems.is_null());
                assert!(elements_count_category(elems, kElementAnimeTitle) == 1);
                assert_eq!(get_element(elems, kElementAnimeTitle), "_Toradora!_");
            }

            anitomy_destroy(ani);
        }
    }

    #[test]
    fn anitomy_options_ignored_strings() {
        unsafe {
            let filename = CString::new(TORADORA_FILENAME).unwrap();
            let ani = anitomy_new();
            assert!(!ani.is_null());

            assert!(anitomy_parse(ani, filename.as_ptr()));
            {
                let elems = anitomy_elements(ani);
                assert!(!elems.is_null());
                assert!(elements_count_category(elems, kElementEpisodeTitle) == 1);
                assert_eq!(get_element(elems, kElementEpisodeTitle), "Tiger and Dragon");
            }

            {
                let opts = anitomy_options(ani);
                assert!(!opts.is_null());
                let ignored_strings = string_array_new();
                assert!(!ignored_strings.is_null());
                let string = CString::new("Dragon").unwrap();
                string_array_add(ignored_strings, string.as_ptr());
                options_ignored_strings(opts, ignored_strings);
                string_array_free(ignored_strings);
            }

            assert!(anitomy_parse(ani, filename.as_ptr()));
            {
                let elems = anitomy_elements(ani);
                assert!(!elems.is_null());
                assert!(elements_count_category(elems, kElementEpisodeTitle) == 1);
                assert_eq!(get_element(elems, kElementEpisodeTitle), "Tiger and");
            }

            anitomy_destroy(ani);
        }
    }

    #[test]
    fn anitomy_options_parse_episode_number() {
        unsafe {
            let filename = CString::new(TORADORA_FILENAME).unwrap();
            let ani = anitomy_new();
            assert!(!ani.is_null());

            assert!(anitomy_parse(ani, filename.as_ptr()));
            {
                let elems = anitomy_elements(ani);
                assert!(!elems.is_null());
                assert!(elements_count_category(elems, kElementEpisodeNumber) == 1);
            }

            {
                let opts = anitomy_options(ani);
                assert!(!opts.is_null());
                options_parse_episode_number(opts, false);
            }

            assert!(anitomy_parse(ani, filename.as_ptr()));
            {
                let elems = anitomy_elements(ani);
                assert!(!elems.is_null());
                assert!(elements_count_category(elems, kElementEpisodeNumber) == 0);
            }

            anitomy_destroy(ani);
        }
    }

    #[test]
    fn anitomy_options_parse_episode_title() {
        unsafe {
            let filename = CString::new(TORADORA_FILENAME).unwrap();
            let ani = anitomy_new();
            assert!(!ani.is_null());

            assert!(anitomy_parse(ani, filename.as_ptr()));
            {
                let elems = anitomy_elements(ani);
                assert!(!elems.is_null());
                assert!(elements_count_category(elems, kElementEpisodeTitle) == 1);
            }

            {
                let opts = anitomy_options(ani);
                assert!(!opts.is_null());
                options_parse_episode_title(opts, false);
            }

            assert!(anitomy_parse(ani, filename.as_ptr()));
            {
                let elems = anitomy_elements(ani);
                assert!(!elems.is_null());
                assert!(elements_count_category(elems, kElementEpisodeTitle) == 0);
            }

            anitomy_destroy(ani);
        }
    }

    #[test]
    fn anitomy_options_parse_file_extension() {
        unsafe {
            let filename = CString::new(TORADORA_FILENAME).unwrap();
            let ani = anitomy_new();
            assert!(!ani.is_null());

            assert!(anitomy_parse(ani, filename.as_ptr()));
            {
                let elems = anitomy_elements(ani);
                assert!(!elems.is_null());
                assert!(elements_count_category(elems, kElementFileExtension) == 1);
            }

            {
                let opts = anitomy_options(ani);
                assert!(!opts.is_null());
                options_parse_file_extension(opts, false);
            }

            assert!(anitomy_parse(ani, filename.as_ptr()));
            {
                let elems = anitomy_elements(ani);
                assert!(!elems.is_null());
                assert!(elements_count_category(elems, kElementFileExtension) == 0);
            }

            anitomy_destroy(ani);
        }
    }

    #[test]
    fn anitomy_options_parse_release_group() {
        unsafe {
            let filename = CString::new(TORADORA_FILENAME).unwrap();
            let ani = anitomy_new();
            assert!(!ani.is_null());

            assert!(anitomy_parse(ani, filename.as_ptr()));
            {
                let elems = anitomy_elements(ani);
                assert!(!elems.is_null());
                assert!(elements_count_category(elems, kElementReleaseGroup) == 1);
            }

            {
                let opts = anitomy_options(ani);
                assert!(!opts.is_null());
                options_parse_release_group(opts, false);
            }

            assert!(anitomy_parse(ani, filename.as_ptr()));
            {
                let elems = anitomy_elements(ani);
                assert!(!elems.is_null());
                assert!(elements_count_category(elems, kElementReleaseGroup) == 0);
            }

            anitomy_destroy(ani);
        }
    }
}
