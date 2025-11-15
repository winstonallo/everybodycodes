use std::collections::HashSet;

use quest09::Notes;

fn is_valid_child(child: &[u8], p1: &[u8], p2: &[u8]) -> bool {
    child
        .iter()
        .zip(p1.iter().zip(p2.iter()))
        .all(|(c, (p1, p2))| c == p1 || c == p2)
}

fn main() -> std::io::Result<()> {
    let notes = Notes::try_from(include_str!("../../notes/part2.txt"))?;
    let seqs = &notes.sequences;

    let mut sum = 0;
    let mut seen = HashSet::new();

    for (child_idx, child) in seqs.iter().enumerate() {
        for (p1_idx, p1) in seqs.iter().enumerate() {
            if p1_idx == child_idx {
                continue;
            }
            for (p2_idx, p2) in seqs.iter().enumerate() {
                if p2_idx == child_idx {
                    continue;
                }
                let family_id = vec![p1.id.min(p2.id), p1.id.max(p2.id), child.id];
                if seen.contains(&family_id) {
                    continue;
                }
                seen.insert(family_id);
                if is_valid_child(&child.sequence, &p1.sequence, &p2.sequence) {
                    let p1_common = child
                        .sequence
                        .iter()
                        .zip(&p1.sequence)
                        .filter(|(c, p)| c == p)
                        .count();
                    let p2_common = child
                        .sequence
                        .iter()
                        .zip(&p2.sequence)
                        .filter(|(c, p)| c == p)
                        .count();

                    sum += p1_common * p2_common;
                }
            }
        }
    }
    println!("{sum}");

    Ok(())
}
