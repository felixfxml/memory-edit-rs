extern crate process_memory;

use std::process::Command;

use winapi::shared::minwindef::{BOOL, DWORD, FALSE, MAX_PATH, TRUE};
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::processthreadsapi::{GetPriorityClass, OpenProcess};
use winapi::um::tlhelp32::{CreateToolhelp32Snapshot, LPMODULEENTRY32, LPPROCESSENTRY32, LPTHREADENTRY32, MAX_MODULE_NAME32, Module32First, Module32Next, MODULEENTRY32, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPMODULE, TH32CS_SNAPPROCESS, TH32CS_SNAPTHREAD, Thread32First, Thread32Next, THREADENTRY32};
use winapi::um::winnt::{CHAR, HANDLE, PROCESS_ALL_ACCESS};

pub fn run() {
    if std::env::consts::OS == "windows" {
        get_process_list_windows();
    }
    if std::env::consts::OS == "linux" {
        get_process_list_linux();
    }
}

fn get_process_list_linux() {
    //TODO: linux support
    let output = Command::new("ls").output().expect("failed");
    println!("{}", cmd_output_to_string(output.stdout));
}

fn cmd_output_to_string(out: Vec<u8>) -> String {
    String::from_utf8(out).unwrap()
}

fn create_toolhelp32snaphot(dw_flags: DWORD, th32process_id: DWORD) -> HANDLE {
    unsafe { CreateToolhelp32Snapshot(dw_flags, th32process_id) }
}

fn process32first(h_snapshot: HANDLE, lppe: LPPROCESSENTRY32) -> BOOL {
    unsafe { Process32First(h_snapshot, lppe) }
}

fn process32next(h_snapshot: HANDLE, lppe: LPPROCESSENTRY32) -> BOOL {
    unsafe { Process32Next(h_snapshot, lppe) }
}

fn close_handle(h_object: HANDLE) -> BOOL {
    unsafe { CloseHandle(h_object) }
}

fn open_process(dw_desired_access: DWORD, b_inherit_handle: BOOL, dw_process_id: DWORD) -> HANDLE {
    unsafe { OpenProcess(dw_desired_access, b_inherit_handle, dw_process_id) }
}

fn get_priority_class(h_process: HANDLE) -> DWORD {
    unsafe { GetPriorityClass(h_process) }
}

fn get_process_list_windows() -> BOOL {
    //windows methods from https://docs.microsoft.com/en-us/windows/win32/toolhelp/taking-a-snapshot-and-viewing-processes
    let h_process_snap: HANDLE;
    let mut h_process: HANDLE;
    let mut pe32: PROCESSENTRY32 = unsafe { std::mem::zeroed() };
    let mut dw_priority_class: DWORD;

    // Take a snapshot of all processes in the system.
    h_process_snap = create_toolhelp32snaphot(TH32CS_SNAPPROCESS, 0);
    if h_process_snap == INVALID_HANDLE_VALUE {
        println!("CreateToolhelp32Snapshot (of processes)");
        return FALSE;
    }

    // Set the size of the structure before using it.
    pe32.dwSize = std::mem::size_of::<PROCESSENTRY32> as u32;

    // Retrieve information about the first process,
    // and exit if unsuccessful
    if process32first(h_process_snap, &mut pe32) == FALSE
    {
        println!("Process32First");// show cause of failure
        close_handle(h_process_snap);          // clean the snapshot object
        return FALSE;
    }

    // Now walk the snapshot of processes, and
    // display information about each process in turn
    while {
        println!("\n=====================================================");
        println!("PROCESS NAME: {}", path_name(pe32.szExeFile));
        println!("-------------------------------------------------------");

        // Retrieve the priority class.
        dw_priority_class = 0;
        h_process = open_process(PROCESS_ALL_ACCESS, FALSE, pe32.th32ProcessID);
        if h_process == INVALID_HANDLE_VALUE {
            println!("OpenProcess");
        } else {
            dw_priority_class = get_priority_class(h_process);
            if !dw_priority_class == 0 {
                println!("GetPriorityClass");
            }
            close_handle(h_process);
        }
        println!("Process ID = 0x{:x}", pe32.th32ProcessID);
        println!("Thread count = {}", pe32.cntThreads);
        println!("Parent process ID = 0x{:x}", pe32.th32ParentProcessID);
        println!("Priority base = {}", pe32.pcPriClassBase);

        if dw_priority_class == 1 {
            println!("Priority class = {}", dw_priority_class);
        }

        // List the modules and threads associated with this process
        list_process_modules(pe32.th32ProcessID);
        list_process_threads(pe32.th32ProcessID);

        process32next(h_process_snap, &mut pe32) == TRUE
    } {}
    close_handle(h_process_snap);
    TRUE
}

fn path_name(char_array: [CHAR; MAX_PATH]) -> String {
    String::from_utf8(char_array.iter().map(|&c| c as u8).collect()).unwrap()
}

fn mod_name(char_array: [CHAR; MAX_MODULE_NAME32 + 1]) -> String {
    String::from_utf8(char_array.iter().map(|&c| c as u8).collect()).unwrap()
}

fn module32first(h_snapshot: HANDLE, lpme: LPMODULEENTRY32) -> BOOL {
    unsafe { Module32First(h_snapshot, lpme) }
}

fn module32next(h_snapshot: HANDLE, lpme: LPMODULEENTRY32) -> BOOL {
    unsafe { Module32Next(h_snapshot, lpme) }
}

fn list_process_modules(dw_pid: DWORD) -> BOOL {
    let h_module_snap: HANDLE;
    let mut me32: MODULEENTRY32 = unsafe { std::mem::zeroed() };

    // Take a snapshot of all modules in the specified process.
    h_module_snap = create_toolhelp32snaphot(TH32CS_SNAPMODULE, dw_pid);
    if h_module_snap == INVALID_HANDLE_VALUE {
        println!("CreateToolhelp32Snapshot (of modules)");
        return FALSE;
    }

    // Set the size of the structure before using it.
    me32.dwSize = std::mem::size_of::<MODULEENTRY32> as u32;

    // Retrieve information about the first module,
    // and exit if unsuccessful
    if module32first(h_module_snap, &mut me32) == FALSE
    {
        println!("Module32First");  // show cause of failure
        close_handle(h_module_snap);           // clean the snapshot object
        return FALSE;
    }

    // Now walk the module list of the process,
    // and display information about each module
    while {
        println!("     MODULE NAME:     {}", mod_name(me32.szModule));
        println!("     Executable     = {}", path_name(me32.szExePath));
        println!("     Process ID     = 0x{:x}", me32.th32ProcessID);
        println!("     Ref count (g)  = 0x{:x}", me32.GlblcntUsage);
        println!("     Ref count (p)  = 0x{:x}", me32.ProccntUsage);
        println!("     Base address   = 0x{:x}", me32.modBaseAddr as u32);
        println!("     Base size      = {}\n", me32.modBaseSize);

        module32next(h_module_snap, &mut me32) == TRUE
    } {}

    close_handle(h_module_snap);
    TRUE
}

fn thread32first(h_snapshot: HANDLE, lpte: LPTHREADENTRY32) -> BOOL {
    unsafe { Thread32First(h_snapshot, lpte) }
}

fn thread32next(h_snapshot: HANDLE, lpte: LPTHREADENTRY32) -> BOOL {
    unsafe { Thread32Next(h_snapshot, lpte) }
}

fn list_process_threads(dw_owner_pid: DWORD) -> BOOL {
    let h_thread_snap: HANDLE;
    let mut te32: THREADENTRY32 = unsafe { std::mem::zeroed() };

    // Take a snapshot of all running threads
    h_thread_snap = create_toolhelp32snaphot(TH32CS_SNAPTHREAD, 0);
    if h_thread_snap == INVALID_HANDLE_VALUE {
        return FALSE;
    }

    // Fill in the size of the structure before using it.
    te32.dwSize = std::mem::size_of::<THREADENTRY32> as u32;

    // Retrieve information about the first thread,
    // and exit if unsuccessful
    if thread32first(h_thread_snap, &mut te32) == FALSE
    {
        println!("Thread32First"); // show cause of failure
        close_handle(h_thread_snap);          // clean the snapshot object
        return FALSE;
    }

    // Now walk the thread list of the system,
    // and display information about each thread
    // associated with the specified process
    while {
        if te32.th32OwnerProcessID == dw_owner_pid
        {
            println!("\n     THREAD ID      = 0x{:x}", te32.th32ThreadID);
            println!("     Base priority  = {}", te32.tpBasePri);
            println!("     Delta priority = {}\n", te32.tpDeltaPri);
        }
        thread32next(h_thread_snap, &mut te32) == TRUE
    } {}

    close_handle(h_thread_snap);
    TRUE
}
