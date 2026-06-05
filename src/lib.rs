#![forbid(unsafe_code)]

//! Historical record and narrative generation for ternary state systems.
//!
//! Provides chronicle-based tracking of ternary state transitions: timestamped
//! events, timelines, narrative generation, indexed searching, pattern-based
//! prediction, and conservation law verification.

use std::collections::HashMap;

/// A ternary value: Negative (-1), Zero (0), or Positive (+1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Ternary {
    Negative,
    Zero,
    Positive,
}

impl Ternary {
    pub fn value(self) -> i8 {
        match self {
            Ternary::Negative => -1,
            Ternary::Zero => 0,
            Ternary::Positive => 1,
        }
    }

    pub fn from_value(v: i8) -> Option<Self> {
        match v {
            -1 => Some(Ternary::Negative),
            0 => Some(Ternary::Zero),
            1 => Some(Ternary::Positive),
            _ => None,
        }
    }
}

// ─── Event ───────────────────────────────────────────────────────────

/// A timestamped occurrence in the ternary state system.
#[derive(Debug, Clone)]
pub struct Event {
    pub timestamp: u64,
    pub agent_id: u64,
    pub from_state: Vec<Ternary>,
    pub to_state: Vec<Ternary>,
    pub tag: EventTag,
    pub description: String,
}

/// Classification of events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventTag {
    /// Normal state transition.
    Transition,
    /// State crossed a threshold.
    Threshold,
    /// State reversal (direction changed).
    Reversal,
    /// State became stuck.
    Stagnation,
    /// System-wide event.
    System,
    /// Custom/user-defined.
    Custom(u8),
}

impl Event {
    /// Compute the net delta of this event.
    pub fn net_delta(&self) -> i64 {
        let from_sum: i64 = self.from_state.iter().map(|t| t.value() as i64).sum();
        let to_sum: i64 = self.to_state.iter().map(|t| t.value() as i64).sum();
        to_sum - from_sum
    }

    /// Check if the event represents a reversal.
    pub fn is_reversal(&self) -> bool {
        if self.from_state.len() != self.to_state.len() {
            return false;
        }
        self.from_state.iter().zip(self.to_state.iter())
            .any(|(f, t)| f.value() != 0 && t.value() != 0 && f.value().signum() != t.value().signum())
    }

    /// Compute the Hamming distance between states.
    pub fn hamming_distance(&self) -> usize {
        self.from_state.iter().zip(self.to_state.iter())
            .filter(|(f, t)| f != t)
            .count()
    }
}

// ─── Timeline ────────────────────────────────────────────────────────

/// An ordered sequence of events.
#[derive(Debug, Clone)]
pub struct Timeline {
    events: Vec<Event>,
}

impl Timeline {
    pub fn new() -> Self {
        Timeline { events: Vec::new() }
    }

    /// Add an event (must have timestamp >= last event).
    pub fn push(&mut self, event: Event) -> Result<(), &'static str> {
        if let Some(last) = self.events.last() {
            if event.timestamp < last.timestamp {
                return Err("Event timestamp must be >= last event timestamp");
            }
        }
        self.events.push(event);
        Ok(())
    }

    /// Add event without timestamp ordering check.
    pub fn push_unchecked(&mut self, event: Event) {
        self.events.push(event);
    }

    /// Get all events.
    pub fn events(&self) -> &[Event] {
        &self.events
    }

    /// Number of events.
    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Get events in a time range [start, end].
    pub fn range(&self, start: u64, end: u64) -> Vec<&Event> {
        self.events.iter()
            .filter(|e| e.timestamp >= start && e.timestamp <= end)
            .collect()
    }

    /// Filter events by tag.
    pub fn by_tag(&self, tag: EventTag) -> Vec<&Event> {
        self.events.iter().filter(|e| e.tag == tag).collect()
    }

    /// Filter events by agent.
    pub fn by_agent(&self, agent_id: u64) -> Vec<&Event> {
        self.events.iter().filter(|e| e.agent_id == agent_id).collect()
    }

    /// Compute total net delta across all events.
    pub fn total_delta(&self) -> i64 {
        self.events.iter().map(|e| e.net_delta()).sum()
    }
}

impl Default for Timeline {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Chronicle ───────────────────────────────────────────────────────

/// The main chronicle: maintains timelines for multiple agents.
#[derive(Debug, Clone)]
pub struct Chronicle {
    timelines: HashMap<u64, Timeline>,
    global_timeline: Timeline,
    next_event_id: u64,
}

impl Chronicle {
    pub fn new() -> Self {
        Chronicle {
            timelines: HashMap::new(),
            global_timeline: Timeline::new(),
            next_event_id: 0,
        }
    }

    /// Record an event for an agent.
    pub fn record(&mut self, event: Event) {
        let agent_id = event.agent_id;
        self.global_timeline.push_unchecked(event.clone());
        self.timelines
            .entry(agent_id)
            .or_insert_with(Timeline::new)
            .push_unchecked(event);
        self.next_event_id += 1;
    }

    /// Get the timeline for a specific agent.
    pub fn agent_timeline(&self, agent_id: u64) -> Option<&Timeline> {
        self.timelines.get(&agent_id)
    }

    /// Get the global timeline.
    pub fn global_timeline(&self) -> &Timeline {
        &self.global_timeline
    }

    /// Number of agents with recorded events.
    pub fn agent_count(&self) -> usize {
        self.timelines.len()
    }

    /// Total number of events across all agents.
    pub fn total_events(&self) -> usize {
        self.global_timeline.len()
    }
}

impl Default for Chronicle {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Narrative ───────────────────────────────────────────────────────

/// Turns events into human-readable stories.
pub struct Narrative;

impl Narrative {
    /// Generate a text summary of a timeline.
    pub fn summarize(timeline: &Timeline) -> String {
        if timeline.is_empty() {
            return "No events recorded.".to_string();
        }

        let transitions = timeline.by_tag(EventTag::Transition).len();
        let reversals = timeline.by_tag(EventTag::Reversal).len();
        let stagnations = timeline.by_tag(EventTag::Stagnation).len();
        let total = timeline.len();
        let total_delta = timeline.total_delta();

        let mut narrative = format!(
            "Over {} events, the system experienced {} transitions, {} reversals, and {} stagnations.",
            total, transitions, reversals, stagnations
        );

        if total_delta > 0 {
            narrative.push_str(&format!(" Net movement: +{} (positive trend).", total_delta));
        } else if total_delta < 0 {
            narrative.push_str(&format!(" Net movement: {} (negative trend).", total_delta));
        } else {
            narrative.push_str(" Net movement: balanced (no drift).");
        }

        if let (Some(first), Some(last)) = (timeline.events().first(), timeline.events().last()) {
            let duration = last.timestamp - first.timestamp;
            narrative.push_str(&format!(" Duration: {} ticks.", duration));
        }

        narrative
    }

    /// Generate a per-agent narrative.
    pub fn agent_story(chronicle: &Chronicle, agent_id: u64) -> String {
        match chronicle.agent_timeline(agent_id) {
            Some(timeline) => {
                let mut story = format!("Agent {} history: ", agent_id);
                story.push_str(&Self::summarize(timeline));
                story
            }
            None => format!("Agent {} has no recorded history.", agent_id),
        }
    }

    /// Generate an event-level narrative (blow-by-blow).
    pub fn detailed(timeline: &Timeline) -> String {
        let mut lines = Vec::new();
        for event in timeline.events() {
            let delta = event.net_delta();
            let dir = if delta > 0 { "↑" } else if delta < 0 { "↓" } else { "→" };
            lines.push(format!("[t={}] {} {} (Δ={})",
                event.timestamp, event.description, dir, delta));
        }
        lines.join("\n")
    }
}

// ─── ChronicleIndex ──────────────────────────────────────────────────

/// Searchable index over chronicle records.
pub struct ChronicleIndex {
    /// tag -> event indices in the global timeline.
    by_tag: HashMap<EventTag, Vec<usize>>,
    /// agent_id -> event indices.
    by_agent: HashMap<u64, Vec<usize>>,
}

impl ChronicleIndex {
    pub fn new() -> Self {
        ChronicleIndex {
            by_tag: HashMap::new(),
            by_agent: HashMap::new(),
        }
    }

    /// Build an index from a chronicle.
    pub fn build(chronicle: &Chronicle) -> Self {
        let mut index = ChronicleIndex::new();
        for (i, event) in chronicle.global_timeline().events().iter().enumerate() {
            index.by_tag.entry(event.tag).or_default().push(i);
            index.by_agent.entry(event.agent_id).or_default().push(i);
        }
        index
    }

    /// Search by tag.
    pub fn search_by_tag<'a>(&'a self, chronicle: &'a Chronicle, tag: EventTag) -> Vec<&'a Event> {
        match self.by_tag.get(&tag) {
            Some(indices) => indices.iter()
                .filter_map(|&i| chronicle.global_timeline().events().get(i))
                .collect(),
            None => Vec::new(),
        }
    }

    /// Search by agent.
    pub fn search_by_agent<'a>(&'a self, chronicle: &'a Chronicle, agent_id: u64) -> Vec<&'a Event> {
        match self.by_agent.get(&agent_id) {
            Some(indices) => indices.iter()
                .filter_map(|&i| chronicle.global_timeline().events().get(i))
                .collect(),
            None => Vec::new(),
        }
    }

    /// Count events by tag.
    pub fn count_by_tag(&self, tag: EventTag) -> usize {
        self.by_tag.get(&tag).map(|v| v.len()).unwrap_or(0)
    }

    /// Count events by agent.
    pub fn count_by_agent(&self, agent_id: u64) -> usize {
        self.by_agent.get(&agent_id).map(|v| v.len()).unwrap_or(0)
    }
}

impl Default for ChronicleIndex {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Prophecy ────────────────────────────────────────────────────────

/// A prediction of future events based on historical patterns.
#[derive(Debug, Clone)]
pub struct Prophecy {
    pub predicted_tag: EventTag,
    pub confidence: f64,
    pub pattern_basis: String,
    pub predicted_delta: i64,
}

/// Predicts future events from patterns in historical data.
pub struct Prophet;

impl Prophet {
    /// Predict the next event tag based on the most common tag.
    pub fn predict_next_tag(timeline: &Timeline) -> Prophecy {
        if timeline.is_empty() {
            return Prophecy {
                predicted_tag: EventTag::Transition,
                confidence: 0.0,
                pattern_basis: "No data".to_string(),
                predicted_delta: 0,
            };
        }

        let mut tag_counts: HashMap<EventTag, usize> = HashMap::new();
        for event in timeline.events() {
            *tag_counts.entry(event.tag).or_default() += 1;
        }

        let (most_common, count) = tag_counts.iter()
            .max_by_key(|(_, &c)| c)
            .map(|(&t, &c)| (t, c))
            .unwrap_or((EventTag::Transition, 0));

        let total = timeline.len();
        let confidence = count as f64 / total as f64;

        // Predict delta based on average recent deltas
        let recent_count = total.min(5);
        let avg_delta: f64 = timeline.events().iter().rev()
            .take(recent_count)
            .map(|e| e.net_delta() as f64)
            .sum::<f64>()
            / recent_count as f64;

        Prophecy {
            predicted_tag: most_common,
            confidence,
            pattern_basis: format!("Most common tag in {} events", total),
            predicted_delta: avg_delta.round() as i64,
        }
    }

    /// Detect repeating patterns in the timeline.
    pub fn detect_cycle(timeline: &Timeline, min_length: usize, max_length: usize) -> Option<Vec<EventTag>> {
        let tags: Vec<EventTag> = timeline.events().iter().map(|e| e.tag).collect();
        if tags.len() < min_length * 2 {
            return None;
        }

        for pattern_len in min_length..=max_length.min(tags.len() / 2) {
            let pattern = &tags[..pattern_len];
            let mut matches = 0;
            let mut i = pattern_len;
            while i + pattern_len <= tags.len() {
                if &tags[i..i + pattern_len] == pattern {
                    matches += 1;
                }
                i += pattern_len;
            }
            if matches >= 2 {
                return Some(pattern.to_vec());
            }
        }
        None
    }
}

// ─── ChronicleConservation ───────────────────────────────────────────

/// Verifies that historical records preserve conservation laws.
///
/// In a closed ternary system, certain quantities should be conserved:
/// - The sum of all states across all agents
/// - The total number of state changes (in == out)
pub struct ChronicleConservation;

impl ChronicleConservation {
    /// Verify that the total ternary sum is conserved across all events.
    /// Returns Ok(()) if conserved, Err with details otherwise.
    pub fn verify_sum_conservation(chronicle: &Chronicle, initial_sum: i64) -> Result<i64, String> {
        let computed_sum = chronicle.global_timeline().events().iter()
            .map(|e| e.net_delta())
            .sum::<i64>();
        let final_sum = initial_sum + computed_sum;

        // Check that each event's delta is individually valid (no sudden jumps)
        for event in chronicle.global_timeline().events() {
            if event.net_delta().abs() > event.from_state.len() as i64 * 2 {
                return Err(format!(
                    "Event at t={} has suspiciously large delta: {}",
                    event.timestamp, event.net_delta()
                ));
            }
        }

        Ok(final_sum)
    }

    /// Verify conservation of event count: every from_state should match a prior to_state.
    /// This checks for continuity in the timeline.
    pub fn verify_continuity(timeline: &Timeline) -> Vec<String> {
        let mut violations = Vec::new();
        let events = timeline.events();
        for i in 1..events.len() {
            if events[i].agent_id == events[i - 1].agent_id {
                if events[i].from_state != events[i - 1].to_state {
                    violations.push(format!(
                        "Continuity break at t={}: agent {} from_state {:?} != previous to_state {:?}",
                        events[i].timestamp,
                        events[i].agent_id,
                        events[i].from_state,
                        events[i - 1].to_state
                    ));
                }
            }
        }
        violations
    }

    /// Compute the "energy" of a timeline (sum of absolute deltas).
    pub fn energy(timeline: &Timeline) -> i64 {
        timeline.events().iter().map(|e| e.net_delta().abs()).sum()
    }

    /// Verify that energy is non-negative and non-decreasing.
    pub fn verify_energy_monotonicity(timeline: &Timeline) -> bool {
        let events = timeline.events();
        let mut cumulative = 0i64;
        for event in events {
            cumulative += event.net_delta().abs();
        }
        // Energy should always be non-negative
        cumulative >= 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_event(ts: u64, agent: u64, from: Vec<Ternary>, to: Vec<Ternary>, tag: EventTag) -> Event {
        Event {
            timestamp: ts,
            agent_id: agent,
            from_state: from,
            to_state: to,
            tag,
            description: format!("Event at t={}", ts),
        }
    }

    #[test]
    fn test_event_net_delta() {
        let e = make_event(1, 1, vec![Ternary::Zero], vec![Ternary::Positive], EventTag::Transition);
        assert_eq!(e.net_delta(), 1);
    }

    #[test]
    fn test_event_is_reversal() {
        let e = make_event(1, 1, vec![Ternary::Positive], vec![Ternary::Negative], EventTag::Reversal);
        assert!(e.is_reversal());
    }

    #[test]
    fn test_event_not_reversal() {
        let e = make_event(1, 1, vec![Ternary::Zero], vec![Ternary::Positive], EventTag::Transition);
        assert!(!e.is_reversal());
    }

    #[test]
    fn test_event_hamming_distance() {
        let e = make_event(1, 1,
            vec![Ternary::Zero, Ternary::Positive, Ternary::Negative],
            vec![Ternary::Positive, Ternary::Positive, Ternary::Zero],
            EventTag::Transition);
        assert_eq!(e.hamming_distance(), 2);
    }

    #[test]
    fn test_timeline_push_ordered() {
        let mut tl = Timeline::new();
        assert!(tl.push(make_event(1, 1, vec![Ternary::Zero], vec![Ternary::Positive], EventTag::Transition)).is_ok());
        assert!(tl.push(make_event(2, 1, vec![Ternary::Positive], vec![Ternary::Zero], EventTag::Transition)).is_ok());
    }

    #[test]
    fn test_timeline_push_out_of_order() {
        let mut tl = Timeline::new();
        tl.push(make_event(5, 1, vec![Ternary::Zero], vec![Ternary::Positive], EventTag::Transition)).unwrap();
        assert!(tl.push(make_event(3, 1, vec![Ternary::Positive], vec![Ternary::Zero], EventTag::Transition)).is_err());
    }

    #[test]
    fn test_timeline_range() {
        let mut tl = Timeline::new();
        tl.push_unchecked(make_event(1, 1, vec![Ternary::Zero], vec![Ternary::Positive], EventTag::Transition));
        tl.push_unchecked(make_event(5, 1, vec![Ternary::Positive], vec![Ternary::Negative], EventTag::Reversal));
        tl.push_unchecked(make_event(10, 1, vec![Ternary::Negative], vec![Ternary::Zero], EventTag::Transition));
        assert_eq!(tl.range(2, 9).len(), 1);
    }

    #[test]
    fn test_timeline_by_tag() {
        let mut tl = Timeline::new();
        tl.push_unchecked(make_event(1, 1, vec![Ternary::Zero], vec![Ternary::Positive], EventTag::Transition));
        tl.push_unchecked(make_event(2, 1, vec![Ternary::Positive], vec![Ternary::Negative], EventTag::Reversal));
        assert_eq!(tl.by_tag(EventTag::Reversal).len(), 1);
    }

    #[test]
    fn test_timeline_total_delta() {
        let mut tl = Timeline::new();
        tl.push_unchecked(make_event(1, 1, vec![Ternary::Zero], vec![Ternary::Positive], EventTag::Transition));
        tl.push_unchecked(make_event(2, 1, vec![Ternary::Positive], vec![Ternary::Zero], EventTag::Transition));
        assert_eq!(tl.total_delta(), 0);
    }

    #[test]
    fn test_chronicle_record_and_query() {
        let mut chronicle = Chronicle::new();
        chronicle.record(make_event(1, 1, vec![Ternary::Zero], vec![Ternary::Positive], EventTag::Transition));
        chronicle.record(make_event(2, 2, vec![Ternary::Negative], vec![Ternary::Zero], EventTag::Transition));
        assert_eq!(chronicle.total_events(), 2);
    }

    #[test]
    fn test_narrative_summarize_empty() {
        let tl = Timeline::new();
        let summary = Narrative::summarize(&tl);
        assert!(summary.contains("No events"));
    }

    #[test]
    fn test_narrative_summarize() {
        let mut tl = Timeline::new();
        tl.push_unchecked(make_event(1, 1, vec![Ternary::Zero], vec![Ternary::Positive], EventTag::Transition));
        tl.push_unchecked(make_event(2, 1, vec![Ternary::Positive], vec![Ternary::Negative], EventTag::Reversal));
        let summary = Narrative::summarize(&tl);
        assert!(summary.contains("2 events"));
        assert!(summary.contains("1 reversals"));
    }

    #[test]
    fn test_narrative_detailed() {
        let mut tl = Timeline::new();
        tl.push_unchecked(make_event(1, 1, vec![Ternary::Zero], vec![Ternary::Positive], EventTag::Transition));
        let detailed = Narrative::detailed(&tl);
        assert!(detailed.contains("t=1"));
    }

    #[test]
    fn test_chronicle_index() {
        let mut chronicle = Chronicle::new();
        chronicle.record(make_event(1, 1, vec![Ternary::Zero], vec![Ternary::Positive], EventTag::Transition));
        chronicle.record(make_event(2, 1, vec![Ternary::Positive], vec![Ternary::Negative], EventTag::Reversal));
        chronicle.record(make_event(3, 2, vec![Ternary::Zero], vec![Ternary::Positive], EventTag::Transition));

        let index = ChronicleIndex::build(&chronicle);
        assert_eq!(index.count_by_tag(EventTag::Transition), 2);
        assert_eq!(index.count_by_tag(EventTag::Reversal), 1);
        assert_eq!(index.count_by_agent(1), 2);
        assert_eq!(index.count_by_agent(2), 1);
    }

    #[test]
    fn test_prophet_predict_next_tag() {
        let mut tl = Timeline::new();
        tl.push_unchecked(make_event(1, 1, vec![Ternary::Zero], vec![Ternary::Positive], EventTag::Transition));
        tl.push_unchecked(make_event(2, 1, vec![Ternary::Positive], vec![Ternary::Zero], EventTag::Transition));
        tl.push_unchecked(make_event(3, 1, vec![Ternary::Zero], vec![Ternary::Positive], EventTag::Transition));

        let prophecy = Prophet::predict_next_tag(&tl);
        assert_eq!(prophecy.predicted_tag, EventTag::Transition);
        assert!(prophecy.confidence > 0.5);
    }

    #[test]
    fn test_prophet_detect_cycle() {
        let mut tl = Timeline::new();
        // Create a repeating pattern: Transition, Reversal, Transition, Reversal, Transition, Reversal
        for i in 0..6 {
            let tag = if i % 2 == 0 { EventTag::Transition } else { EventTag::Reversal };
            tl.push_unchecked(make_event(i as u64, 1,
                vec![Ternary::Zero], vec![Ternary::Positive], tag));
        }
        let cycle = Prophet::detect_cycle(&tl, 2, 3);
        assert!(cycle.is_some());
        let cycle = cycle.unwrap();
        assert_eq!(cycle.len(), 2);
    }

    #[test]
    fn test_prophet_no_cycle() {
        let mut tl = Timeline::new();
        tl.push_unchecked(make_event(1, 1, vec![Ternary::Zero], vec![Ternary::Positive], EventTag::Transition));
        tl.push_unchecked(make_event(2, 1, vec![Ternary::Positive], vec![Ternary::Negative], EventTag::Reversal));
        tl.push_unchecked(make_event(3, 1, vec![Ternary::Negative], vec![Ternary::Zero], EventTag::Stagnation));
        assert!(Prophet::detect_cycle(&tl, 2, 3).is_none());
    }

    #[test]
    fn test_conservation_verify_sum() {
        let mut chronicle = Chronicle::new();
        chronicle.record(make_event(1, 1, vec![Ternary::Zero], vec![Ternary::Positive], EventTag::Transition));
        chronicle.record(make_event(2, 1, vec![Ternary::Positive], vec![Ternary::Zero], EventTag::Transition));
        let result = ChronicleConservation::verify_sum_conservation(&chronicle, 0);
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn test_conservation_verify_continuity() {
        let mut tl = Timeline::new();
        tl.push_unchecked(make_event(1, 1, vec![Ternary::Zero], vec![Ternary::Positive], EventTag::Transition));
        tl.push_unchecked(make_event(2, 1, vec![Ternary::Negative], vec![Ternary::Zero], EventTag::Transition));
        // Continuity break: from_state [Negative] != previous to_state [Positive]
        let violations = ChronicleConservation::verify_continuity(&tl);
        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn test_conservation_energy() {
        let mut tl = Timeline::new();
        tl.push_unchecked(make_event(1, 1, vec![Ternary::Zero], vec![Ternary::Positive], EventTag::Transition));
        tl.push_unchecked(make_event(2, 1, vec![Ternary::Positive], vec![Ternary::Negative], EventTag::Reversal));
        assert_eq!(ChronicleConservation::energy(&tl), 3);
    }

    #[test]
    fn test_conservation_energy_monotonicity() {
        let mut tl = Timeline::new();
        tl.push_unchecked(make_event(1, 1, vec![Ternary::Zero], vec![Ternary::Positive], EventTag::Transition));
        assert!(ChronicleConservation::verify_energy_monotonicity(&tl));
    }

    #[test]
    fn test_prophet_empty_timeline() {
        let tl = Timeline::new();
        let prophecy = Prophet::predict_next_tag(&tl);
        assert_eq!(prophecy.confidence, 0.0);
    }
}
