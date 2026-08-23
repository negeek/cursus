# Cursus

> **Work in progress — not yet usable.**

> *Cursus Honorum* — the Roman sequential path of offices climbed in strict order.
> No skipping. No shortcuts. Every step unlocks the next.

Cursus decides **when** your work runs. It never runs the work itself.

You create a workflow: each step is an HTTP endpoint you own, and you say which
steps must finish before which. Cursus calls each endpoint when its dependencies
are done, tracks what succeeded, and retries what did not.

## The one thing that makes it different

Every workflow tool has to solve "run this, then that". They almost all solve it
by owning execution: your code runs inside their worker, their runtime, their
sandbox.

Cursus has no code execution surface at all. It makes an HTTP call and waits for
an answer. That is the whole mechanism.

What follows from that:

- **Nothing to escape from**, because there is no sandbox to escape. Cursus never
  loads, compiles, or runs anything you wrote.
- **Any language, no SDK required.** If it can serve HTTP, it can be a step.
- **Small to operate.** Postgres and one binary. No message broker, no worker
  fleet to size, no runtime to patch per language.
- **Your infrastructure stays yours.** Steps run where your code already runs,
  with the secrets, network, and database access it already has.

The trade is real and worth stating plainly: a tool that owns execution can do
things Cursus cannot, like capturing a stack trace from inside your function or
pausing mid-function and resuming later. Cursus sees an endpoint and a result.
If you want the orchestrator to be your runtime, use one that owns execution.
Cursus is for when you would rather it did not.

## The problem this creates, and how it gets solved

A task endpoint usually cannot finish the work before it replies. It accepts the
request, returns immediately, and does the job in the background. So a `200 OK`
means "received", not "succeeded" — and a workflow cannot advance on "received".

Cursus hands every dispatch a single-use token. When the work actually finishes,
the task reports back with that token and says how it went. The step completes
then, not when the HTTP call returned.

How you do the work in between is your choice and Cursus has no opinion on it:
`tokio::spawn`, a Python thread, a goroutine, or a queue you already run like
Celery, Hatchet, or Temporal.

## Status and direction

Cursus is one API today, and that is the foundation everything else sits on.

| Piece | State | What it is |
|---|---|---|
| **Engine (HTTP API)** | in progress | Accounts, tasks, workflows, steps, and edges. |
| **GUI** | planned | Build and watch workflows without writing calls by hand. |
| **SDK** | planned | Two halves: receive a dispatch and report its result back with the token, and a typed client for creating tasks and workflows from code instead of by hand. |

Two commitments about the parts that do not exist yet.

**Anything the GUI or SDK can do, you can do with curl.** Neither of them gets an
endpoint you do not have, because both are only making the same documented calls
underneath.

**The SDK stays optional.** It removes boilerplate; it is not a framework to
build against, and nothing requires it.

## Prerequisites

- [Docker](https://docs.docker.com/get-docker/) & Docker Compose

## Running the Application

```bash
# Start the API server and PostgreSQL database
make run
```

Copy `.env.example` to `.env` and update the values before starting. The server
applies its own migrations at startup, so there is no separate migration step.

## API Documentation

Once the server is running, the interactive API docs are available at:

| URL | Description |
|-----|-------------|
| `http://localhost:8080/docs/` | Swagger UI — browse and try endpoints |
| `http://localhost:8080/api-docs/openapi.json` | Raw OpenAPI spec |
