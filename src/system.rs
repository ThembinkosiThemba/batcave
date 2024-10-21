use chrono::Local;
use colored::*;
use sysinfo::{CpuRefreshKind, System};

pub fn system_info() -> String {
    let mut sys = System::new_all();
    sys.refresh_cpu_list(CpuRefreshKind::everything());
    sys.refresh_memory();

    let current_time = Local::now();
    let cpu_usage = sys.global_cpu_usage();
    let total_memory = sys.total_memory() / 1024 / 1024;
    let used_memory = sys.used_memory() / 1024 / 1024;
    let memory_usage = (used_memory as f64 / total_memory as f64) * 100.0;

    let mut info = String::new();
    info.push_str(&format!(
        "{}\n",
        "System Overview".bright_yellow().bold().underline()
    ));
    info.push_str(&format!(
        "{:<20}: {}\n",
        "User".bright_green(),
        sysinfo::System::host_name().unwrap_or_else(|| "Unknown".to_string())
    ));
    info.push_str(&format!(
        "{:<20}: {}\n",
        "Time".bright_green(),
        current_time.format("%Y-%m-%d %H:%M:%S")
    ));
    info.push_str(&format!(
        "{:<20}: {}\n",
        "System Name".bright_green(),
        System::name().unwrap_or_else(|| "Unknown".to_string())
    ));
    info.push_str(&format!(
        "{:<20}: {}\n",
        "Kernel Version".bright_green(),
        System::kernel_version().unwrap_or_else(|| "Unknown".to_string())
    ));
    info.push_str(&format!(
        "{:<20}: {}\n",
        "OS Version".bright_green(),
        System::os_version().unwrap_or_else(|| "Unknown".to_string())
    ));
    info.push_str(&format!(
        "{:<20}: {}\n",
        "Number of CPUs".bright_green(),
        sys.cpus().len()
    ));

    // CPU Usage Graph
    info.push_str(&format!(
        "\n{}\n",
        "CPU Usage".bright_yellow().bold().underline()
    ));
    info.push_str(&format!("{:<10}: {:.2}%\n", "Current", cpu_usage));
    info.push_str(&cpu_usage_graph(cpu_usage.into()));

    // Memory Usage Graph
    info.push_str(&format!(
        "\n{}\n",
        "Memory Usage".bright_yellow().bold().underline()
    ));
    info.push_str(&format!(
        "{:<10}: {} MB / {} MB ({:.2}%)\n",
        "Current", used_memory, total_memory, memory_usage
    ));
    info.push_str(&memory_usage_graph(memory_usage));

    // Swap Information
    info.push_str(&format!(
        "\n{}\n",
        "Swap Usage".bright_yellow().bold().underline()
    ));
    let total_swap = sys.total_swap() / 1024 / 1024;
    let used_swap = sys.used_swap() / 1024 / 1024;
    info.push_str(&format!(
        "{:<10}: {} MB / {} MB\n",
        "Current", used_swap, total_swap
    ));

    info
}

fn cpu_usage_graph(usage: f64) -> String {
    let width = 50;
    let filled = (usage / 100.0 * width as f64) as usize;
    let empty = width - filled;
    format!(
        "[{}{}] {:.2}%\n",
        "█".repeat(filled).bright_green(),
        "░".repeat(empty),
        usage
    )
}

fn memory_usage_graph(usage: f64) -> String {
    let width = 50;
    let filled = (usage / 100.0 * width as f64) as usize;
    let empty = width - filled;
    format!(
        "[{}{}] {:.2}%\n",
        "█".repeat(filled).bright_blue(),
        "░".repeat(empty),
        usage
    )
}
