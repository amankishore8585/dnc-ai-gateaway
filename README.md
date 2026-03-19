# AI Gateway (Rust)

A lightweight API Gateway written in Rust using Tokio.

The gateway sits between client applications and AI providers (such as OpenAI) and provides:

-authentication
-rate limiting
-request routing
-traffic protection
-metrics and logging

It is designed as a minimal, high-performance gateway for AI workloads.

The gateway can be run locally (self-hosted) or used via a hosted endpoint.

## Why This Project Exists

Many modern applications rely on external AI APIs. Calling those APIs directly from applications makes it difficult to manage:

authentication
rate limiting
usage monitoring
backend routing
traffic protection

This project explores how an AI gateway layer can be implemented from scratch using async Rust (Tokio).

The goal was to build a simple gateway capable of protecting AI APIs and controlling traffic before it reaches backend services.

## Architecture

Client Application
        |
        v
+------------------+
|    AI Gateway    |
|------------------|
| API Key Auth     |
| Rate Limiting    |
| Load Balancer    |
| Health Checks    |
| Metrics & Logs   |
+---------+--------+
          |
          v
+------------------+
|   AI Provider    |
|   (OpenAI API)   |
+------------------+

## Using the Gateway

You can use the gateway in two ways:

1. Self-Hosted (Recommended)

  Run the gateway locally or on your own VPS for full control.

  cargo build --release
  ./target/release/ai_gateaway

  The gateway will be available at:

  http://127.0.0.1:8080

  Use this option if you want:

  -full control over configuration
  -local development and testing
  -no dependency on external services

2. Hosted Gateway (Experimental)

  You can also use a hosted version of the gateway:

  https://dncgateway.com/v1

  ⚠️ Note: This is an early version and not production-ready.

  Use this option if you want:
  -quick setup without running infrastructure
  -simple integration for testing  

## Using the Gateway from an Application

Applications can call the gateway instead of calling the OpenAI API directly.

Depending on your setup, this can be:

-your local gateway (http://127.0.0.1:8080)

-your own deployed instance

-or the hosted gateway (https://dncgateway.com/v1)

The gateway forwards requests to the backend AI provider while applying authentication and rate limiting

Clients must include:

X-API-Key

along with their OpenAI request.

## Python Example

Using the official OpenAI Python client.

# Option 1: Self-hosted
  client = OpenAI(
      api_key=os.getenv("OPENAI_API_KEY"),
      base_url="http://127.0.0.1:8080/v1",
      default_headers={
          "X-API-Key": "user1"
      }
  )

# Option 2: Hosted  
  client = OpenAI(
      api_key=os.getenv("OPENAI_API_KEY"),
      base_url="https://dncgateway.com/v1",
      default_headers={
          "X-API-Key": "user1"
      }
  )

## JavaScript Example

# Option 1: Self-hosted
const client = new OpenAI({
  apiKey: process.env.OPENAI_API_KEY,
  baseURL: "http://127.0.0.1:8080/v1",
  defaultHeaders: {
    "X-API-Key": "user1"
  }
});

# Option 2; Hosted
const client = new OpenAI({
  apiKey: process.env.OPENAI_API_KEY,
  baseURL: "https://dncgateway.com/v1",
  defaultHeaders: {
    "X-API-Key": "user1"
  }
});


## Features

* API Key Authentication
  Requests must include an API key:
  X-API-Key
  Keys and limits are configured via api_keys.json.

* Token Bucket Rate Limiting
  Per-user rate limiting using a token bucket algorithm.

  Example:

  user1 → 2 requests/sec
  user2 → 10 requests/sec

* Round-Robin Load Balancing

* Automatic Backend Health Checks
  Backends are periodically checked and automatically marked unhealthy if they fail.

* Metrics Endpoint

  A Prometheus-style metrics endpoint is available:

  /metrics

  Example output:

  requests_total 125
  auth_failures 10
  rate_limited 2
  successful_requests 80

* Logging

  The gateway uses structured logging via `tracing`.

  Each request is assigned a unique `request_id` and logs are emitted for the full request lifecycle.

  Example logs:

  request_started request_id=...
  incoming_request request_id=... path=/v1/models method=GET
  routing request_id=... upstream=api.openai.com:443
  upstream_client_error request_id=... upstream_status=HTTP/1.1 404 Not Found
  request_completed request_id=... duration_ms=1149

  Log levels:

  - INFO → successful requests
  - WARN → client errors (4xx)
  - ERROR → upstream/server errors (5xx)


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

Basic Request
```
curl -H "X-API-Key: user1" http://127.0.0.1:8080/test
```

OpenAi request through gateway
---
curl http://127.0.0.1:8080/v1/chat/completions \
-H "Authorization: Bearer OPENAI_API_KEY" \
-H "X-API-Key: user1" \
-H "Content-Type: application/json" \
-d '{
  "model": "gpt-4o-mini",
  "messages": [{"role": "user", "content": "hello"}]
}'

## API Keys

API keys and rate limits are defined in:

```
api_keys.json
```

Example:

{
  "user1": 2,
  "user2": 10,
  "premium": 30
}

Meaning:

user1 → 2 requests/sec
user2 → 10 requests/sec
premium → 30 requests/sec
---

## Metrics
Metrics endpoint:

curl http://127.0.0.1:8080/metrics

Example output:

requests_total 230
auth_failures 12
rate_limited 3
successful_requests 180


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

## Future Improvements
Possible extensions:

improved HTTP parsing

response inspection & analytics

distributed rate limiting

multi-provider routing


## License

Personal project

## Contact

For questions, feedback, or collaboration

dncsoftwarehelp@gmail.com