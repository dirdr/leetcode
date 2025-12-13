use std::collections::HashMap;

impl Solution {
    pub fn validate_coupons(code: Vec<String>, business_line: Vec<String>, is_active: Vec<bool>) -> Vec<String> {
        let mut business_order = HashMap::from([("electronics", 0), ("grocery", 1), ("pharmacy", 2), ("restaurant", 3)]);
        let mut valids = (0..code.len()).filter(|&idx| {
            if !is_active[idx] {
                return false;
            }
            if !business_order.contains_key(&business_line[idx].as_str()) {
                return false;
            }
            if code[idx].is_empty() || !code[idx].chars().all(|ch| ch.is_ascii_alphabetic() || ch.is_ascii_digit() || ch == '_') {
                return false;
            }
            true
        }).collect::<Vec<_>>();
        valids.sort_unstable_by(|&a, &b| business_order[business_line[a].as_str()].cmp(&business_order[business_line[b].as_str()])
            .then(code[a].cmp(&code[b])));
        valids.into_iter().map(|idx| code[idx].clone()).collect::<Vec<_>>()
    }
}
