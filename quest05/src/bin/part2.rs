use std::usize;

use quest05::VecNotes;

fn main() -> std::io::Result<()> {
    let notes = VecNotes::try_from(include_str!("../../notes/part2.txt"))?;

    let mut max_quality = 0;
    let mut min_quality = usize::MAX;
    for note in notes.notes {
        let mut segments = vec![vec![0, note.nums[0], 0]];
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
        if quality < min_quality {
            min_quality = quality;
        } else if quality > max_quality {
            max_quality = quality;
        }
    }

    println!("{}", max_quality - min_quality);

    Ok(())
}
