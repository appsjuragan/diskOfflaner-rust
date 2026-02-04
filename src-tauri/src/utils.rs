use std::mem;
use std::ptr;

#[allow(clippy::borrow_as_ptr)]
#[allow(clippy::ptr_as_ptr)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::ref_as_ptr)]
pub fn is_elevated() -> bool {
    #[cfg(windows)]
    {
        use winapi::um::processthreadsapi::{GetCurrentProcess, OpenProcessToken};
        use winapi::um::securitybaseapi::GetTokenInformation;
        use winapi::um::winnt::{TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY};
        unsafe {
            let mut token_handle = ptr::null_mut();
            if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token_handle) == 0 {
                return false;
            }
            let mut elevation: TOKEN_ELEVATION = mem::zeroed();
            let mut return_length = 0u32;
            let result = GetTokenInformation(
                token_handle,
                TokenElevation,
                &mut elevation as *mut _ as *mut _,
                mem::size_of::<TOKEN_ELEVATION>() as u32,
                &mut return_length,
            );
            winapi::um::handleapi::CloseHandle(token_handle);
            if result == 0 {
                return false;
            }
            elevation.TokenIsElevated != 0
        }
    }
    #[cfg(unix)]
    {
        unsafe { libc::geteuid() == 0 }
    }
    #[cfg(not(any(windows, unix)))]
    {
        false
    }
}
