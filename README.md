# Cursus

> **Work in progress — not yet usable.**

> *Cursus Honorum* — the Roman sequential path of offices climbed in strict order. No skipping. No shortcuts. Every step unlocks the next.

Cursus is a DAG-based workflow orchestration service. You define tasks, declare dependencies between them, and register an HTTP endpoint per task. When all of a task's dependencies complete, Cursus automatically calls that task's endpoint — triggering execution on your own infrastructure.

You bring the tasks. Cursus decides when they run.

## Prerequisites

- [Docker](https://docs.docker.com/get-docker/) & Docker Compose

## Running the Application

```bash
# Start the API server and PostgreSQL database
make run
```

Copy `.env.example` to `.env` and update the values before starting.

## API Documentation

Once the server is running, the interactive API docs are available at:

| URL | Description |
|-----|-------------|
| `http://localhost:8080/docs/` | Swagger UI — browse and try endpoints |
| `http://localhost:8080/api-docs/openapi.json` | Raw OpenAPI spec |
