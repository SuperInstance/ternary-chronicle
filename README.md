# Ternary Chronicle

**Ternary Chronicle** provides historical record and narrative generation for ternary state systems — tracking timestamped events, building timelines, generating narratives from state transitions, indexed searching, pattern-based prediction, and conservation law verification.

## Why It Matters

Every fleet generates a stream of state changes: agents switching from Explore to Choose, strategies being adopted and abandoned, population ratios oscillating. Without a chronicle, these events are lost — and without history, there's no learning. Ternary Chronicle records every transition as an immutable event, indexes it for fast retrieval, detects patterns for prediction, and generates human-readable narratives for operators. The conservation law verification ensures that recorded history is internally consistent.

## How It Works

### Event Structure

```rust
Event {
    timestamp: u64,
    agent_id: u64,
    from_state: Vec<Ternary>,
    to_state: Vec<Ternary>,
    tag: EventTag,        // Transition, Threshold, Reversal, Anomaly, Recovery
    description: String,
}
```

Event creation: **O(1)** (struct construction). Event recording: **O(1)** amortized (Vec push).

### Timeline and Indexing

```rust
Chronicle {
    events: Vec<Event>,
    by_agent: HashMap<u64, Vec<usize>>,      // agent → event indices
    by_tag: HashMap<EventTag, Vec<usize>>,    // tag → event indices
    by_time: BTreeMap<u64, Vec<usize>>,       // timestamp → event indices
}
```

- Insert: **O(log T)** (BTreeMap insert for time index)
- Query by agent: **O(1)** HashMap + **O(K)** for K events
- Range query by time: **O(log T + K)** for K results in T total timestamps

### Event Tags

```
Transition: Normal state transition
Threshold:  State crossed a configured threshold
Reversal:   Direction of change reversed
Anomaly:    Unexpected state pattern detected
Recovery:   System returned to normal after anomaly
```

Classification: **O(1)** per event (compare to thresholds).

### Pattern Detection

Identify recurring sequences:

```
for each window of W events:
    if sequence matches known pattern:
        record pattern occurrence

Patterns: oscillation (A→B→A→B), drift (gradual increase), 
          step change (sudden jump), cascade (one triggers many)
```

Pattern matching: **O(N · W)** for N events, window size W.

### Prediction

Based on detected patterns, predict next events:

```
if pattern = oscillation(period=P):
    next_event predicted at t + P
confidence = pattern_frequency × pattern_accuracy
```

Prediction: **O(P)** for P detected patterns.

### Conservation Verification

```
verify_conservation():
    for each agent:
        expected = Σ state_values
        actual = current_state_sum
        if |expected - actual| > tolerance:
            return ConservationViolation
```

Full verification: **O(N · D)** for N agents, D state dimensions.

## Quick Start

```rust
use ternary_chronicle::{Chronicle, Event, EventTag, Ternary};

let mut chronicle = Chronicle::new();
chronicle.record(Event::new(
    1000, 1,
    vec![Ternary::Zero], vec![Ternary::Positive],
    EventTag::Transition,
    "Agent 1 chose to commit"
));

let agent_history = chronicle.by_agent(1);
println!("Agent 1 has {} events", agent_history.len());
```

## API

| Type | Description |
|------|-------------|
| `Chronicle` | Event log with multi-dimensional indexing |
| `Event` | Timestamped state transition with tag and description |
| `EventTag` | Transition, Threshold, Reversal, Anomaly, Recovery |
| `Ternary` | Negative (-1), Zero (0), Positive (+1) |
| `Timeline` | Chronological event sequence |

Key methods: `record()`, `by_agent()`, `by_tag()`, `range_query()`, `detect_patterns()`.

## Architecture Notes

Ternary Chronicle provides the historical record layer for fleet auditing in SuperInstance. In γ + η = C, the chronicle tracks how γ (growth events — transitions to +1) and η (avoidance events — transitions to -1) combine over time to maintain the conservation C. Pattern detection on the chronicle enables prediction of future fleet behavior. Integrates with `ternary-archive` for persistent storage.

See [ARCHITECTURE.md](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md) for fleet history architecture.

## References

1. Lamport, L. (1978). "Time, Clocks, and the Ordering of Events in a Distributed System." *Communications of the ACM*, 21(7), 558–565.
2. Kulkarni, S. S. et al. (2012). "Event Sourcing for Distributed Systems." *IEEE Computer*.
3. Smith, B. C. (1985). "Procedural Reflection in Programming Languages." *MIT LCS TR-272*.

## License

MIT
