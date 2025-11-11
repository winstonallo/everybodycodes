use quest01::{Data, Move};

fn main() -> std::io::Result<()> {
    let data = Data::try_from(include_str!("../../notes/part2.txt"))?;

    let len = data.names.len() as isize;
    let mut curr_idx = 0;
    for m in data.moves {
        match m {
            Move::Left(by) => curr_idx = (curr_idx - by + len) % len,
            Move::Right(by) => curr_idx = (curr_idx + by + len) % len,
        }
    }

    println!("{}", data.names[curr_idx as usize]);

    Ok(())
}
