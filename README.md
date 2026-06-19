# FastAPI Benchmarking
This repository contains a baseline implementation of a high-throughput microservice built with Python and FastAPI.
The primary goal of this project is to establish baseline performance metrics for routing, I/O bound operations in Python

## 🗺️ API Endpoints
This service exposes two distinct endpoints designed to test different operational scenarios:

### The CPU/Routing Benchmark (`/greeting`)
* **Endpoint:** `GET /greeting`
* **Purpose:** Tests pure framework overhead, routing speed, and serialization.
* **Response:** Returns a simple `"Hello, World!"` string.

### The I/O Benchmark (`/request`)
* **Endpoint:** `GET /request`
* **Purpose:** Simulates a real-world, I/O-bound workload.
* **Behavior:** Fetches a structured data record from a database.
* **Testing Variants:** This endpoint is designed to be load-tested state to measure how FastAPI's async event loop handles database latency
---

## 🛠️ Setup & Local Development
To run this project locally and prepare it for load testing:

**1. Clone and navigate to the project:**
```bash
git clone <url>
cd fastapi-benchmarking
```

**2. Set up a virtual environment and install dependencies:**
```bash
python -m venv .venv
source .venv/bin/activate  # On Windows use `.venv\Scripts\activate`
pip install -r requirements.txt
```

**3. Run the application:**
*Note: For production-grade benchmarking, it is highly recommended to run Uvicorn with multiple workers inside Dockerfile.*
```bash
docker compose up --build
```

---

## 📊 Benchmarking Methodology

To ensure a fair and rigorous test, we will be using standard load testing tools **wrk** to simulate real-world traffic patterns and hit our target endpoints. Our approach focuses on establishing a controlled environment where variables are isolated, allowing us to accurately identify bottlenecks and measure the system's scalability limits.

The metrics we are optimizing to track include:

| **Metric**               | **Description**                                                                                    |
| ------------------------ | -------------------------------------------------------------------------------------------------- |
| **Thread Stats**         | Insights into how many concurrent threads/virtual users are active and their resource consumption. |
| **Average Latency**      | The mean time (typically in ms) it takes for a request to be processed and a response received.    |
| **Req/Sec (Throughput)** | The total volume of requests the system can handle per second under load.                          |
| **Total Data Volume**    | Cumulative count of successful requests and the total throughput in MB read/transferred.           |
| **Transfer/Sec**         | The network bandwidth utilization, measured in throughput rate (e.g., MB/s).                       |

---
## 🛠 Environment & Setup
To ensure consistency, all tests were performed in a clean, isolated containerized environment.
- **Command:** `docker compose up --build`
- **Load Tool:** `wrk`
- **Test Parameters:** 5 threads, 100 connections, 20-second duration.
- **Target Endpoints:** `http://127.0.0.1:8000/greeting` & `http://127.0.0.1:8000/request`

---
## 📊 Performance Summary
### `/greeting`
##### Raw WRK output
```
Running 20s test @ http://127.0.0.1:8000/greeting
  5 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    72.03ms  184.18ms   1.10s    91.11%
    Req/Sec     1.33k   250.90     2.00k    89.90%
  118269 requests in 20.02s, 17.03MB read
Requests/sec:   5907.19
Transfer/sec:      0.85MB
```

| Key Metric               | Result        |
| ------------------------ | ------------- |
| **Average Latency**      | 72.03 ms      |
| **Total Requests**       | 118 269       |
| **Throughput (Req/Sec)** | 5907.19 req/s |
| **Transfer Rate**        | 0.85 MB/s     |
### `/request`
##### Raw WRK output
```
Running 20s test @ http://127.0.0.1:8000/request
  5 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    49.51ms   54.58ms 618.71ms   87.49%
    Req/Sec   545.72    105.83   838.00     72.03%
  54050 requests in 20.02s, 60.72MB read
Requests/sec:   2700.15
Transfer/sec:      3.03MB
```

| Key Metric               | Result        |
| ------------------------ | ------------- |
| **Average Latency**      | 49.51 ms      |
| **Total Requests**       | 54 050        |
| **Throughput (Req/Sec)** | 2700.15 req/s |
| **Transfer Rate**        | 3.03 MB/s     |
_Pay attention for **Throughput**, even average latency for `/request` is lower, you can see that it took much less requests than `/greeting`_
