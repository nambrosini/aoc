use std::{cmp, collections::VecDeque, ops::RangeInclusive};

pub fn merge_ranges<T: Copy + Clone + Ord>(fresh: &[RangeInclusive<T>]) -> Vec<RangeInclusive<T>> {
    let mut ranges = fresh.to_vec();
    ranges.sort_by_key(|r| *r.start());
    let mut ranges: VecDeque<RangeInclusive<T>> = ranges.iter().cloned().collect();
    let mut merged_ranges: Vec<RangeInclusive<T>> = Vec::new();

    while let Some(current_range) = ranges.pop_front() {
        if let Some(next_range) = ranges.front()
            && current_range.end() >= next_range.start()
        {
            let new_end = cmp::max(*current_range.end(), *next_range.end());
            let new_range = *current_range.start()..=new_end;

            ranges.pop_front();
            ranges.push_front(new_range);
        } else {
            merged_ranges.push(current_range);
        }
    }

    merged_ranges
}
