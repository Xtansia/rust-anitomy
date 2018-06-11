pub mod ffi;

use std::ffi::{CString, NulError};

#[derive(Debug)]
pub struct Elements {
    elements: *mut ffi::elements_t,
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
                    elements: Elements {
                        elements: elems,
                    },
                });
            }
        }
            
        Err(())
    }

    pub unsafe fn parse(&mut self, filename: &str) -> Result<bool, NulError> {
        let filename = CString::new(filename)?;
        Ok(ffi::anitomy_parse(self.anitomy, filename.as_ptr()))
    }

    pub unsafe fn elements(&mut self) -> &mut Elements {
      &mut self.elements
    }

    pub unsafe fn destroy(&mut self) {
        ffi::anitomy_destroy(self.anitomy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BLACK_BULLET_FILENAME: &'static str = "[異域字幕組][漆黑的子彈][Black Bullet][11-12][1280x720][繁体].mp4";

    #[test]
    fn anitomy_new_destroy() {
        unsafe {
          let mut ani = Anitomy::new().unwrap();
          ani.destroy();
        }
    }

    #[test]
    fn anitomy_parse_good_input_success() {
        unsafe {
          let mut ani = Anitomy::new().unwrap();
          let success = ani.parse(BLACK_BULLET_FILENAME).unwrap();
          assert!(success);
          ani.destroy();
        }
    }

    #[test]
    fn anitomy_parse_bad_input_fail() {
        unsafe {
          let mut ani = Anitomy::new().unwrap();
          let success = ani.parse("").unwrap();
          assert!(!success);
          ani.destroy();
        }
    }

    /*#[test]
    fn anitomy_get_elements_not_null() {
        unsafe {
          let ani = anitomy_new();
          assert!(!ani.is_null());
          let elems = anitomy_elements(ani);
          assert!(!elems.is_null());
          anitomy_destroy(ani);
        }
    }*/
}