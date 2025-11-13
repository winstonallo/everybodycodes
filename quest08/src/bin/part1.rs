use quest08::Notes;

fn main() -> std::io::Result<()> {
    let notes = Notes::try_from(include_str!("../../notes/part1.txt"))?;
    let total_nails = 32;
    let mut middle_passes = 0;

    for pair in notes.nails.windows(2) {
        if (pair[0] - pair[1]).abs() == total_nails / 2 {
            middle_passes += 1;
        }
    }
    println!("{middle_passes}");
    Ok(())
}
