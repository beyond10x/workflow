# Workflow

Workflow is the experimental Rust domain boundary for user-maintained workflow definitions and
immutable revisions. It is the foundation of a future product-neutral workflow service, not that
service presented as already complete.

## Current status

The repository currently contains the governed AEP roadmap and a minimal domain crate. HTTP,
persistence, schedule and webhook delivery, durable runs, step leases, cancellation, and execution
evidence remain planned. The predecessor API is input to an explicit compatibility map, not a
source dependency.

Build and validate the boundary that exists today:

```console
cargo test --workspace --locked
aep artifact validate --strict
```

<!-- b10x-docs:start -->
## Documentation

[Workflow documentation](https://beyond10x.github.io/docs/workflow/) · [Start](https://beyond10x.github.io/) · [Ecosystem](https://beyond10x.github.io/ecosystem/) · [Impact](https://beyond10x.github.io/changes/) · [Releases](https://beyond10x.github.io/releases/)
<!-- b10x-docs:end -->
