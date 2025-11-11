#[derive(Debug)]
pub enum Move {
    Right(isize),
    Left(isize),
}

impl From<String> for Move {
    fn from(value: String) -> Self {
        match &value[..1] {
            "R" => Self::Right(value[1..].parse::<isize>().unwrap()),
            "L" => Self::Left(value[1..].parse::<isize>().unwrap()),
            _ => unreachable!(),
        }
    }
}

impl From<&str> for Move {
    fn from(value: &str) -> Self {
        match &value[..1] {
            "R" => Self::Right(value[1..].parse::<isize>().unwrap()),
            "L" => Self::Left(value[1..].parse::<isize>().unwrap()),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct Data {
    pub moves: Vec<Move>,
    pub names: Vec<String>,
}

impl TryFrom<&str> for Data {
    type Error = std::io::Error;

    fn try_from(value: &str) -> Result<Self, std::io::Error> {
        let content = std::fs::read_to_string(value)?;
        let content = content.split("\n").collect::<Vec<&str>>();
        Ok(Self {
            names: content[0]
                .split(",")
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
            moves: content[2]
                .split(",")
                .collect::<Vec<&str>>()
                .into_iter()
                .map(|x| Move::from(x))
                .collect::<Vec<Move>>(),
        })
    }
}
