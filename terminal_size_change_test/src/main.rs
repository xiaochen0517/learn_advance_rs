extern crate kernel32;
extern crate winapi;

use std::ptr;
use winapi::um::wincontypes::{COORD, SMALL_RECT};
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::winbase::STD_OUTPUT_HANDLE;
use winapi::um::processenv::GetStdHandle;
use winapi::um::wincon::{SetConsoleScreenBufferSize, SetConsoleWindowInfo};

fn main() {
    unsafe {
        // 获取标准输出句柄
        let h_console = GetStdHandle(STD_OUTPUT_HANDLE);
        if h_console == INVALID_HANDLE_VALUE {
            eprintln!("Failed to get standard output handle");
            return;
        }

        // 设置窗口缓冲区大小
        let buffer_size = COORD { X: 120, Y: 30 }; // 设置缓冲区大小为 120x30
        if SetConsoleScreenBufferSize(h_console, buffer_size) == 0 {
            eprintln!("Failed to set console screen buffer size");
            return;
        }

        // 设置窗口大小
        let rect = SMALL_RECT { Left: 0, Top: 0, Right: 119, Bottom: 29 }; // 设置窗口大小为 120x30
        if SetConsoleWindowInfo(h_console, 1, &rect) == 0 {
            eprintln!("Failed to set console window info");
            return;
        }

        println!("Terminal size adjusted to 120x30");
    }
}
