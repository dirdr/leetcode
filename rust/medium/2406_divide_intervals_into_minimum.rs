use std::cmp::Ordering;

enum EventType {
    Start,
    End,
}

struct Event {
    time: i32,
    event_type: EventType,
}

impl Event {
    pub fn new(time: i32, event_type: EventType) -> Self {
        Event { time, event_type }
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

impl Eq for Event {}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.time.cmp(&other.time) {
            Ordering::Equal => {
                match (&self.event_type, &other.event_type) {
                    (EventType::End, EventType::Start) => Ordering::Less,
                    (EventType::Start, EventType::End) => Ordering::Greater,
                    _ => Ordering::Equal,
                }
            }
            other_order => other_order,
        }
    }
}

impl Solution {
    pub fn min_groups(intervals: Vec<Vec<i32>>) -> i32 {
        // O(Nlog(N)) TC - O(N) SC
        let mut events = vec![];
        for (start, end) in intervals.iter().map(|i| (i[0], i[1])) {
            events.push(Event::new(start, EventType::Start));
            events.push(Event::new(end + 1, EventType::End));
        }
        events.sort_unstable();
        let mut count = 0;
        let mut max = 0;
        for event in events {
            match event.event_type {
                EventType::Start => count += 1,
                EventType::End => count -= 1,
            }
            max = max.max(count);
        }
        max
    }
}
