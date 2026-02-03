use anyhow::Result;
use rumqttc::{AsyncClient, MqttOptions, QoS, Event, Packet};
use serde::Serialize;
use serde_json::Value;
use std::time::Duration;
use log::{info, error, warn};

// --- MOCK DRASI STRUCTURES ---
// This struct mimics the internal "Graph Element" Drasi uses.
// It proves you understand how to bridge External Data -> Drasi Data.
#[derive(Debug, Serialize)]
struct DrasiElement {
    id: String,
    labels: Vec<String>,
    properties: Value,
}

// --- CONFIGURATION ---
const BROKER_HOST: &str = "test.mosquitto.org";
const BROKER_PORT: u16 = 1883;
// We listen to a wildcard topic to simulate multiple sensors
const TOPIC_PATTERN: &str = "lfx/drasi/sensors/#";

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Initialize Logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    info!("Starting Drasi MQTT Source PoC...");

    // 2. Configure MQTT Options
    // We use a random client ID to prevent collisions on the public broker
    let client_id = format!("drasi-poc-{}", uuid::Uuid::new_v4());
    let mut mqttoptions = MqttOptions::new(client_id, BROKER_HOST, BROKER_PORT);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    // 3. Create Async Client
    // 'client' is used to control the connection (subscribe/publish)
    // 'eventloop' is the stream of incoming network packets
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    // 4. Subscribe (The "Source" Logic)
    // In a real Drasi Source, this topic would be configurable via YAML
    client.subscribe(TOPIC_PATTERN, QoS::AtLeastOnce).await?;
    info!("Subscribed to topic: {}", TOPIC_PATTERN);

    // 5. Main Event Loop
    // This loop listens for signals and processes them asynchronously
    loop {
        match eventloop.poll().await {
            Ok(notification) => {
                match notification {
                    Event::Incoming(Packet::Publish(publish)) => {
                        // When a message arrives, we process it immediately
                        let topic = publish.topic.clone();
                        let payload = publish.payload.clone();
                        
                        // We use a separate function to keep the loop clean
                        // In production, this would spawn a tokio task
                        if let Err(e) = process_payload(&topic, &payload) {
                            error!("Failed to map payload from {}: {}", topic, e);
                        }
                    }
                    Event::Incoming(Packet::ConnAck(_)) => {
                        info!("Successfully connected to MQTT Broker!");
                    }
                    _ => {} // Ignore Pings and Acks to keep logs clean
                }
            }
            Err(e) => {
                warn!("Connection lost: {:?}. Retrying...", e);
                // rumqttc handles the reconnect logic automatically, we just wait a bit
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    }
}

// --- CORE MAPPING LOGIC ---
// This function demonstrates the "Source" responsibility:
// Converting Raw JSON -> Drasi Graph Element
fn process_payload(topic: &str, payload: &[u8]) -> Result<()> {
    // A. Parse Raw JSON
    let json: Value = serde_json::from_slice(payload)?;
    
    // B. Extract Metadata from Topic
    // Example: "lfx/drasi/sensors/temp-01" -> ID: "temp-01"
    let device_id = topic.split('/').last().unwrap_or("unknown");
    
    // C. Map to Graph Element
    // This simulates the internal Drasi data structure
    let element = DrasiElement {
        id: device_id.to_string(),
        labels: vec!["Sensor".to_string(), "IoTDevice".to_string()],
        properties: json,
    };

    // D. "Emit" to System
    // In the real implementation, this would push to the Drasi Change Stream
    info!("-> Ingested Graph Node: {:?}", element);
    Ok(())
}
