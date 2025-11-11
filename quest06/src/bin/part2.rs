use quest06::{Notes, Participant, Profession};

fn main() -> std::io::Result<()> {
    let content = Notes::try_from(include_str!("../../notes/part2.txt"))?;
    let mut pairs = 0;
    let mut mentors = std::collections::HashMap::<Profession, usize>::new();
    for p in content.participants {
        match p {
            Participant::Knight(profession) => *mentors.entry(profession).or_insert(0) += 1,
            Participant::Novice(profession) => pairs += *mentors.entry(profession).or_insert(0),
        };
    }
    println!("{pairs}");
    Ok(())
}
