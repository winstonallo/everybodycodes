use quest04::GearList;

fn main() -> std::io::Result<()> {
    let gears = GearList::try_from(std::path::Path::new("./notes/part2.txt"))?;

    let ratio = gears.0[0] as f64 / *gears.0.last().unwrap() as f64;

    println!("{}", (10000000000000usize as f64 / ratio).round() as usize);

    Ok(())
}
