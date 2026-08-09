use sysinfo::{Components, MINIMUM_CPU_UPDATE_INTERVAL, System};

pub fn current_machine_stats() -> String {
    let mut sys: System = System::new();

    sys.refresh_cpu_all();
    std::thread::sleep(MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_cpu_all();
    let current_cpu_usage = sys.global_cpu_usage();
    
    let current_cpu_temp = match cpu_temp() {
        Some(t) => format!("{} °C", t),
        None => "CPU temperature unknown.".to_string()
    };

    sys.refresh_memory();
    let ram_used  = sys.used_memory() as f64 / 1073741824.0;
    let ram_total = sys.total_memory() as f64 / 1073741824.0;

    format!("
    CPU usage: {current_cpu_usage} %    \n
    CPU temp:  {current_cpu_temp}       \n
    RAM usage: {ram_used} / {ram_total} \n
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