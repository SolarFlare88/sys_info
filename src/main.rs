use sysinfo::{ System, Pid };
use std::{io::{self, Write}, thread, time::{Duration}};
fn main() {

    let mut sys = System::new_all();

    // 获取用户输入的 PID
    println!("Enter the PID of the process to monitor:");
    let mut input = String::new();
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let pid_u32 = input.trim().parse::<u32>().expect("Invalid PID format");
    let pid = Pid::from(pid_u32 as usize);

    loop {
        sys.refresh_process(pid);

        // 获取进程信息
        if let Some(process) = sys.process(pid) {
            let memory_value = process.memory();
            let mb = memory_value as f64 / 1_048_576.0;
            // 打印进程的内存使用情况
            println!("Process memory usage: {:.3} MB", mb);
        } else {
            println!("Process with PID {} not found", pid);
            break; // 如果找不到进程，则退出循环
        }

        // 每隔一段时间检查一次
        thread::sleep(Duration::from_secs(5)); // 这里设置为每5秒检查一次
    }
}
