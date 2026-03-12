# Data Model

The world-model is authoritative up to the runtime frontier.

## Canonical records

- RuntimeContext
- ModIdentity
- DiscoveredXmlAsset
- UnifiedXmlState
- ResolvedInheritanceGraph
- ParsedDefBucket
- ResolvedDefGraph
- DefOfState
- ShortHashState
- SnapshotManifest
- RunDecisionState

## Rules

- The world-model is not advisory.
- Shared mutable global state should not be the architectural center.
- Parallel phases must produce shard-local outputs and deterministic commits.
- Diagnostics must be able to explain any important world-model-derived decision.