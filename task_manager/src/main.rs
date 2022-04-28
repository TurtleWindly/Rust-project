use std::{thread, time};

// let ten_millis = time::Duration::from_millis(10);
// let now = time::Instant::now();

// thread::sleep(ten_millis);

// assert!(now.elapsed() >= ten_millis);

fn main() {
    loop {
        let time = time::Duration::from_secs(2);
        clearscreen::clear().unwrap();
        task_manager();
        thread::sleep(time);
    }
}

fn task_manager() {
    println!("----------Cpu");
    let cpu_speed = sys_info::cpu_speed();
    if let Ok(speed) = cpu_speed {
        println!("speed: {} MHz", speed);
    }

    println!("\n----------Ram");
    let memory_info = sys_info::mem_info();
    if let Ok(info) = memory_info {
        println!("total: {}G", kb_to_gb(info.total));
        println!("avail: {}G", kb_to_gb(info.avail));
        println!("used:  {}G", kb_to_gb(info.total - info.avail));
    }

    println!("\n----------Disk");
    let disk_info = sys_info::disk_info();
    if let Ok(info) = disk_info {
        println!("total: {}G", kb_to_gb(info.total));
        println!("free:  {}G", kb_to_gb(info.free));
        println!("used:  {}G", kb_to_gb(info.total - info.free));
    }

    println!("\n----------OS info");
    if let Ok(type_os) = sys_info::os_type() {
        println!("Operating System: {}", type_os);
    }
    if let Ok(release_os) = sys_info::os_release() {
        println!("Release:          {}", release_os);
    }

    if let Ok(host_name) = sys_info::hostname() {
        print!("Host:               {}\n", host_name);
    }

}

fn kb_to_gb(number: u64) -> f64 {
    number as f64 / 1_000_000f64
}
