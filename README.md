# ternary-chronicle: Historical record and narrative generation for ternary state systems

Timestamped events, timelines, narrative summaries, indexed search, pattern-based prediction, and conservation law verification for {-1, 0, +1} state sequences.

## Why This Exists

Ternary systems produce sequences of state transitions, and without a way to record and reason about those transitions, you lose institutional memory. This crate gives you a chronicle — a structured historical record that lets you search the past, generate human-readable narratives, detect repeating patterns, and verify that your history obeys conservation laws. It's the fleet's logbook.

## Core Concepts

- **Ternary**: A value in {-1, 0, +1}. Negative, Zero, or Positive.
- **Event**: A timestamped occurrence: agent transitions from one state vector to another, tagged with a classification.
- **EventTag**: Classification — Transition, Threshold, Reversal, Stagnation, System, or Custom.
- **Timeline**: An ordered (monotonic-timestamp) sequence of events with range and filter queries.
- **Chronicle**: The top-level container managing per-agent timelines and a global timeline.
- **Narrative**: Generates human-readable summaries and detailed blow-by-blow accounts from timelines.
- **ChronicleIndex**: A searchable index built from a chronicle, supporting tag and agent queries.
- **Prophecy**: Pattern-based prediction of future events from historical trends.
- **ChronicleConservation**: Verifies that historical records obey conservation laws (sum conservation, continuity, energy).

## Quick Start

```toml
# Cargo.toml
[dependencies]
ternary-chronicle = "0.1"
```

```rust
use ternary_chronicle::*;

let mut chronicle = Chronicle::new();

// Record events
chronicle.record(Event {
    timestamp: 100,
    agent_id: 1,
    from_state: vec![Ternary::Zero],
    to_state: vec![Ternary::Positive],
    tag: EventTag::Transition,
    description: "Agent 1 activated".to_string(),
});
chronicle.record(Event {
    timestamp: 200,
    agent_id: 1,
    from_state: vec![Ternary::Positive],
    to_state: vec![Ternary::Negative],
    tag: EventTag::Reversal,
    description: "Agent 1 reversed".to_string(),
});

// Generate a narrative
let summary = Narrative::summarize(chronicle.global_timeline());
println!("{}", summary);

// Search with an index
let index = ChronicleIndex::build(&chronicle);
assert_eq!(index.count_by_tag(EventTag::Reversal), 1);
```

## API Overview

| Type | Description |
|------|-------------|
| `Chronicle` | Top-level container for per-agent and global timelines |
| `Timeline` | Ordered sequence of events with range/filter queries |
| `Event` | A timestamped state transition with tag and description |
| `EventTag` | Classification of events (Transition, Reversal, etc.) |
| `Narrative` | Generates text summaries from timelines |
| `ChronicleIndex` | Searchable index over chronicle records |
| `Prophet` | Predicts future events from historical patterns |
| `ChronicleConservation` | Verifies conservation laws on historical records |

## How It Works

Events are recorded into both a per-agent timeline and a global timeline. The `Timeline` enforces monotonic timestamps — you can't insert an event before the last one. Events carry a net delta (sum of to_state minus from_state), which enables conservation tracking.

`Narrative` generates summaries by counting event tags, computing net delta, and measuring duration. `Narrative::detailed()` produces a per-event log with delta indicators (↑↓→).

`ChronicleIndex` builds hash maps from tags and agent IDs to event positions, enabling O(1) lookups.

`Prophet` uses tag frequency analysis (most common tag = most likely next tag) and autocorrelation for cycle detection. It looks for repeating patterns by testing candidate periods from min to max length.

`ChronicleConservation` provides three checks: sum conservation (initial sum + all deltas should match), continuity (each event's from_state should match the previous to_state for the same agent), and energy (cumulative absolute delta should be non-negative).

## Known Limitations

- Timeline requires monotonic timestamps; you can't insert out-of-order events (use `push_unchecked` for that, but you lose the invariant).
- Conservation checking is local (per-event), not global — it won't catch drift across non-adjacent events.
- Pattern detection uses simple autocorrelation; it won't find patterns with noise or missing events.
- Narrative generation is template-based; custom formatters are not supported.
- The index is rebuilt from scratch each time; incremental updates are not supported.
- No persistence — all data is in-memory.
- Prophecy predictions are basic frequency analysis; they don't account for context or causality.

## Use Cases

- **Audit trail**: Record every state transition in a ternary system for compliance or debugging.
- **Institutional memory**: Maintain a searchable history of agent behavior over long deployments.
- **Anomaly investigation**: Search historical records by tag (e.g., all reversals) to investigate incidents.
- **Pattern discovery**: Detect repeating cycles in agent behavior using Prophet's cycle detection.
- **Conservation verification**: Ensure that a closed ternary system obeys expected conservation laws.

## Ecosystem Context

Part of the SuperInstance ternary ecosystem. Receives events from `ternary-compass` (anomalies become events) and `ternary-dockyard` (maintenance events). Feeds historical data to `ternary-prophet` for forecasting. The chronicle is the central historical record that other crates write to and read from.

## License

MIT
