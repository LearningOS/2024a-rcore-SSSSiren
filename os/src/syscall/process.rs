//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM,
    task::{get_current_taskinfo, exit_current_and_run_next, suspend_current_and_run_next, TaskStatus},
    timer::{get_time_us, get_time_ms},
};


/// time
#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    /// normal
    pub sec: usize,
    /// miu time 
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    pub status: TaskStatus,
    /// The numbers of syscall called by task
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    pub time: usize,
}


/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");
    if _ti.is_null() {
        return -1
    }
    let current_taskptr = get_current_taskinfo();
    let current_task = unsafe {
        & *current_taskptr
    };
    let task_info = unsafe {
        &mut  *_ti
    };

    // 填充任务信息
    task_info.status = current_task.status;
    task_info.syscall_times.copy_from_slice(&current_task.syscall_times); // 拷贝系统调用次数
    task_info.time = get_time_ms() - current_task.time;
    0
}
