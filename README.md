# AI Gateway (Rust)

A lightweight API Gateway written in Rust using Tokio.

This project implements core gateway functionality such as authentication, rate limiting, load balancing, and backend health checks.

---

## Why This Project Exists

Modern applications often rely on API gateways to handle authentication, rate limiting, and request routing before traffic reaches backend services.

This project was built as a learning exercise to explore how core gateway features work internally by implementing them from scratch in Rust.

The goal was to build a lightweight gateway capable of:

- Authenticating requests using API keys
- Protecting services with rate limiting
- Distributing traffic across multiple backends
- Automatically detecting unhealthy services
- Handling high concurrency using async Rust (Tokio)

## Features

* API Key Authentication
* Token Bucket Rate Limiting
* Round-Robin Load Balancing
* Automatic Backend Health Checks
* Backend Failover
* Request Size Protection
* Request Timeout Protection
* Async Networking with Tokio


                +------------------+
                |      Client      |
                |  curl / browser  |
                +---------+--------+
                          |
                          |
                          v
                +------------------+
                |    AI Gateway    |
                |------------------|
                | API Key Auth     |
                | Rate Limiting    |
                | Load Balancer    |
                | Health Checks    |
                +---------+--------+
                          |
            +-------------+-------------+
            |                           |
            v                           v
     +-------------+             +-------------+
     |  Backend 1  |             |  Backend 2  |
     | 127.0.0.1   |             | 127.0.0.1   |
     |    :9002    |             |    :9003    |
     +-------------+             +-------------+


---
## Running

Build the gateway:

```
cargo build --release
```

Run it:

```
./target/release/ai_gateaway
```

The gateway will start on:

```
0.0.0.0:8080
```

---

## Example Request

```
curl -H "X-API-Key: user1" http://127.0.0.1:8080/test
```

---

## API Keys

API keys and rate limits are defined in:

```
api_keys.json
```

Example:

```
{
  "user1": 2,
  "user2": 10
}
```

This means:

* `user1` → 2 requests per second
* `user2` → 10 requests per second

---

## Architecture

```
Client → Gateway → Backend Services
```

Example route configuration:

```
/test → 127.0.0.1:9002
/test → 127.0.0.1:9003
```

Requests are distributed using **round-robin load balancing**.

---

## Benchmark

Example benchmark using `wrk`:

```
wrk -t4 -c100 -d30s \
-H "X-API-Key: user1" \
http://127.0.0.1:8080/test
```

Example result:

```
Requests/sec: ~60,000
```

---

## Project Structure

```
src/
 ├── main.rs
 ├── metrics.rs
 ├── rate_limiter.rs
 ├── load_balancer.rs
 └── config.rs
```

---

## License

Personal project
