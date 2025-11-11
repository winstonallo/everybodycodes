use quest04::GearList;

fn main() -> std::io::Result<()> {
    let gears = GearList::try_from(include_str!("../../notes/part1.txt"))?;

    let ratio = gears.0[0] as f64 / *gears.0.last().unwrap() as f64;

    println!("{}", (ratio * 2025.0) as usize);

    Ok(())
}
