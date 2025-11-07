use quest01::{Data, Move};

fn main() -> std::io::Result<()> {
    let data = Data::try_from("./notes/part1.txt")?;

    let max_idx = data.names.len() - 1;
    let mut curr_idx = 0;

    for m in data.moves {
        curr_idx = match m {
            Move::Left(by) => std::cmp::max(0, curr_idx - by),
            Move::Right(by) => std::cmp::min(max_idx as isize, curr_idx + by),
        };
    }

    println!("{}", data.names[curr_idx as usize]);

    Ok(())
}
