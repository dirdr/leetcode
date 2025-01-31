struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    set_count: usize,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        UnionFind {
            parent: (0..size).collect(),
            size: vec![1; size],
            set_count: size,
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

        if px != py {
            if self.size[px] < self.size[py] {
                self.parent[px] = py;
                self.size[py] += self.size[px];
            } else {
                self.parent[py] = px;
                self.size[px] += self.size[py];
            }
            self.set_count -= 1;
        }
    }

    pub fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    pub fn set_count(&self) -> usize {
        self.set_count
    }

    pub fn get_size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        self.size[root]
    }
}

impl Solution {
    pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
        let mut uf = UnionFind::new(m * n);
        for r in 0..m {
            for c in 0..n {
                if grid[r][c] == 0 {
                    continue;
                }
                let current_cell = r * n + c;

                for &(dr, dc) in &DIRECTIONS {
                    let (nr, nc) = (r as isize + dr, c as isize + dc);
                    if nr < 0 || nr >= m as isize || nc < 0 || nc >= n as isize {
                        continue;
                    }
                    let (nr, nc) = (nr as usize, nc as usize);

                    if grid[nr][nc] == 1 {
                        uf.union(current_cell, nr * n + nc);
                    }
                }
            }
        }
        let mut max_size = uf.get_size(0);
        for r in 0..m {
            for c in 0..n {
                if grid[r][c] == 1 {
                    continue;
                }
                let mut connected_roots = std::collections::HashSet::new();
                let mut size = 1;
                for &(dr, dc) in &DIRECTIONS {
                    let (nr, nc) = (r as isize + dr, c as isize + dc);
                    if nr < 0 || nr >= m as isize || nc < 0 || nc >= n as isize {
                        continue;
                    }
                    let (nr, nc) = (nr as usize, nc as usize);
                    if grid[nr][nc] == 1 {
                        let root = uf.find(nr * n + nc);
                        if connected_roots.insert(root) {
                            size += uf.get_size(root);
                        }
                    }
                }
                max_size = max_size.max(size);
            }
        }
        max_size as i32
    }
}
