use sysinfo::{Disks, Components, MINIMUM_CPU_UPDATE_INTERVAL, System};

pub fn current_machine_stats() -> String {
    let mut sys: System = System::new();

    // ! CPU
    sys.refresh_cpu_all();
    std::thread::sleep(MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_cpu_all();
    let current_cpu_usage = sys.global_cpu_usage();
    
    let current_cpu_temp = match cpu_temp() {
        Some(t) => format!("{} °C", t),
        None => "CPU temperature unknown.".to_string()
    };

    // ! RAM
    sys.refresh_memory();
    let ram_used  = sys.used_memory() as f64 / 1073741824.0;
    let ram_total = sys.total_memory() as f64 / 1073741824.0;

    // ! DISK
    let disks = Disks::new_with_refreshed_list();
    let mut disks_info = String::new();
    for disk in disks.list() {
        let used = disk.total_space() - disk.available_space();
        let disk_str = format!("
        Disk name:       {:?}   
        Disk kind:       {}   
        Used storage:    {} / {}
        Available space: {} \n",
        disk.name(), disk.kind(), used, disk.total_space(), disk.available_space());
        
        disks_info.push_str(&disk_str);
    }

    format!("
    CPU usage: {current_cpu_usage} %    \n
    CPU temp:  {current_cpu_temp}       \n
    RAM usage: {ram_used} / {ram_total} \n
    Disks:\n   
    {disks_info}             
    ")
}

fn cpu_temp() -> Option<f32> {
    let components = Components::new_with_refreshed_list();
    for c in &components {
        let label = c.label().to_lowercase();
        if label.contains("cpu") || label.contains("package")
        || label.contains("tctl") || label.contains("k10temp") {
            return c.temperature()
        }
    }
    None
}