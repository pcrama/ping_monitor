mod ping_monitor;

#[tokio::main]
async fn main() {
    let hosts = vec![
        "8.8.8.8".to_string(),
        "8.8.4.4".to_string(),
        "berlaymont.smartschool.be".to_string(),
        "192.168.129.250".to_string(),
    ];

    let mut monitor = ping_monitor::PingMonitor::new(hosts);
    monitor.start().await;
}
