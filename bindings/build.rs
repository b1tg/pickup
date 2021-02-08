fn main() {
    windows::build!(
        windows::win32::system_services::MB_OK
        windows::win32::windows_and_messaging::MessageBoxA
        windows::win32::system_services::VirtualAlloc
        windows::win32::data_exchange::OpenClipboard
        windows::win32::data_exchange::EmptyClipboard
        windows::win32::data_exchange::CloseClipboard
        windows::win32::data_exchange::SetClipboardData
        windows::win32::system_services::GlobalLock
        windows::win32::system_services::GlobalAlloc
        windows::win32::system_services::GlobalUnlock

        windows::win32::windows_and_messaging::HWND
        windows::win32::system_services::HANDLE
        windows::win32::shell::DROPFILES

        windows::win32::com::OleSetClipboard
        windows::win32::com::IDataObject
        // windows::BOOL
        // windows::TRUE
        // windows::FALSE
    );
}
