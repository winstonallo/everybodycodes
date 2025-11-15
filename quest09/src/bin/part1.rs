use std::collections::HashSet;

use quest09::Notes;

fn get_parents(child_id: usize) -> (usize, usize) {
    match child_id {
        1 => (2, 3),
        2 => (1, 3),
        3 => (1, 2),
        _ => unreachable!(),
    }
}

fn main() -> std::io::Result<()> {
    let notes = Notes::try_from(include_str!("../../notes/part1.txt"))?;

    let (s1, s2, s3) = (
        &notes.sequences[0],
        &notes.sequences[1],
        &notes.sequences[2],
    );

    let mut possible_children = HashSet::from([1, 2, 3]);
    for idx in 0..s1.sequence.len() {
        if ![s2.sequence[idx], s3.sequence[idx]].contains(&s1.sequence[idx]) {
            possible_children.remove(&1);
        }
        if ![s1.sequence[idx], s3.sequence[idx]].contains(&s2.sequence[idx]) {
            possible_children.remove(&2);
        }
        if ![s1.sequence[idx], s2.sequence[idx]].contains(&s3.sequence[idx]) {
            possible_children.remove(&3);
        }
    }

    let child = &notes.sequences[possible_children.drain().collect::<Vec<usize>>()[0] - 1];
    let (p1, p2) = get_parents(child.id);
    let (p1, p2) = (&notes.sequences[p1 - 1], &notes.sequences[p2 - 1]);

    let mut p1_common = 0;
    let mut p2_common = 0;
    for idx in 0..child.sequence.len() {
        if child.sequence[idx] == p1.sequence[idx] {
            p1_common += 1;
        }
        if child.sequence[idx] == p2.sequence[idx] {
            p2_common += 1;
        }
    }

    println!("{}", p1_common * p2_common);

    Ok(())
}
