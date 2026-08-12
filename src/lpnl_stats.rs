use sysinfo::{Disks, Components, MINIMUM_CPU_UPDATE_INTERVAL, System};

pub fn short_stats() -> String {
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

    format!("
    Host name:          {host_name}     \n
    System name:        {system_name}   \n
    OS version:         {os_ver}        \n
    Kernel version:     {kernel_ver}    \n
    Uptime:             {uptime_hours}:{uptime_minutes}:{uptime_secs}

    ------------------------------------------

    CPU usage: {current_cpu_usage} %        \n
    CPU temp:  {current_cpu_temp}           \n
    RAM usage: {ram_used} / {ram_total} GB  \n        
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

    // ! DISKS
pub fn disk_stats() -> String {
    let disks = Disks::new_with_refreshed_list();
    let mut disks_info = String::new();
    disks_info.push_str("Disks:\n");
    for disk in disks.list() {
        let used = disk.total_space() / 1073741824 - disk.available_space() / 1073741824;
        let total_space = disk.total_space() / 1073741824;
        let used_pecr = ( used as f64 / total_space as f64 ) * 100.0;
        let available = disk.available_space() / 1073741824;
        
        let (f_ffree, f_favail, f_files) = match nix::sys::statvfs::statvfs(disk.mount_point()) {
            Ok(s) => ( s.files_free(), s.files_available(), s.files() ),
            Err(_) => {( 0, 0, 0 )},
        };
    
        let f_ffree_perc = if f_files == 0 {
            0.0
        } else {
            f_ffree as f64 / f_files as f64 * 100.0
        };
        let f_favail_perc = if f_files == 0 {
            0.0
        } else {
            f_favail as f64 / f_files as f64 * 100.0
        };


        let disk_str = format!("
        Disk name:          {:?}   
        Disk kind:          {}   
        Used storage:       {} / {} GB ( {:.2} % )
        Available space:    {} GB
        Inode free:         {} / {} ( {:.2} % )
        Inode available:    {} / {} ( {:.2} % ) \n",
         disk.name(), 
         disk.kind(), 
         used, total_space, used_pecr,
         available,
         f_ffree, f_files, f_ffree_perc, 
         f_favail, f_files, f_favail_perc,
        );
        disks_info.push_str(&disk_str);
    }
    disks_info
}