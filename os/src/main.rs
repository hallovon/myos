#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::arch::global_asm;

mod lang_item;
/// rustsbi模块，与硬件交互
mod sbi;
/// 处理屏幕信息输出
#[macro_use]
mod console;
/// 全局配置
mod config;
/// 加载app
mod loader;
/// 用于同步全局变量
mod sync;
/// 处理系统调用
mod syscall;
/// 任务管理
mod task;
/// 定时器
mod timer;
/// 切换用户态/内核态
mod trap;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("[kernel] Hello, world!");
    trap::init();
    loader::load_apps();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    task::run_first_task();
    panic!("Unreachable in rust_main!");
}

/// 将操作系统的bss段清零
fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }

    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) })
}
