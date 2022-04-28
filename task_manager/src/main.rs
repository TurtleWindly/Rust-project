fn main() {
    // clearscreen::clear().unwrap();
    println!("Cpu");
    let cpu_speed = sys_info::cpu_speed();
    if let Ok(speed) = cpu_speed {
        println!("speed: {} MHz", speed);
    }

    println!("\nRam");
    let memory_info = sys_info::mem_info();
    if let Ok(info) = memory_info {
        println!("total: {}G", kb_to_gb(info.total));
        println!("avail: {}G", kb_to_gb(info.avail));
        println!("used: {}G", kb_to_gb(info.total - info.avail));
    }

    println!("\nDisk");
    let disk_info = sys_info::disk_info();
    if let Ok(info) = disk_info {
        println!("total: {}G", kb_to_gb(info.total));
        println!("free: {}G", kb_to_gb(info.free));
        println!("used: {}G", kb_to_gb(info.total - info.free));
    }

    println!("\nOS info");
    if let Ok(type_os) = sys_info::os_type() {
        println!("Operating System: {}", type_os);
    }
    if let Ok(release_os) = sys_info::os_release() {
        println!("Release: {}", release_os);
    }

    if let Ok(host_name) = sys_info::hostname() {
        print!("Host: {}", host_name);
    }

    println!();

}

fn kb_to_gb(number: u64) -> f64 {
    number as f64 / 1_000_000f64
}
