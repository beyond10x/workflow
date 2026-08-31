# Workflow

Workflow is the product-neutral service boundary for user-maintained workflow definitions,
immutable revisions, schedules and webhooks, durable runs, step leases, cancellation and evidence.

This is a clean implementation. The predecessor API is input to an explicit compatibility map,
not a source dependency. The initial repository contains the governed AEP roadmap and a minimal
Rust domain crate; HTTP and persistence begin only after their contracts are reviewed.

```console
cargo test --workspace --locked
protocol artifact validate --strict
```
