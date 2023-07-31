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
/// 批处理系统加载程序到主存中
mod batch;
/// 用于同步全局变量
mod sync;
/// 处理系统调用
mod syscall;
/// 切换用户态/内核态
mod trap;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("[kernel] Hello, world!");
    trap::init();
    batch::init();
    batch::run_next_app();
}

/// 将操作系统的bss段清零
fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }

    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) })
}
