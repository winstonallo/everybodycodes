use std::collections::HashMap;

use quest09::Notes;

#[inline]
fn is_valid_child(child: &[u8], p1: &[u8], p2: &[u8]) -> bool {
    child
        .iter()
        .zip(p1.iter().zip(p2.iter()))
        .all(|(c, (p1, p2))| c == p1 || c == p2)
}

/// Union Find (Disjoint Set) keeps track of connected
/// nodes in a network.
struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    /// Create a new Union Find structure with no connections
    /// (i.e., each element is its own root).
    fn make_set(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
        }
    }

    /// Finds the root node for `x`, and performs path
    /// compression by creating a direct connection between
    /// `x` and its root.
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    /// Merges the two groups that contain `x` and `y` by making `y`
    /// the new root of `x`.
    ///
    /// Example:
    /// ```
    /// // Before
    ///   0      3   5
    /// __|__    |
    /// |   |    4
    /// 1   2
    ///
    /// uf.union(3, 0);
    ///
    /// // After
    ///   0          5
    /// __|______
    /// |   |   |
    /// 1   2   3
    ///         |
    ///         4
    /// ```
    fn union(&mut self, x: usize, y: usize) {
        let px = self.find(x);
        let py = self.find(y);
        if px != py {
            self.parent[px] = py;
        }
    }
}

fn main() -> std::io::Result<()> {
    let notes = Notes::try_from(include_str!("../../notes/part3.txt"))?;
    let seqs = &notes.sequences;

    let mut uf = UnionFind::make_set(seqs.len());

    for (child_idx, child) in seqs.iter().enumerate() {
        for (p1_idx, p1) in seqs.iter().enumerate() {
            if p1_idx == child_idx {
                continue;
            }
            for (p2_idx, p2) in seqs.iter().enumerate() {
                if p2_idx == child_idx || p2_idx <= p1_idx {
                    continue;
                }

                if is_valid_child(&child.sequence, &p1.sequence, &p2.sequence) {
                    uf.union(child_idx, p1_idx);
                    uf.union(child_idx, p2_idx);
                }
            }
        }
    }

    let mut families: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..seqs.len() {
        let root = uf.find(i);
        families.entry(root).or_insert_with(Vec::new).push(i);
    }
    let largest_family = families.values().max_by_key(|f| f.len()).unwrap();
    let sum = largest_family
        .iter()
        .map(|&idx| seqs[idx].id)
        .sum::<usize>();

    println!("{sum}");

    Ok(())
}
