use windows::Win32::Foundation::{GetLastError, HINSTANCE, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::Graphics::Gdi::HBRUSH;
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DestroyWindow, DispatchMessageW, GetMessageW, HCURSOR, HICON,
    MSG, PostQuitMessage, RegisterClassExW, TranslateMessage, WINDOW_EX_STYLE, WINDOW_STYLE,
    WM_CLOSE, WM_DESTROY, WNDCLASS_STYLES, WNDCLASSEXW,
};
use windows::core::PCWSTR;

// https://winprog.org/tutorial/simple_window.html
unsafe extern "system" fn wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_CLOSE => {
            unsafe {
                let _ = DestroyWindow(hwnd);
            };
            LRESULT(0)
        }
        WM_DESTROY => unsafe {
            PostQuitMessage(0);
            LRESULT(0)
        },
        _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}

pub struct Window {
    class_name: Vec<u16>,
    hwnd: Option<HWND>,
    wc: WNDCLASSEXW,
    msg: MSG,
}

impl Window {
    pub fn new(class_name: &str) -> Self {
        let mut class_name: Vec<u16> = class_name.encode_utf16().collect();
        class_name.push(0); // null terminator
        Self {
            class_name,
            hwnd: None,
            wc: WNDCLASSEXW::default(),
            msg: MSG::default(),
        }
    }

    pub fn register(&mut self) -> u16 {
        self.wc.cbSize = size_of_val(&self.wc) as u32;
        self.wc.style = WNDCLASS_STYLES(0);
        self.wc.lpfnWndProc = Some(wnd_proc);
        self.wc.cbClsExtra = 0;
        self.wc.cbWndExtra = 0;
        self.wc.hInstance = HINSTANCE::default();
        self.wc.hIcon = HICON::default();
        self.wc.hCursor = HCURSOR::default();
        self.wc.hbrBackground = HBRUSH::default();
        self.wc.lpszMenuName = PCWSTR::default();
        self.wc.lpszClassName = PCWSTR(self.class_name.as_ptr());
        self.wc.hIconSm = HICON::default();

        unsafe {
            let class_atom = RegisterClassExW(&self.wc);

            if class_atom == 0 {
                panic!("Window could not be registered: {:?}", GetLastError())
            }

            class_atom
        }
    }
    pub fn create(&mut self, class_atom: u16) -> isize {
        let hwnd = unsafe {
            CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                PCWSTR(class_atom as usize as *const u16), // Same as using macro MAKEINTATOM(class_atom)
                PCWSTR::default(),
                WINDOW_STYLE::default(),
                0,
                0,
                0,
                0,
                None,
                None,
                None,
                None,
            )
        };

        match hwnd {
            Ok(hwnd) => {
                self.hwnd = Some(hwnd);
                hwnd.0 as isize
            }
            Err(error) => panic!("Window could not be created: {:?}", error),
        }
    }

    pub fn message_loop(&mut self) {
        while unsafe { GetMessageW(&mut self.msg, self.hwnd, 0, 0).as_bool() } {
            unsafe {
                let _ = TranslateMessage(&mut self.msg).as_bool();
                DispatchMessageW(&mut self.msg);
            }
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        match self.hwnd {
            Some(hwnd) => unsafe {
                let _ = DestroyWindow(hwnd);
            },
            None => (),
        }
    }
}
