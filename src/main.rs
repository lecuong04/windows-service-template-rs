#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(static_mut_refs)]
#![allow(unused_assignments)]

mod shared;
use shared::*;

use core::ffi::*;

use std::ptr::*;

use windows::core::*;
use windows::Win32::System::Services::*;

extern "system" fn service_control_handler(
    dwcontrol: u32,
    dweventtype: u32,
    lpeventdata: *mut c_void,
    lpcontext: *mut c_void,
) -> u32 {
    unsafe {
        match dwcontrol {
            SERVICE_CONTROL_SHUTDOWN | SERVICE_CONTROL_STOP => {
                _SERVICE_STATUS.dwCurrentState = SERVICE_STOPPED;
            }
            _ => {}
        }
        SetServiceStatus(_SERVICE_STATUS_HANDLE, &mut _SERVICE_STATUS).unwrap();
    }
    0
}

unsafe extern "system" fn service_main(argc: u32, argv: *mut PWSTR) {
    match RegisterServiceCtrlHandlerExW(
        PCWSTR(SERVICE_NAME.encode_utf16().collect::<Vec<u16>>().as_ptr()),
        Some(service_control_handler),
        None,
    ) {
        Ok(o) => {
            _SERVICE_STATUS_HANDLE = o;
            _SERVICE_STATUS.dwCurrentState = SERVICE_RUNNING;
            SetServiceStatus(o, &mut _SERVICE_STATUS).unwrap();
            todo!()
        }
        Err(_) => {}
    }
}

fn main() {
    let service_table: [SERVICE_TABLE_ENTRYW; 2] = [
        SERVICE_TABLE_ENTRYW {
            lpServiceName: PWSTR(
                SERVICE_NAME
                    .encode_utf16()
                    .collect::<Vec<u16>>()
                    .as_mut_ptr(),
            ),
            lpServiceProc: Some(service_main),
        },
        SERVICE_TABLE_ENTRYW {
            lpServiceName: PWSTR(null_mut()),
            lpServiceProc: None,
        },
    ];
    unsafe { StartServiceCtrlDispatcherW(service_table.as_ptr()).unwrap() };
}
