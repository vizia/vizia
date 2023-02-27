use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

pub struct ParentWindow(pub *mut ::std::ffi::c_void);

#[cfg(target_os = "macos")]
unsafe impl HasRawWindowHandle for ParentWindow {
    fn raw_window_handle(&self) -> RawWindowHandle {
        use raw_window_handle::AppKitHandle;

        let mut handle = AppKitHandle::empty();
        handle.ns_view = self.0;

        RawWindowHandle::AppKit(handle)
    }
}

#[cfg(target_os = "windows")]
unsafe impl HasRawWindowHandle for ParentWindow {
    fn raw_window_handle(&self) -> RawWindowHandle {
        use raw_window_handle::Win32Handle;

        let mut handle = Win32Handle::empty();
        handle.hwnd = self.0;

        RawWindowHandle::Win32(handle)
    }
}

#[cfg(target_os = "linux")]
unsafe impl HasRawWindowHandle for ParentWindow {
    fn raw_window_handle(&self) -> RawWindowHandle {
        use raw_window_handle::XcbHandle;

        let mut handle = XcbHandle::empty();
        handle.window = self.0 as u32;

        RawWindowHandle::Xcb(handle)
    }
}
