# AGENTS.md — workflow

## Serves

- **O1 — governed reach.** Every effectful workflow step executes under current, attributable
  authority and produces a named refusal when that authority is absent.
- **O4 — products run on the foundation.** The service consumes released Identity, Connector,
  Agent Platform, AEP and execution contracts rather than predecessor runtime code.
- **O5 — the generic agent platform.** Users create, revise, trigger, run and inspect workflows
  through a product-neutral API.

## Boundary

This public repository owns workflow definitions, immutable revisions, activation, triggers,
runs, leases, step state and workflow evidence. Connector credentials, agents, engineering
artifacts, identities and sandboxes remain with their owning services.

The predecessor is a semantic reference only. Preserve useful API concepts through an explicit
compatibility map; do not import its runtime or persistence coupling.

## Planning

`.engineering/planning/` is written only through `protocol artifact`. Run
`protocol artifact list` before planning changes and `protocol artifact validate --strict` after.
Anything that runs is Rust. Never store credential bytes or organization-specific deployment data.
