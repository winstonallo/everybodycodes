use quest05::Notes;

fn main() -> std::io::Result<()> {
    let notes = Notes::try_from(include_str!("../../notes/part1.txt"))?;

    let mut segments = vec![vec![0, notes.nums[0], 0]];

    for n in notes.nums.iter().skip(1) {
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

    for s in segments {
        print!("{}", s[1]);
    }
    Ok(())
}
