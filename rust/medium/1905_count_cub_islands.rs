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
    pub fn count_sub_islands(grid1: Vec<Vec<i32>>, grid2: Vec<Vec<i32>>) -> i32 {
        let mut uf = Self::create_uf_from_grid(&grid2);
        let (m, n) = (grid2.len(), grid2[0].len());
        let mut is_subisland = vec![true; m * n];
        let mut roots = std::collections::HashSet::new();
        for i in 0..m {
            for j in 0..n {
                if grid2[i][j] == 1 {
                    let root = uf.find(i * n + j);
                    roots.insert(root);
                    if grid1[i][j] == 0 {
                        is_subisland[root] = false;
                    }
                }
            }
        }
        roots.into_iter().filter(|&root| is_subisland[root]).count() as i32
    }

    fn create_uf_from_grid(grid: &Vec<Vec<i32>>) -> UnionFind {
        let (m, n) = (grid.len(), grid[0].len());
        let directions: [(isize, isize); 4] = [
            (-1, 0),
            (0, 1),
            (1, 0),
            (0, -1)
        ];
        let mut uf = UnionFind::new(n * m);
        for row in 0..m {
            for col in 0..n {
                for direction in directions {
                    let new_row = row as isize + direction.0;
                    let new_col = col as isize + direction.1;
                    if new_row < 0 || new_row as usize >= m || new_col < 0 || new_col as usize >= n {
                        continue
                    }
                    if grid[row][col] == 0 || grid[new_row as usize][new_col as usize] == 0 {
                        continue;
                    }
                    uf.union(new_row as usize * n + new_col as usize, row * n + col);
                }
            }
        }
        return uf;
    }
}
