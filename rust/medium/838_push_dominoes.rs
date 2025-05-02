impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let chars: Vec<char> = dominoes.chars().collect();
        let n = chars.len();
        let mut right_force = vec![0; n];
        let mut left_force = vec![0; n];
        let mut force = 0;
        for i in 0..n {
            match chars[i] {
                'R' => force = n as i32,
                'L' => force = 0,
                '.' => {
                    if force > 0 {
                        force -= 1;
                    }
                },
                _ => unreachable!(),
            }
            right_force[i] = force;
        }
        force = 0;
        for i in (0..n).rev() {
            match chars[i] {
                'L' => force = n as i32,
                'R' => force = 0,
                '.' => {
                    if force > 0 {
                        force -= 1;
                    }
                },
                _ => unreachable!(),
            }
            left_force[i] = force;
        }
        let mut result = String::with_capacity(n);
        for i in 0..n {
            match chars[i] {
                'R' | 'L' => result.push(chars[i]),
                '.' => {
                    if right_force[i] > left_force[i] {
                        result.push('R');
                    } else if left_force[i] > right_force[i] {
                        result.push('L');
                    } else {
                        result.push('.');
                    }
                },
                _ => unreachable!(),
            }
        }
        
        result
    }
}
