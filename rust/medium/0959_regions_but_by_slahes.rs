struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x != root_y {
            match self.rank[root_x].cmp(&self.rank[root_y]) {
                std::cmp::Ordering::Less => self.parent[root_x] = root_y,
                std::cmp::Ordering::Greater => self.parent[root_y] = root_x,
                std::cmp::Ordering::Equal => {
                    self.parent[root_y] = root_x;
                    self.rank[root_x] += 1;
                }
            }
        }
    }
}

impl Solution {
    pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
        let n = grid.len();
        let mut uf = UnionFind::new(4 * n * n);
        
        for i in 0..n {
            for j in 0..n {
                if i > 0 { uf.union(get_index(n, i-1, j, 2), get_index(n, i, j, 0)); }
                if j > 0 { uf.union(get_index(n, i, j-1, 1), get_index(n, i, j, 3)); }
                
                match grid[i].chars().nth(j).unwrap() {
                    '/' => {
                        uf.union(get_index(n, i, j, 0), get_index(n, i, j, 3));
                        uf.union(get_index(n, i, j, 1), get_index(n, i, j, 2));
                    }
                    '\\' => {
                        uf.union(get_index(n, i, j, 0), get_index(n, i, j, 1));
                        uf.union(get_index(n, i, j, 2), get_index(n, i, j, 3));
                    }
                    _ => {
                        uf.union(get_index(n, i, j, 0), get_index(n, i, j, 1));
                        uf.union(get_index(n, i, j, 0), get_index(n, i, j, 2));
                        uf.union(get_index(n, i, j, 0), get_index(n, i, j, 3));
                    }
                }
            }
        }
        
        let mut regions = 0;
        for i in 0..(4 * n * n) {
            if uf.find(i) == i {
                regions += 1;
            }
        }
        regions
    }
}

fn get_index(n: usize, row: usize, col: usize, k: usize) -> usize {
    4 * (row * n + col) + k
}
