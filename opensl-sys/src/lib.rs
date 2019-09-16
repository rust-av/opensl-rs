// TODO do w/out the unions?
//#![feature(untagged_unions)]

pub mod opensles;

#[cfg(test)]
mod tests {
    use std::ffi::CStr;
    #[test]
    fn version() {
    /*    println!("{}", unsafe {
            CStr::from_ptr(opus_get_version_string()).to_string_lossy()
        }); */
    }
}
