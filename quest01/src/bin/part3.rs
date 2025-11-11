use quest01::{Data, Move};

fn main() -> std::io::Result<()> {
    let mut data = Data::try_from(include_str!("../../notes/part3.txt"))?;

    let len = data.names.len() as isize;

    for m in data.moves {
        let offset = match m {
            Move::Left(by) => -by,
            Move::Right(by) => by,
        };

        let swap_pos = ((offset % len) + len) % len;

        data.names.swap(0, swap_pos as usize);
    }

    println!("{}", data.names[0]);
    Ok(())
}
