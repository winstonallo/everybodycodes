use itertools::Itertools;
use quest08::Notes;

fn main() -> std::io::Result<()> {
    let notes = Notes::try_from(include_str!("../../notes/part3.txt"))?;

    let mut max_knots = 0;

    for c in (1..=256).combinations(2) {
        let min = c[0];
        let max = c[1];
        let mut knots = 0;
        for w in notes.nails.windows(2) {
            if ((min + 1..max).contains(&w[0]) ^ (min + 1..max).contains(&w[1])
                && ![min, max].contains(&w[0])
                && ![min, max].contains(&w[1]))
                || ([min, max].contains(&w[0]) && [min, max].contains(&w[1]))
            {
                knots += 1;
            }
        }
        max_knots = max_knots.max(knots);
    }

    println!("{max_knots}");

    Ok(())
}
