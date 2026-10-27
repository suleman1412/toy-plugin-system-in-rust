# Graph Report - /Users/sulemahnh/project/plugin/toy-plugin  (2026-10-27)

## Corpus Check
- Corpus is ~644 words - fits in a single context window. You may not need a graph.

## Summary
- 29 nodes · 49 edges · 7 communities detected
- Extraction: 88% EXTRACTED · 8% INFERRED · 4% AMBIGUOUS · INFERRED: 4 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- [[_COMMUNITY_Plugin Transform Exports|Plugin Transform Exports]]
- [[_COMMUNITY_Host Plugin Lifecycle|Host Plugin Lifecycle]]
- [[_COMMUNITY_ABI Data Types|ABI Data Types]]
- [[_COMMUNITY_Dynamic Plugin Architecture|Dynamic Plugin Architecture]]
- [[_COMMUNITY_Transform Data Flow|Transform Data Flow]]
- [[_COMMUNITY_Host Execution Flow|Host Execution Flow]]
- [[_COMMUNITY_Uppercase Plugin Export|Uppercase Plugin Export]]

## God Nodes (most connected - your core abstractions)
1. `host` - 10 edges
2. `PluginDescriptor` - 8 edges
3. `shared C ABI` - 7 edges
4. `PluginString` - 7 edges
5. `InputBuffer` - 5 edges
6. `plugin_entry` - 4 edges
7. `plugin_entry` - 4 edges
8. `dynamic plugin design` - 4 edges
9. `transform()` - 3 edges
10. `PLUGIN_ABI_VERSION` - 3 edges

## Surprising Connections (you probably didn't know these)
- `dynamic plugin design` --rationale_for--> `host`  [EXTRACTED]
  README.md → crates/host/src/main.rs
- `host` --calls--> `free_string`  [AMBIGUOUS]
  crates/host/src/main.rs → plugins/uppercase/src/lib.rs
- `host` --calls--> `plugin_entry`  [INFERRED]
  crates/host/src/main.rs → plugins/uppercase/src/lib.rs
- `host` --calls--> `free_string`  [AMBIGUOUS]
  crates/host/src/main.rs → plugins/size/src/lib.rs
- `dynamic plugin design` --rationale_for--> `shared C ABI`  [EXTRACTED]
  README.md → crates/plugin-api/src/lib.rs

## Communities

### Community 0 - "Plugin Transform Exports"
Cohesion: 0.47
Nodes (3): free_string(), plugin_entry(), transform()

### Community 1 - "Host Plugin Lifecycle"
Cohesion: 0.6
Nodes (6): host, PluginEntry, PLUGIN_ABI_VERSION, PluginDescriptor, free_string, plugin_entry

### Community 2 - "ABI Data Types"
Cohesion: 0.5
Nodes (3): InputBuffer, PluginDescriptor, PluginString

### Community 3 - "Dynamic Plugin Architecture"
Cohesion: 0.83
Nodes (4): shared C ABI, dynamic plugin design, size plugin, uppercase plugin

### Community 4 - "Transform Data Flow"
Cohesion: 0.67
Nodes (4): InputBuffer, PluginString, transform, transform

### Community 5 - "Host Execution Flow"
Cohesion: 1.0
Nodes (2): main(), run()

### Community 6 - "Uppercase Plugin Export"
Cohesion: 1.0
Nodes (2): free_string, plugin_entry

## Ambiguous Edges - Review These
- `host` → `free_string`  [AMBIGUOUS]
  crates/host/src/main.rs · relation: calls
- `host` → `free_string`  [AMBIGUOUS]
  crates/host/src/main.rs · relation: calls

## Knowledge Gaps
- **2 isolated node(s):** `InputBuffer`, `PluginDescriptor`
  These have ≤1 connection - possible missing edges or undocumented components.
- **Thin community `Host Execution Flow`** (3 nodes): `main.rs`, `main()`, `run()`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Uppercase Plugin Export`** (2 nodes): `free_string`, `plugin_entry`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **What is the exact relationship between `host` and `free_string`?**
  _Edge tagged AMBIGUOUS (relation: calls) - confidence is low._
- **What is the exact relationship between `host` and `free_string`?**
  _Edge tagged AMBIGUOUS (relation: calls) - confidence is low._
- **Why does `host` connect `Host Plugin Lifecycle` to `Dynamic Plugin Architecture`, `Transform Data Flow`, `Uppercase Plugin Export`?**
  _High betweenness centrality (0.082) - this node is a cross-community bridge._
- **Why does `shared C ABI` connect `Dynamic Plugin Architecture` to `Host Plugin Lifecycle`, `Transform Data Flow`?**
  _High betweenness centrality (0.057) - this node is a cross-community bridge._
- **Are the 2 inferred relationships involving `host` (e.g. with `plugin_entry` and `plugin_entry`) actually correct?**
  _`host` has 2 INFERRED edges - model-reasoned connections that need verification._
- **What connects `InputBuffer`, `PluginDescriptor` to the rest of the system?**
  _2 weakly-connected nodes found - possible documentation gaps or missing edges._