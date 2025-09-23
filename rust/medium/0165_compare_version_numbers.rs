impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let parse = |version: &str| -> Vec<i32> {
            version.split('.')
                .map(|n| n.chars().skip_while(|&d| d == '0').collect::<String>().parse::<i32>().unwrap_or(0))
                .collect::<Vec<_>>()
        };

        let version1 = parse(&version1);
        let version2 = parse(&version2);
        let mut i = 0;
        while i < version1.len() || i < version2.len() {
            let v1 = if i < version1.len() { version1[i] } else { 0 };
            let v2 = if i < version2.len() { version2[i] } else { 0 };
            if v1 > v2 {
                return 1;
            } 
            if v1 < v2 {
                return -1;
            }
            i += 1;
        }
        0
    }
}
