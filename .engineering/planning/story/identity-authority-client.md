---
format: aep.planning-md/1
id: story:identity-authority-client
kind: story
status: implemented
title: Identity authority client
summary: Derive tenant, actor, and delegation from an Identity-owned verifier.
relations:
- derived_from: epic:authenticated-workflow-surface
- serves: vision:O1
scope:
- confidence: cited
  path: CHANGELOG.md
- confidence: cited
  path: README.md
- confidence: cited
  path: generated/client
- confidence: cited
  path: generated/connectors
- confidence: cited
  path: generated/ess/projections/openapi
- confidence: cited
  path: generated/http
- confidence: cited
  path: service.yaml
- confidence: cited
  path: service/runtime.yaml
revision: 6
---
# Identity authority and official Workflow client

## Outcome

Workflow exposes a supported tenant-scoped client boundary that downstream products can call with exchanged Identity authority.

## Context

The service boundary is generated from the same Workflow service package as the authoring model; no caller can supply tenant, actor, realm, or deployment coordinates as operation input.

## Acceptance

- The released audience is `urn:b10x:workflow`.
- Read operations require exactly `workflows.read`; authoring and draft mutation require exactly `workflows.manage`.
- Identity verification and scope checks run before request DTO decoding.
- The release includes the official typed Workflow client, generated server, OpenAPI projection, Connector contribution, stable problem codes, and probes.
- Tenant and actor come only from verified authority, optional realm remains authority context, and cross-tenant reads and writes are refused.

## Out of Scope

Deployment-specific issuer URLs, tenant identifiers, credentials, Identity repository changes, and a compatibility layer for the abandoned Platform client.

## Open Questions

None.

## Scope

- cited: Workflow `service.yaml`, generated client/server/OpenAPI/Connector artifacts, README, changelog, and conformance scenarios.
- inferred: deployment registers the opaque audience and exchange policy outside this public repository.
