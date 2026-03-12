# Module Map

## Managed shell
Responsibilities:
- package discovery and path resolution
- runtime revision and environment capture
- native loading
- ABI validation
- startup entry into Rust
- mixed-zone managed assistance
- structured diagnostics surfacing

## Native core
Responsibilities:
- world-model
- fingerprinting and invalidation
- snapshot manifest logic
- snapshot load and store logic
- XML and patch normalization
- execution planning
- equivalence validation
- compatibility classification
- observability state

## Planned Rust-core module families

- bootstrap
- package_resolver
- modset_model
- asset_discovery
- xml_pipeline
- parser_lane_manager
- def_pipeline
- mixed_zone_bridge
- xref_and_resolve
- defof_and_hash
- snapshot
- equivalence
- compatibility_classifier
- diagnostics
- benchmarking
- task_scheduler
- deterministic_reducer