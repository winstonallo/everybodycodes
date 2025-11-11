use quest06::{Notes, Participant, Profession};

fn main() -> std::io::Result<()> {
    let content = Notes::try_from(include_str!("../../notes/part1.txt"))?;
    let mut pairs = 0;
    let mut mentors = std::collections::HashMap::<Profession, usize>::new();
    for p in content.participants {
        match p {
            Participant::Knight(Profession::Swordfighting) => {
                *mentors.entry(Profession::Swordfighting).or_insert(0) += 1
            }
            Participant::Novice(Profession::Swordfighting) => {
                pairs += *mentors.entry(Profession::Swordfighting).or_insert(0)
            }
            _ => continue,
        };
    }
    println!("{pairs}");
    Ok(())
}
