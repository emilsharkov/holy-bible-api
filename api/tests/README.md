# Integration Tests

This directory contains integration tests (also called end-to-end or E2E tests) for the Holy Bible API.

## What are Integration Tests?

Integration tests verify that the API endpoints work correctly by:
- Starting a real HTTP server on a random available port
- Making actual HTTP requests to the endpoints
- Verifying response status codes and data structures
- Testing the full request/response cycle including middleware, routing, and database queries

## Running the Tests

To run all integration tests:

```bash
cargo test --test '*'
```

To run a specific test file:

```bash
cargo test --test health_test
cargo test --test bibles_test
cargo test --test audio_bibles_test
```

To run with output:

```bash
cargo test --test '*' -- --nocapture
```

## Test Structure

- `common.rs` - Test helper utilities, including `TestServer` which starts the API server
- `health_test.rs` - Tests for the `/health` endpoint
- `bibles_test.rs` - Tests for all Bible-related endpoints
- `audio_bibles_test.rs` - Tests for all Audio Bible-related endpoints

## Requirements

These tests require:
- A running PostgreSQL database (configured via environment variables)
- A running Redis instance (configured via environment variables)
- AWS credentials configured (for S3 access, configured via environment variables)

The tests will use the same environment variables as the main application (loaded from `.env` file).

## How It Works

Each test:
1. Creates a `TestServer` instance which starts the API server on a random port
2. Makes HTTP requests using `reqwest` to the test server
3. Asserts that responses have the correct status codes and data structures
4. The server automatically shuts down when the test completes

The `TestServer` helper handles:
- Finding an available port automatically
- Starting the server in a background task
- Graceful shutdown when tests complete
- Providing a `reqwest::Client` for making requests

