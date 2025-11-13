use quest08::Notes;

fn main() -> std::io::Result<()> {
    let notes = Notes::try_from(include_str!("../../notes/part2.txt"))?;
    let mut knots = 0;

    for i in 1..notes.nails.len() {
        // two lines [a, b], [c, d] intersect if their endpoints
        // alternate around the circle
        let min = notes.nails[i].min(notes.nails[i - 1]);
        let max = notes.nails[i].max(notes.nails[i - 1]);
        for w in notes.nails[..i].windows(2) {
            if (min + 1..max).contains(&w[0]) ^ (min + 1..max).contains(&w[1])
                && ![min, max].contains(&w[0])
                && ![min, max].contains(&w[1])
            {
                knots += 1;
            }
        }
    }
    println!("{knots}");

    Ok(())
}
