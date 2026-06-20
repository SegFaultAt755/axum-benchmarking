# ⚙️ Axum Benchmarking

This repository contains a baseline implementation of a high-throughput microservice built with Rust and Axum.

The primary goal of this project is to establish baseline performance metrics for routing and I/O-bound operations in Rust.

### Requirements

- **Rust** & **Cargo** 1.95+
- **Docker** & **Docker Compose**
- **wrk** (for load testing)

## 🗺️ API Endpoints

This service exposes two distinct endpoints designed to test different operational scenarios:

### 1. The CPU/Routing Benchmark

- **Endpoint:** `GET /greeting`
- **Purpose:** Tests pure framework overhead, routing speed, and serialization.
- **Response:** Returns a simple "Hello, World!" string.

### 2. The I/O Benchmark

- **Endpoint:** `GET /request`
- **Purpose:** Simulates a real-world, I/O-bound workload.
- **Behavior:** Fetches a structured data record from a database.
- **Testing Variants:** Designed to be load-tested to measure how Axum & SQLx async event loop handles database latency.

## 🛠️ Setup & Development

To run this project locally and prepare it for load testing:

**1. Clone and navigate to the project:**

Bash

```
git clone https://github.com/SegFaultAt755/axum-benchmarking.git
cd axum-benchmarking
```

**2. Run the application:**

Bash

```
docker compose up --build
```

> **Note:** For production-grade benchmarking, it is highly recommended to run Uvicorn with multiple workers inside the `Dockerfile`.

## ⚙️ Load Testing Tool (wrk) Setup

To run the benchmarks, you will need to install `wrk`. Use the command appropriate for your operating system:

**MacOS (Homebrew):**

Bash

```
brew install wrk
```

**Ubuntu / Debian:**

Bash

```
sudo apt-get update
sudo apt-get install wrk
```

**Arch Linux:**

Bash

```
yay -S wrk
```

_(The AUR doesn't have `wrk` natively via `pacman`, so `yay` is used to install it)._

**Windows:** `wrk` is not natively supported on Windows. You must install it inside WSL or download `winrk.exe` from [SourceForge](https://sourceforge.net/projects/winrk/).

## 📊 Benchmarking Methodology

To ensure a fair and rigorous test, we use the standard load testing tool `wrk` to simulate real-world traffic patterns and hit our target endpoints. Our approach focuses on establishing a controlled environment where variables are isolated, allowing us to accurately identify bottlenecks and measure the system's scalability limits.

### Hardware Specifications

|**Component**|**Specification**|
|---|---|
|**OS**|Arch Linux x86_64|
|**CPU**|AMD Ryzen 5 5600 (12) @ 4.470GHz|
|**RAM**|15,862 MiB DDR4|
|**Storage**|512GB NVMe SSD|

### Test Environment & Setup

To ensure consistency, all tests were performed in a clean, isolated containerized environment.

- **Command:** `docker compose up --build`
- **Target Endpoints:** `http://127.0.0.1:8000/greeting` & `http://127.0.0.1:8000/request`
- **Test Command:** `wrk -t5 -c100 -d20s http://127.0.0.1:8000/[endpoint]`
- **Configuration:** 5 threads, 100 connections, 20-second duration.

### Metrics Tracked

- **Thread Stats:** Active concurrent threads and resource consumption.
- **Average Latency:** Mean time to process a request and return a response.
- **Throughput:** Total volume of requests handled per second.
- **Transfer/Sec:** Network bandwidth utilization (MB/s).

## 📈 Performance Summary

### `/greeting` Benchmark

**Raw `wrk` Output:**

Plaintext

```
Not yet...
```

**Results:**

_No results yet..._

### `/request` Benchmark

**Raw `wrk` Output:**

Plaintext

```
Not yet...
```

**Results:**

_No results yet..._