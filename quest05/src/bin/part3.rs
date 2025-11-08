use std::usize;

use quest05::{Sword, VecNotes};

fn main() -> std::io::Result<()> {
    let notes = VecNotes::try_from(std::path::Path::new("./notes/part3.txt"))?;

    let mut swords = Vec::new();
    for note in notes.notes {
        let mut segments = vec![(vec![0, note.nums[0], 0])];
        for n in note.nums.iter().skip(1) {
            let mut inserted = false;
            for idx in 0..segments.len() {
                if segments[idx][0] == 0 && *n < segments[idx][1] {
                    inserted = true;
                    segments[idx][0] = *n;
                    break;
                } else if segments[idx][2] == 0 && *n > segments[idx][1] {
                    segments[idx][2] = *n;
                    inserted = true;
                    break;
                } else {
                    continue;
                }
            }
            if !inserted {
                segments.push(vec![0, *n, 0]);
            }
        }
        let quality = segments
            .iter()
            .map(|s| s[1].to_string())
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        swords.push(Sword {
            id: note.id,
            segments,
            quality,
        })
    }

    swords.sort_by(|a, b| {
        if a.quality > b.quality {
            std::cmp::Ordering::Less
        } else if a.quality < b.quality {
            std::cmp::Ordering::Greater
        } else {
            for idx in 0..std::cmp::min(a.segments.len(), b.segments.len()) {
                let aq = a.segments[idx]
                    .iter()
                    .filter(|x| **x != 0)
                    .map(|s| s.to_string())
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap();
                let bq = b.segments[idx]
                    .iter()
                    .filter(|x| **x != 0)
                    .map(|s| s.to_string())
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap();
                if aq > bq {
                    return std::cmp::Ordering::Less;
                } else if aq < bq {
                    return std::cmp::Ordering::Greater;
                }
            }

            if a.id > b.id {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        }
    });

    let res = swords
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, sword)| acc + (idx + 1) * sword.id);

    println!("{res}");

    Ok(())
}
