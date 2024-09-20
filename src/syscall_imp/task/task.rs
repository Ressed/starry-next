use arceos_posix_api::{self as api};

use axtask::{current, TaskExtRef};
use num_enum::TryFromPrimitive;

use crate::syscall_body;

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(i32)]
/// ARCH_PRCTL codes
///
/// It is only avaliable on x86_64, and is not convenient
/// to generate automatically via c_to_rust binding.
enum ArchPrctlCode {
    /// Set the GS segment base
    ArchSetGs = 0x1001,
    /// Set the FS segment base
    ArchSetFs = 0x1002,
    /// Get the FS segment base
    ArchGetFs = 0x1003,
    /// Get the GS segment base
    ArchGetGs = 0x1004,
    /// The setting of the flag manipulated by ARCH_SET_CPUID
    ArchGetCpuid = 0x1011,
    /// Enable (addr != 0) or disable (addr == 0) the cpuid instruction for the calling thread.
    ArchSetCpuid = 0x1012,
}

pub(crate) fn sys_getpid() -> i32 {
    api::sys_getpid()
}

pub(crate) fn sys_exit(status: i32) -> ! {
    let curr = current();
    let clear_child_tid = curr.task_ext().clear_child_tid() as *mut i32;
    if !clear_child_tid.is_null() {
        // TODO: check whether the address is valid
        unsafe {
            // TODO: Encapsulate all operations that access user-mode memory into a unified function
            *(clear_child_tid) = 0;
        }
        // TODO: wake up threads, which are blocked by futex, and waiting for the address pointed by clear_child_tid
    }
    axtask::exit(status);
}

pub(crate) fn sys_exit_group(status: i32) -> ! {
    warn!("Temporarily replace sys_exit_group with sys_exit");
    axtask::exit(status);
}

/// To set the clear_child_tid field in the task extended data.
///
/// The set_tid_address() always succeeds
pub(crate) fn sys_set_tid_address(tid_ptd: *const i32) -> isize {
    syscall_body!(sys_set_tid_address, {
        let curr = current();
        curr.task_ext().set_clear_child_tid(tid_ptd as _);
        Ok(curr.id().as_u64() as isize)
    })
}

#[cfg(target_arch = "x86_64")]
pub(crate) fn sys_arch_prctl(code: i32, addr: *mut usize) -> isize {
    use axerrno::LinuxError;
    syscall_body!(sys_arch_prctl, {
        match ArchPrctlCode::try_from(code) {
            // TODO: check the legality of the address
            Ok(ArchPrctlCode::ArchSetFs) => {
                unsafe {
                    axhal::arch::write_thread_pointer(*addr);
                }
                Ok(0)
            }
            Ok(ArchPrctlCode::ArchGetFs) => {
                unsafe {
                    *addr = axhal::arch::read_thread_pointer();
                }
                Ok(0)
            }
            Ok(ArchPrctlCode::ArchSetGs) => {
                unsafe {
                    x86::msr::wrmsr(x86::msr::IA32_KERNEL_GSBASE, *addr as u64);
                }
                Ok(0)
            }
            Ok(ArchPrctlCode::ArchGetGs) => {
                unsafe {
                    *addr = x86::msr::rdmsr(x86::msr::IA32_KERNEL_GSBASE) as usize;
                }
                Ok(0)
            }
            _ => Err(LinuxError::ENOSYS),
        }
    })
}
