use quest07::Notes;

fn main() -> std::io::Result<()> {
    let notes = Notes::try_from(include_str!("../../notes/part1.txt"))?;

    'outer: for name in notes.names {
        for idx in 0..(name.len() - 1) {
            if !notes.rules.contains(&name[idx..=idx + 1]) {
                continue 'outer;
            }
        }
        println!("{name}");
        break;
    }

    Ok(())
}
