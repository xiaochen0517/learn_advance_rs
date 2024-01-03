use windows::{
    core::*, Win32::Foundation::*
    , Win32::UI::WindowsAndMessaging::*,
};
use windows::Win32::Graphics::Gdi::{BeginPaint, CreateSolidBrush, EndPaint, PAINTSTRUCT};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;

fn create_colorref(red: u8, green: u8, blue: u8) -> COLORREF {
    COLORREF((blue as u32) << 16 | (green as u32) << 8 | (red as u32))
}

const BUTTON_ID: i32 = 1000;

fn main() -> Result<()> {
    unsafe {
        let hinstance = GetModuleHandleW(None)?;
        debug_assert!(hinstance.0 != 0);
        let class_name = w!("MyWindowClass");
        let window_name = w!("Hello, Windows!");
        let window_background = CreateSolidBrush(create_colorref(0, 0, 255));
        let windows_class = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            style: CS_OWNDC | CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: hinstance.into(),
            hIcon: Default::default(),
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            hbrBackground: Default::default(),
            lpszMenuName: PCWSTR::null(),
            lpszClassName: class_name,
            hIconSm: Default::default(),
        };
        let atom = RegisterClassExW(&windows_class);
        debug_assert!(atom != 0);
        let hwnd = CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            class_name,
            window_name,
            WS_OVERLAPPEDWINDOW | WS_TABSTOP | WS_VISIBLE,
            500,
            500,
            640,
            480,
            None,
            None,
            hinstance,
            None,
        );
        let button = CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            w!("BUTTON"),
            w!("Click Me"),
            WS_TABSTOP | WS_VISIBLE | WS_CHILD,
            10, // x position
            10, // y position
            80, // button width
            25, // button height
            hwnd,
            HMENU(BUTTON_ID as isize),
            HINSTANCE(GetWindowLongPtrW(hwnd, GWLP_HINSTANCE)),
            None,
        );
        // ShowWindow(hwnd, SW_SHOW);
        let mut message = MSG::default();

        while GetMessageW(&mut message, None, 0, 0).into() {
            TranslateMessage(&message);
            DispatchMessageW(&message);
        }
    }
    Ok(())
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_PAINT => {
                println!("WM_PAINT");
                let mut ps = PAINTSTRUCT::default();
                let hdc = BeginPaint(window, &mut ps);
                // ValidateRect(window, &ps.rcPaint);
                EndPaint(window, &ps);
                LRESULT(0)
            }
            WM_DESTROY => {
                println!("WM_DESTROY");
                PostQuitMessage(0);
                LRESULT(0)
            }
            WM_DPICHANGED => {
                println!("WM_DPICHANGED");
                LRESULT(0)
            }
            WM_COMMAND => {
                println!("WM_COMMAND");
                let control_id = wparam.0 as i32;
                if control_id == BUTTON_ID {
                    println!("Button was clicked!");
                }
                LRESULT(0)
            }
            WM_SIZE => {
                println!("WM_SIZE");
                println!("wparam: {:?}", lparam);
                LRESULT(0)
            }
            _ => DefWindowProcW(window, message, wparam, lparam),
        }
    }
}