use std::collections::HashMap;
use tokio::time::{interval, Duration};
use ping::Pinger;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
struct PingResult {
    min: f64,
    max: f64,
    timestamps: Vec<DateTime<Utc>>,
}

#[derive(Debug)]
struct PingMonitor {
    hosts: Vec<String>,
    results: HashMap<String, PingResult>,
    pinger: Pinger,
}

impl PingMonitor {
    pub fn new(hosts: Vec<String>) -> Self {
        Self {
            hosts,
            results: HashMap::new(),
            pinger: Pinger::new().unwrap(),
        }
    }

    pub async fn start(&mut self) {
        let mut interval = interval(Duration::from_secs(60));
        loop {
            interval.tick().await;
            self.ping_hosts().await;
            self.cleanup_old_data();
        }
    }

    async fn ping_hosts(&mut self) {
        for host in &self.hosts {
            match self.pinger.ping(host).await {
                Ok(response) => {
                    let time = response.time;
                    let entry = self.results.entry(host.clone()).or_insert(PingResult {
                        min: time,
                        max: time,
                        timestamps: Vec::new(),
                    });

                    entry.min = entry.min.min(time);
                    entry.max = entry.max.max(time);
                    entry.timestamps.push(Utc::now());

                    if entry.timestamps.len() > 10080 { // Keep up to 7 days (60 * 24 * 7 = 10080)
                        entry.timestamps.remove(0);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to ping {}: {}", host, e);
                }
            }
        }
    }

    fn cleanup_old_data(&mut self) {
        let week_ago = Utc::now() - chrono::Duration::days(7);
        for result in self.results.values_mut() {
            result.timestamps.retain(|&ts| ts > week_ago);
        }
    }
}
