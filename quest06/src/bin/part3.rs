use quest06::{Notes, Participant, Profession};

fn main() -> std::io::Result<()> {
    let content = Notes::try_from(include_str!("../../notes/part3.txt"))?;
    let mut pairs = 0;
    let window = 1000;
    let reps = 1000;
    let modulus = content.participants.len();
    let n = modulus * reps;

    let mut mentors = content.participants.iter().cycle().take(window + 1).fold(
        std::collections::HashMap::<Profession, usize>::default(),
        |mut acc, p| {
            if let Participant::Knight(prof) = p {
                *acc.entry(*prof).or_default() += 1
            }
            acc
        },
    );

    for idx in 0..n {
        if let Participant::Novice(profession) = content.participants[idx % modulus] {
            pairs += *mentors.get(&profession).unwrap_or(&0);
        }

        if idx >= window {
            let left = idx - window;
            if let Participant::Knight(profession) = content.participants[left % modulus] {
                *mentors.entry(profession).or_default() -= 1;
            }
        }

        let right = idx + window + 1;
        if right < n {
            if let Participant::Knight(profession) = content.participants[right % modulus] {
                *mentors.entry(profession).or_default() += 1;
            }
        }
    }
    println!("{pairs}");
    Ok(())
}
