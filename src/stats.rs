use sysinfo::{Disks, Components, MINIMUM_CPU_UPDATE_INTERVAL, System};

pub fn current_machine_stats() -> String {
    let mut sys: System = System::new();

    // ! SYSTEM DATA
    let host_name = match System::host_name() {
        Some(h) => h,
        None => "unknown".to_string()
    };
    let system_name = match System::name() {
        Some(s) => s,
        None => "unknown".to_string()
    };
    let os_ver = match System::long_os_version() {
        Some(o) => o,
        None => "unknown".to_string()
    };
    let kernel_ver = System::kernel_long_version();

    let uptime_total_secs = System::uptime();
    let uptime_hours = uptime_total_secs / 3600;
    let uptime_minutes = (uptime_total_secs - uptime_hours * 3600) / 60;
    let uptime_secs = uptime_total_secs - uptime_hours * 3600 - uptime_minutes * 60;

    // ! CPU
    sys.refresh_cpu_all();
    std::thread::sleep(MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_cpu_all();
    let current_cpu_usage = (sys.global_cpu_usage() * 1000.0).round() / 1000.0;
    
    let current_cpu_temp = match cpu_temp() {
        Some(t) => format!("{} °C", t),
        None => "CPU temperature unknown.".to_string()
    };

    // ! RAM
    sys.refresh_memory();
    let ram_used = (sys.used_memory() as f64 / 1073741824.0 * 100.0).round() / 100.0;
    let ram_total = (sys.total_memory() as f64 / 1073741824.0 * 100.0).round() / 100.0;

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
    Host name:          {host_name}     \n
    System name:        {system_name}   \n
    OS version:         {os_ver}        \n
    Kernel version:     {kernel_ver}    \n
    Uptime:             {uptime_hours}:{uptime_minutes}:{uptime_secs}        \n

    ------------------------------------------

    CPU usage: {current_cpu_usage} %        \n
    CPU temp:  {current_cpu_temp}           \n
    RAM usage: {ram_used} / {ram_total} GB  \n
    Disks:                                  \n   
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