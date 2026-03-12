# Native core area

This directory will contain the Rust core.

## Native-core identity

The Rust core is the authoritative owner of:
- the startup world-model
- fingerprinting and invalidation
- snapshot logic
- XML and patch normalization
- execution planning
- equivalence validation
- compatibility classification
- observability state

## Planned module families

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