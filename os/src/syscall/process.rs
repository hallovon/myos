use crate::{
    task::{exit_current_and_run_next, 
        suspend_current_and_run_next, 
        TASK_MANAGER},
    timer::{get_time_us, TimeVal},
};

pub fn sys_exit(xstate: i32) -> ! {
    println!("[kernel] Application exited with code {}", xstate);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

pub fn sys_yeild() -> isize {
    suspend_current_and_run_next();
    0
}

// pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
//     let total = get_time_us();
//     unsafe {
//         *ts = TimeVal {
//             sec: total / 1_000_000,
//             usec: total % 1_000_000,
//         };
//     }
//     0
// }

pub fn sys_get_time() -> isize {
    get_time_us() as isize
}
