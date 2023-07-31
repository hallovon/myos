use core::arch::asm;

use lazy_static::lazy_static;

use crate::{sync::up::UPSafeCell, trap::context::TrapContext, sbi::shutdown};

const MAX_APP_NUM: usize = 16;
const APP_BASE_ADDRESS: usize = 0x80400000;
const APP_SIZE_LIMIT: usize = 0x20000;

/// 应用管理器
struct AppManager {
    /// app个数
    num_app: usize,
    /// 当前运行的app编号
    current_app: usize,
    /// app在内存中的地址
    app_start: [usize; MAX_APP_NUM + 1],
}

lazy_static! {
    static ref APP_MANAGER: UPSafeCell<AppManager> = unsafe {
        UPSafeCell::new( {
            extern "C" {
                fn _num_app();
            }
            // 获取_num_app在内存中的地址
            let num_app_ptr = _num_app as usize as *const usize;
            let num_app = num_app_ptr.read_volatile();
            let mut app_start: [usize; MAX_APP_NUM + 1] = [0; MAX_APP_NUM + 1];
            // 获取app内存起始地址
            let app_start_raw: &[usize] = core::slice::from_raw_parts(
                num_app_ptr.add(1), num_app + 1
            );

            app_start[..=num_app].copy_from_slice(app_start_raw);
            AppManager {
                num_app,
                current_app: 0,
                app_start,
            }
        })
    };
}

impl AppManager {
    /// 加载app
    unsafe fn load_app(&self, app_id: usize) {
        if app_id >= self.num_app {
            println!("All application completed!");
            shutdown(false)
        }
        println!("[kernel] Loading app_{}", app_id);
        // 清空app区域
        core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, APP_SIZE_LIMIT).fill(0);
        
        // 获取app_id号应用的内存地址
        let app_src = core::slice::from_raw_parts(
            self.app_start[app_id] as *const u8,
            self.app_start[app_id + 1] - self.app_start[app_id],
        );
        // 获取加载app的目的地址
        let app_dst = core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, app_src.len());
        // 将应用数据加载到指定位置
        app_dst.copy_from_slice(app_src);
        asm!("fence.i");
    }

    pub fn print_app_info(&self) {
        println!("[kernel] num_app = {}", self.num_app);
        for i in 0..self.num_app {
            println!(
                "[kernel] app_{} ({:#x}, {:#x})",
                i,
                self.app_start[i],
                self.app_start[i + 1]
            );
        }
    }

    pub fn get_current_app(&self) -> usize {
        self.current_app
    }

    pub fn move_to_next_app(&mut self) {
        self.current_app += 1;
    }
}

/// 初始化批处理系统
pub fn init() {
    print_app_info();
}

/// 输出应用详细信息
pub fn print_app_info() {
    APP_MANAGER.exclusive_access().print_app_info();
}

const USER_STACK_SIZE: usize = 4096 * 2;
const KERNEL_STACK_SIZE: usize = 4096 * 2;

/// app的内核栈
#[repr(align(4096))]
struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

/// app的用户栈
#[repr(align(4096))]
struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

static KERNEL_STACK: KernelStack = KernelStack {
    data: [0; KERNEL_STACK_SIZE],
};

static USER_STACK: UserStack = UserStack {
    data: [0; USER_STACK_SIZE],
};

impl KernelStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }

    pub fn push_context(&self, cx: TrapContext) -> &'static mut TrapContext {
        let cx_ptr = (self.get_sp() - core::mem::size_of::<TrapContext>()) as *mut TrapContext;

        unsafe {
            *cx_ptr = cx;
        }

        unsafe { cx_ptr.as_mut().unwrap() }
    }
}

impl UserStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }
}

pub fn run_next_app() -> ! {
    let mut app_manager = APP_MANAGER.exclusive_access();
    let current_app = app_manager.get_current_app();
    unsafe {
        app_manager.load_app(current_app);
    }
    app_manager.move_to_next_app();
    
    drop(app_manager);

    extern "C" {
        fn __restore(cx_addr: usize);
    }

    unsafe {
        __restore(KERNEL_STACK.push_context(
            TrapContext::app_init_context(
            APP_BASE_ADDRESS,
            USER_STACK.get_sp())
        ) as *const _ as usize)
    }

    panic!("Unreachable in batch::run_current_app!");
}
