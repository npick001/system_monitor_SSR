use sysinfo::{System, Disks, Networks}; // Removed 'Components', added 'Networks'
use monitor_shared::SystemMetric;
use std::time::Duration;
use reqwest::Client;
use chrono::Utc;
use nvml_wrapper::Nvml;

// Helper to get a unique ID for the computer
fn get_hostname() -> String {
    hostname::get()
        .unwrap_or_else(|_| "unknown_host".into())
        .to_string_lossy()
        .into_owned()
}

// Updated Function Signature: Accepts System, Disks, AND Networks
fn collect_metrics(
    sys: &mut System, 
    disks: &mut Disks, 
    networks: &mut Networks, 
    nvml: &Option<Nvml>,
    host_id: &str
) -> SystemMetric {
    sys.refresh_cpu_all(); // FIX: Name changed from refresh_cpu()
    sys.refresh_memory();
    
    networks.refresh(true); 
    disks.refresh(true);    

    // CPU & RAM
    let cpu_usage = sys.global_cpu_usage();
    let ram_usage_mb = sys.used_memory() as f32 / 1024.0 / 1024.0;

    // NETWORK
    let (total_rx, total_tx) = networks.iter().fold((0, 0), |acc, (_name, data)| {
        (acc.0 + data.received(), acc.1 + data.transmitted())
    });
    
    let net_rx_kb = total_rx as f32 / 1024.0;
    let net_tx_kb = total_tx as f32 / 1024.0;

    // DISK 
    let (total_space, available_space) = disks.list().iter().fold((0, 0), |acc, disk| {
        (acc.0 + disk.total_space(), acc.1 + disk.available_space())
    });

    let disk_usage_percent = if total_space > 0 {
        ((total_space - available_space) as f32 / total_space as f32) * 100.0
    } else {
        0.0
    };

    // GPU
    let (gpu_usage, gpu_temp, gpu_vram_used_mb) = if let Some(n) = nvml {
        // Try to get the first device (GPU 0)
        match n.device_by_index(0) {
            Ok(device) => {
                let util = device.utilization_rates().map(|r| r.gpu as f32).unwrap_or(0.0);
                let temp = device.temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu).map(|t| t as f32).unwrap_or(0.0);
                let mem = device.memory_info().map(|m| m.used as f32 / 1024.0 / 1024.0).unwrap_or(0.0);
                (util, temp, mem)
            },
            Err(_) => (0.0, 0.0, 0.0) // NVML loaded, but no device found
        }
    } else {
        (0.0, 0.0, 0.0) // NVML failed to load (No NVIDIA driver)
    };

    SystemMetric {
        host_id: host_id.to_string(),
        cpu_usage,
        ram_usage_mb,
        disk_usage_percent,
        net_rx_kb,
        net_tx_kb,
        gpu_usage,
        gpu_temp,
        gpu_vram_used_mb,
        timestamp: Utc::now().timestamp(),
    }
}

#[tokio::main]
async fn main() {
    let client = Client::new();
    let api_url = "http://localhost:3000/ingest";
    let host_id = get_hostname();

    println!("Agent started for host: {}", host_id);

    let mut sys = System::new_all();
    let mut disks = Disks::new_with_refreshed_list();
    let mut networks = Networks::new_with_refreshed_list();

    // Initialize NVIDIA (Fail safely if not present)
    let nvml = match Nvml::init() {
        Ok(n) => {
            println!("✅ NVIDIA GPU Driver Detected.");
            Some(n)
        },
        Err(e) => {
            println!("⚠️ No NVIDIA GPU Driver found ({}). GPU stats will be 0.", e);
            None
        }
    };

    // Initial sleep to establish a CPU baseline
    tokio::time::sleep(Duration::from_secs(1)).await;

    loop {
        // Pass all three mutable references
        let metric = collect_metrics(&mut sys, &mut disks, &mut networks, &nvml, &host_id);

        match client.post(api_url).json(&metric).send().await {
            Ok(_) => println!("Sent: CPU {:.1}% | RAM {:.0}MB | Disk {:.1}% | GPU {:.1}% | VRAM {:.0}MB", 
                metric.cpu_usage, metric.ram_usage_mb, metric.disk_usage_percent, metric.gpu_usage, metric.gpu_vram_used_mb),
            Err(e) => println!("Failed to send metric: {}", e),
        }

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}