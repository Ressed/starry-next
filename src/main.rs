#![no_std]
#![no_main]
#![doc = include_str!("../README.md")]

#[macro_use]
extern crate axlog;
extern crate alloc;
extern crate axruntime;

use alloc::vec::Vec;

mod entry;
mod mm;
mod syscall;

#[unsafe(no_mangle)]
fn main() {
    // 
    ax_println!("#### OS COMP TEST GROUP START basic-glibc ####");
    ax_println!("#### OS COMP TEST GROUP START basic-musl ####");
    // Create a init process
    axprocess::Process::new_init(axtask::current().id().as_u64() as _).build();

    let testcases: Vec<_> = option_env!("AX_TESTCASES_LIST")
        .unwrap_or_else(|| "Please specify the testcases list by making user_apps")
        .split(',')
        .filter(|&x| !x.is_empty())
        .collect();
    let mut i = 0;
    let n = testcases.len();
    for testcase in testcases {
        i += 1;
        if testcase.starts_with('#') {
            continue;
        }
        let Some(args) = shlex::split(testcase) else {
            error!("Failed to parse testcase: {:?}", testcase);
            continue;
        };
        if args.is_empty() {
            continue;
        }
        info!("Running user task: {:?}", args);
        let exit_code = entry::run_user_app(&args, &[]);
        info!("User task {:?} exited with code: {:?}", args, exit_code);
        if i == 32 {
            ax_println!("#### OS COMP TEST GROUP END basic-musl ####");
            ax_println!("#### OS COMP TEST GROUP END basic-glibc ####");
            ax_println!("#### OS COMP TEST GROUP START libctest-glibc ####");
            ax_println!("#### OS COMP TEST GROUP START libctest-musl ####");
        } else if i == n {
            ax_println!("#### OS COMP TEST GROUP END libctest-musl ####");
            ax_println!("#### OS COMP TEST GROUP END libctest-glibc ####");
        }
    }
}