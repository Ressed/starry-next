#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[unsafe(no_mangle)]
pub fn main() -> i32 {
    let filename = "test_rw.txt";
    let content = b"Hello, world!";
    let mut buf = [0u8; 32];

    // 打开文件（写模式），如果不存在则创建
    let fd = user_lib::open(filename, user_lib::OpenFlags::CREATE | user_lib::OpenFlags::WRONLY);
    if fd < 0 {
        println!("Failed to open file for writing");
        return -1;
    }
    let fd = fd as usize;

    // 写入1000次
    let start_write = user_lib::get_time_us();
    for _ in 0..1000 {
        let _ = user_lib::write(fd, content);
    }
    let end_write = user_lib::get_time_us();
    println!("Write 1000 times cost: {} us", end_write - start_write);

    user_lib::close(fd);

    // 打开文件（读模式）
    let fd = user_lib::open(filename, user_lib::OpenFlags::RDONLY);
    if fd < 0 {
        println!("Failed to open file for reading");
        return -1;
    }
    let fd = fd as usize;

    // 读1000次
    let start_read = user_lib::get_time_us();
    for _ in 0..1000 {
        let _ = user_lib::read(fd, &mut buf);
    }
    let end_read = user_lib::get_time_us();
    println!("Read 1000 times cost: {} us", end_read - start_read);

    user_lib::close(fd);
    0
}
