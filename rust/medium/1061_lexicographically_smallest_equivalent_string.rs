struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    pub fn new() -> Self {
        Self {
            parent: (0..26).collect()
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let px = self.find(x);
        let py = self.find(y);

        if px < py {
            self.parent[py] = px;
        } else {
            self.parent[px] = py;
        }
    }
}

impl Solution {
    pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        let mut uf = UnionFind::new();
        for (a, b) in s1.chars().zip(s2.chars()) {
            uf.union((a as u8 - b'a') as usize, (b as u8 - b'a') as usize);
        }
        base_str.chars()
            .map(|c| (uf.find((c as u8 - b'a') as usize) as u8 + b'a') as char)
            .collect()
    }
}
