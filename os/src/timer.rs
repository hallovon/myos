use riscv::register::time;

use crate::{config::CLOCK_FREQ, sbi::set_timer};

const TICKS_PER_SEC: usize = 100;
const MICRO_PER_SEC: usize = 1_000_000;

#[repr(C)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// 获取mtime寄存器中时间值
pub fn get_time() -> usize {
    time::read()
}

/// 给cpu设置下一次时钟中断的时间点
pub fn set_next_trigger() {
    set_timer(get_time() + CLOCK_FREQ / TICKS_PER_SEC);
}

/// 计算当前时间（微秒）
pub fn get_time_us() -> usize {
    time::read() / (CLOCK_FREQ / MICRO_PER_SEC)
}
