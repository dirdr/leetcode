const N_LETTERS: usize = (b'z' - b'a' + 1) as _;

#[derive(Default)]
struct Node {
    value: i32,
    children: [usize; N_LETTERS],
}

struct Trie {
    nodes: Vec<Node>,
}

impl Trie {
    pub fn new() -> Self {
        Self { nodes: vec![Node::default()] }
    }

    pub fn insert(&mut self, s: &[u8]) {
        let mut node = 0;
        for b in s.iter().map(|b| (*b - b'a') as usize) {
            if self.nodes[node].children[b] == 0 {
                self.nodes[node].children[b] = self.nodes.len();
                self.nodes.push(Node::default());
            }
            node = self.nodes[node].children[b];
            self.nodes[node].value += 1;
        }    
    }

    pub fn find(&self, s: &[u8]) -> i32 {
        let mut node = 0;
        let mut sum = 0;
        for b in s.iter().map(|b| (*b - b'a') as usize) {
            node = self.nodes[node].children[b];
            sum += self.nodes[node].value;
        }
        sum
    }
}

impl Solution {
    pub fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
        let mut trie = Trie::new();
        for word in words.iter() {
            trie.insert(word.as_bytes());
        }
        words.iter().map(|w| trie.find(w.as_bytes())).collect::<Vec<_>>()
    }
}
