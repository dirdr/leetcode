use std::{collections::BinaryHeap, cmp::Reverse};

impl Solution {
    pub fn count_days(days: i32, meetings: Vec<Vec<i32>>) -> i32 {
        let mut pq = BinaryHeap::new();
        for meeting in meetings {
            pq.push((Reverse(meeting[0]), 1));
            pq.push((Reverse(meeting[1] + 1), -1));
        }

        let (mut meeting_count, mut days_without_meetings) = (0, 0);
        let mut last_meeting_day = 1;
        while let Some((Reverse(event_day), effect)) = pq.pop() {
            if event_day > days {
                break;
            }
            if meeting_count == 0 {
                days_without_meetings += (event_day - last_meeting_day);
            }
            meeting_count += effect;
            last_meeting_day = event_day;
        }
        if last_meeting_day <= days && meeting_count == 0 {
            days_without_meetings += (days - last_meeting_day + 1);
        }
        days_without_meetings
    }
}
