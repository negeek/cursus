# Cursus

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

The API will be available at `http://localhost:8080`.

Copy `.env.example` to `.env` and update the values before starting.
