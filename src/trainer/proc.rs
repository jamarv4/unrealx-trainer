#![allow(non_snake_case)]

use bindings::Windows::Win32::{
    Foundation::{CloseHandle, HANDLE, HWND, PSTR},
    System::Diagnostics::{
        Debug::GetLastError,
        ToolHelp::{
            Module32FirstW, Module32NextW, CREATE_TOOLHELP_SNAPSHOT_FLAGS,
            MODULEENTRY32W,
        },
    },
};
use bindings::Windows::Win32::{
    System::Diagnostics::ToolHelp::CreateToolhelp32Snapshot,
    UI::WindowsAndMessaging::{FindWindowA, GetWindowThreadProcessId},
};

use std::*;

const GAME_WINDOW_NAMES: [&str; 3] = ["Unreal", "Unreal Tournament", "Unreal Tournament 2004"];
const BASE_MODULE_NAME: &str = "Engine.dll";

#[derive(Debug)]
pub struct Game<'a> {
    pub name: &'a str,
    pub pid: u32,
    pub handle: HANDLE,
    pub module_address: *mut u8,
}

impl Drop for Game<'_> {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.handle);
        }
    }
}

impl<'b> Game<'b> {
    fn new<'a>(handle: HANDLE, pid: u32, name: &'a str) -> Game<'a> {
        let module_address: *mut u8 = find_module_entry_address(handle);

        Game {
            name,
            pid,
            handle,
            module_address,
        }
    }
}

fn find_module_entry_address(handle: HANDLE) -> *mut u8 {
    let last_error;
    let mut module_entry;
    let mut base_address: *mut u8 = (0x00) as *mut u8;

    module_entry = MODULEENTRY32W::default();
    module_entry.dwSize = mem::size_of::<MODULEENTRY32W>() as u32;

    unsafe {
        if Module32FirstW(handle, &mut module_entry) == false {
            last_error = GetLastError();
            println!("No modules were found: {:?}", last_error);
            println!("{:?}", module_entry);
            process::exit(1);
        }

        loop {
            let sz_module = String::from_utf16(&module_entry.szModule).unwrap();
            // println!("Detected Module: {}", sz_module);

            if sz_module.contains(BASE_MODULE_NAME) {
                println!("Base module found: {}", &BASE_MODULE_NAME);
                println!("Base module address: {:?}", module_entry.modBaseAddr);

                base_address = module_entry.modBaseAddr;
                break;
            }

            if Module32NextW(handle, &mut module_entry) == false {
                println!("Could not find module");
                break;
            }
        }
    }

    return base_address as *mut u8;
}

fn find_window_process<'a>() -> Option<(u32, &'a str)> {
    let mut pid: u32 = 0;
    let mut name: &str = "";
    let mut window_handle: HWND;

    for window_name in GAME_WINDOW_NAMES.iter() {
        unsafe {
            window_handle = FindWindowA(PSTR::NULL, *window_name);
            GetWindowThreadProcessId(window_handle, &mut pid);
        }

        if pid != 0 {
            println!("Game Found: {:?}, PID: {:?}", &window_name, &pid);
            name = *window_name;
            break;
        }
    }

    if pid == 0 {
        println!("No running games were found.");
        process::exit(1);
    }

    Some((pid, name))
}

fn find_game_handle<'a>() -> Option<(HANDLE, u32, &'a str)> {
    let TH32CS_SNAPMODULE32 = CREATE_TOOLHELP_SNAPSHOT_FLAGS(10);
    let TH32CS_SNAPMODULE64 = CREATE_TOOLHELP_SNAPSHOT_FLAGS(0);
    let handle_snapshot32: HANDLE;
    let handle_snapshot64: HANDLE;
    let (pid, name) = find_window_process().unwrap();

    unsafe {
        handle_snapshot32 = CreateToolhelp32Snapshot(TH32CS_SNAPMODULE32, pid);
        handle_snapshot64 = CreateToolhelp32Snapshot(TH32CS_SNAPMODULE64, pid);
    }

    if handle_snapshot32 != HANDLE::NULL && handle_snapshot32 != HANDLE::INVALID {
        return Some((handle_snapshot32, pid, name));
    }

    if handle_snapshot64 != HANDLE::NULL && handle_snapshot64 != HANDLE::INVALID {
        return Some((handle_snapshot64, pid, name));
    }

    println!("Could not find process handle.");
    process::exit(1);
}

pub fn find_game<'a>() -> Game<'a> {
    let (handle, pid, name): (HANDLE, u32, &str) = find_game_handle().unwrap();
    Game::new(handle, pid, name)
}
