use std::fs;
use std::time::Duration;
use std::thread;

struct NodeResources {
    cpu_cores: usize,
    total_memory_mb: u64,
    available_memory_mb: u64,
    cpu_usage_percent: f32,
}

fn detect_cpu_cores() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1)
}

fn detect_memory() -> (u64, u64) {
    let contents = fs::read_to_string("/proc/meminfo")
        .expect("Failed to read /proc/meminfo");

    let mut total_kb = 0;
    let mut available_kb = 0;

    for line in contents.lines() {
        if let Some(value) = line.strip_prefix("MemTotal:") {
            total_kb = value
                .trim()
                .trim_end_matches(" kB")
                .parse()
                .expect("Invalid MemTotal value");
        }

        if let Some(value) = line.strip_prefix("MemAvailable:") {
            available_kb = value
                .trim()
                .trim_end_matches(" kB")
                .parse()
                .expect("Invalid MemAvailable value");
        }
    }

    (total_kb / 1024, available_kb / 1024)
}

fn read_cpu_times() -> (u64, u64) {
    let contents = fs::read_to_string("/proc/stat")
        .expect("Failed to read /proc/stat");

    let line = contents
        .lines()
        .find(|line| line.starts_with("cpu " ))
        .expect("CPU information not found");

    let values: Vec<u64> = line
        .split_whitespace()
        .skip(1)
        .map(|value| value.parse().unwrap_or(0))
        .collect();

    let idle = values.get(3).copied().unwrap_or(0)
        + values.get(4).copied().unwrap_or(0);

    let total: u64 = values.iter().sum();

    (idle, total)
}

fn detect_cpu_usage() -> f32 {
    let (idle1, total1) = read_cpu_times();

    thread::sleep(Duration::from_millis(500));

    let (idle2, total2) = read_cpu_times();

    let idle_delta = idle2 - idle1;
    let total_delta = total2 - total1;

    if total_delta == 0 {
        return 0.0;
    }

    let usage = 100.0 * (1.0 - idle_delta as f32 / total_delta as f32);

    usage.clamp(0.0, 100.0)
}

fn detect_resources() -> NodeResources {
    let (total_memory_mb, available_memory_mb) = detect_memory();

    NodeResources {
        cpu_cores: detect_cpu_cores(),
        total_memory_mb,
        available_memory_mb,
        cpu_usage_percent: detect_cpu_usage(),
    }
}

fn main() {
    println!("Nebula Node Agent");
    println!("=================");
    println!();

    let resources = detect_resources();

    println!("CPU cores       : {}", resources.cpu_cores);
    println!("CPU usage       : {:.2}%", resources.cpu_usage_percent);
    println!(
        "Total RAM       : {} MB",
        resources.total_memory_mb
    );
    println!(
        "Available RAM   : {} MB",
        resources.available_memory_mb
    );

    println!();
    println!("READY");
}