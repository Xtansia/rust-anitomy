use std::os::raw::c_char;

#[repr(C)]
pub struct elements_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct anitomy_t {
    _unused: [u8; 0],
}

extern "C" {
    pub fn anitomy_new() -> *mut anitomy_t;
    pub fn anitomy_parse(anitomy: *mut anitomy_t, filename: *const c_char) -> bool;
    pub fn anitomy_elements(anitomy: *mut anitomy_t) -> *mut elements_t;
    pub fn anitomy_destroy(anitomy: *mut anitomy_t);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    const BLACK_BULLET_FILENAME: &'static str =
        "[異域字幕組][漆黑的子彈][Black Bullet][11-12][1280x720][繁体].mp4";

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
            let ani = anitomy_new();
            assert!(!ani.is_null());
            let filename = CString::new(BLACK_BULLET_FILENAME).unwrap();
            let success = anitomy_parse(ani, filename.as_ptr());
            assert!(success);
            anitomy_destroy(ani);
        }
    }

    #[test]
    fn anitomy_parse_bad_input() {
        unsafe {
            let ani = anitomy_new();
            assert!(!ani.is_null());
            let filename = CString::new("").unwrap();
            let success = anitomy_parse(ani, filename.as_ptr());
            assert!(!success);
            anitomy_destroy(ani);
        }
    }

    #[test]
    fn anitomy_elements_not_null() {
        unsafe {
            let ani = anitomy_new();
            assert!(!ani.is_null());
            let elems = anitomy_elements(ani);
            assert!(!elems.is_null());
            anitomy_destroy(ani);
        }
    }
}
