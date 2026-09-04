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

`.engineering/planning/` is written only through `aep artifact`. Run
`aep artifact list` before planning changes and `aep artifact validate --strict` after.
Anything that runs is Rust. Never store credential bytes or organization-specific deployment data.

<!-- b10x-docs-operations:start -->
## Public documentation operations

This repository owns the public source and presentation allowlist in `b10x.docs.yaml`. The generated credential-free `.github/workflows/b10x-docs-bundle.yml` passively packages only those declared files for the exact successful `main` commit; it must never run repository code. Atlas selects the latest successful bundle with every other catalog source, and Website plus Docs System own rendering, shared components, search, and feeds. Do not add a standalone docs deployer or put App credentials in this public repository. If Atlas catalogs a former Pages workflow, that file remains repository-owned validation: preserve its bespoke checks while keeping exact read-only permissions, an unconditional pull-request trigger, and no deployment primitives. Project Pages at `/workflow/` is only the generated stable redirect façade in `.github/workflows/b10x-docs-pages.yml`; content-only publication never rebuilds it.

From the complete organization workspace, verify the contract with a clean Atlas checkout at the current remote `main`. Set `B10X_ATLAS_CHECKOUT` to a managed Atlas worktree when the primary checkout is dirty or stale; never infer command availability from the primary alone.

```bash
atlas_checkout="${B10X_ATLAS_CHECKOUT:-atlas}"
atlas_head="$(git -C "$atlas_checkout" rev-parse HEAD)"
atlas_main="$(git -C "$atlas_checkout" ls-remote origin refs/heads/main | awk '{print $1}')"
test -z "$(git -C "$atlas_checkout" status --porcelain)"
test "$atlas_head" = "$atlas_main"
cargo run --manifest-path "$atlas_checkout/Cargo.toml" --locked -q -- \
  --store "$atlas_checkout/catalog/store" docs reconcile --workspace . --check
```

Keep internal plans, stories, ADRs, decisions, worklogs, security material, and research out of the public allowlist unless a repository authority explicitly declares them public.
<!-- b10x-docs-operations:end -->
