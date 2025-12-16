use std::ptr::copy_nonoverlapping;



pub struct StringEntry {
    length: usize,
    data : *const u8,
}

impl StringEntry {
    pub fn new(string: &str) -> StringEntry {
        use std::alloc::*;
        let layout = Layout::array::<u8>(string.len() + 1).expect("Invalid layout for string");
        let ptr = unsafe {
            alloc(layout)
        };
        if ptr.is_null() {
            handle_alloc_error(layout);
        }
        unsafe {
            copy_nonoverlapping(string.as_ptr(), ptr, string.len());
            // Null terminate string to make it compatible with C
            *ptr.add(string.len()) = 0;
        }
        StringEntry { length: string.len(), data: ptr }
    }

    pub fn as_str(&self) -> &'static str {
        unsafe {
            let slice = std::slice::from_raw_parts(self.data, self.length);
            std::str::from_utf8_unchecked(slice)
        }
    }
}