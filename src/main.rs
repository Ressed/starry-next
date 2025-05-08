#![no_std]
#![no_main]
#![doc = include_str!("../README.md")]

#[macro_use]
extern crate axlog;
extern crate alloc;

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

    let testcases = option_env!("AX_TESTCASES_LIST")
        .unwrap_or_else(|| "Please specify the testcases list by making user_apps")
        .split(',')
        .filter(|&x| !x.is_empty());
    let mut i = 0;
    for testcase in testcases {
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
        i += 1;
        if i == 32 {
            ax_println!("#### OS COMP TEST GROUP END basic-musl ####");
            ax_println!("#### OS COMP TEST GROUP END basic-glibc ####");
            ax_println!("#### OS COMP TEST GROUP START libctest-glibc ####");
            ax_println!("#### OS COMP TEST GROUP START libctest-musl ####");
        } else if i == 207 {
            ax_println!("#### OS COMP TEST GROUP END libctest-musl ####");
            ax_println!("#### OS COMP TEST GROUP END libctest-glibc ####");
        }
    }
}