# Drasi MQTT Source - Proof of Concept

This repository serves as a technical **Proof of Concept (PoC)** for the **LFX Mentorship 2026** project: *"MQTT Support for Drasi Lib"*.

It demonstrates a minimal, standalone implementation of a **Drasi Source** for MQTT, written in Rust.

## 🎯 Architecture validated
This prototype validates the critical components required to build `drasi-source-mqtt`:

1.  **Async Event Loop:** Utilizes `tokio` and `rumqttc` to manage persistent broker connections without blocking the main thread.
2.  **Declarative Mapping:** Implements a mapping logic that transforms raw JSON payloads from Edge devices into structured "Graph Elements" (Drasi's internal data model).
3.  **Resilience:** Includes automatic reconnection logic to handle unstable IoT network conditions.

## 🚀 How to Run

### Prerequisites
- Rust (cargo) installed

### Steps
1. **Run the Source:**
   ```bash
   RUST_LOG=info cargo run
   ```
   You will see a connection success message.

2. **Simulate an IoT Device:** Open a separate terminal and use `mosquitto_pub` (or any MQTT client) to send data to the public test broker:

    ```bash
    mosquitto_pub -h test.mosquitto.org \
      -t "lfx/drasi/sensors/temp-sensor-01" \
      -m '{"temperature": 24.5, "humidity": 60}'
    ```

3. **Observe the Output:** The application will ingest the JSON and map it to a Graph Node structure:

    ```plaintext
    INFO: -> Ingested Graph Node: DrasiElement { id: "temp-sensor-01", ... }
    ```

## 🛠 Tech Stack
- **Language:** Rust (2021 Edition)
- **Runtime:** Tokio (Async I/O)
- **Protocol:** MQTT v3.1.1 (via rumqttc)
- **Serialization:** Serde JSON
