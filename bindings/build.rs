fn main() {
    windows::build! {
        Windows::Win32::Foundation::{HANDLE, HWND, PSTR, CloseHandle},
        Windows::Win32::System::Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory, GetLastError},
        Windows::Win32::UI::WindowsAndMessaging::{FindWindowA, GetWindowThreadProcessId},
        Windows::Win32::System::Diagnostics::ToolHelp::{PROCESSENTRY32, MODULEENTRY32, MODULEENTRY32W, Module32FirstW, Module32NextW, Process32First, Process32Next, CREATE_TOOLHELP_SNAPSHOT_FLAGS, CreateToolhelp32Snapshot},
        Windows::Win32::System::VirtualDosMachines::MODULEENTRY,
        Windows::Win32::System::Threading::{GetProcessId},
    };
}
