use std::{ptr::null_mut, sync::*};

use windows::Win32::System::Services::*;

pub const SERVICE_NAME: &str = "TodoSvc\0";

pub static mut _SERVICE_STATUS: Mutex<SERVICE_STATUS> = Mutex::new(SERVICE_STATUS {
    dwServiceType: SERVICE_WIN32,
    dwCurrentState: SERVICE_START_PENDING,
    dwControlsAccepted: SERVICE_ACCEPT_STOP | SERVICE_ACCEPT_SHUTDOWN,
    dwWin32ExitCode: 0,
    dwServiceSpecificExitCode: 0,
    dwCheckPoint: 0,
    dwWaitHint: 0,
});

pub static mut _SERVICE_STATUS_HANDLE: Mutex<SERVICE_STATUS_HANDLE> =
    Mutex::new(SERVICE_STATUS_HANDLE(null_mut()));
