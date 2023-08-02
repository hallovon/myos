use crate::config::MAX_SYSCALL_NUM;

use super::context::TaskContext;

#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    UnInit,  // 未初始化
    Ready,   // 准备运行
    Running, // 正在运行
    Exited,  // 已退出
}

#[derive(Clone, Copy)]
pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
}

pub struct TaskInfo {
    id: usize,
    status: TaskStatus,
    call: [SyscallInfo; MAX_SYSCALL_NUM],
    time: usize,
}

struct SyscallInfo {
    id: usize,
    times: usize,
}
