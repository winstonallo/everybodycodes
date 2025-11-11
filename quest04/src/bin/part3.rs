use quest04::{Gear, MountedGearList};

fn main() -> std::io::Result<()> {
    let gears = MountedGearList::try_from(include_str!("../../notes/part3.txt"))?;

    let mut running_ratio: f64 = 1.0;

    for idx in 0..gears.0.len() - 1 {
        let left = match gears.0[idx] {
            Gear::Single(x) => x,
            Gear::Mounted(_, right) => right,
        };
        let right = match gears.0[idx + 1] {
            Gear::Single(x) => x,
            Gear::Mounted(left, _) => left,
        };

        running_ratio *= left as f64 / right as f64;
    }

    println!("{}", (running_ratio * 100.0) as usize);

    Ok(())
}
