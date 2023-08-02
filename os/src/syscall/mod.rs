use self::{
    fs::sys_write,
    process::{sys_exit, sys_get_time, sys_yeild},
};

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YEILD: usize = 124;
const SYSCALL_GET_TIME: usize = 169;

mod fs;
mod process;

pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    match syscall_id {
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_YEILD => sys_yeild(),
        // SYSCALL_GET_TIME => sys_get_time(args[0] as *mut TimeVal, 0),
        SYSCALL_GET_TIME => sys_get_time(),
        // SYSCALL_TASK_INFO => sys_task_info(args[0], args[1] as *mut TaskInfo),
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}
