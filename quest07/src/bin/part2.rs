use quest07::Notes;

fn main() -> std::io::Result<()> {
    let notes = Notes::try_from(include_str!("../../notes/part2.txt"))?;
    let mut indices = 0;

    'outer: for (outer_idx, name) in notes.names.iter().enumerate() {
        for inner_idx in 0..(name.len() - 1) {
            if !notes.rules.contains(&name[inner_idx..=inner_idx + 1]) {
                continue 'outer;
            }
        }
        indices += outer_idx + 1;
    }
    println!("{indices}");

    Ok(())
}
