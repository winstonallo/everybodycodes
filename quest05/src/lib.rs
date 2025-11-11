#[derive(Debug, Clone)]
pub struct Fishbone {
    pub segments: Vec<Vec<usize>>,
}

#[derive(Debug)]
pub struct Notes {
    pub id: usize,
    pub nums: Vec<usize>,
}

impl TryFrom<&str> for Notes {
    type Error = std::io::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let content = std::fs::read_to_string(value)?;
        let split = content.split(":").collect::<Vec<&str>>();
        Ok(Self {
            id: split[0].parse().unwrap(),
            nums: split[1]
                .split(",")
                .filter_map(|x| x.parse::<usize>().ok())
                .collect::<Vec<usize>>(),
        })
    }
}

#[derive(Debug)]
pub struct Sword {
    pub id: usize,
    pub segments: Vec<Vec<usize>>,
    pub quality: usize,
}

#[derive(Debug)]
pub struct VecNotes {
    pub notes: Vec<Notes>,
}

impl TryFrom<&str> for VecNotes {
    type Error = std::io::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self {
            notes: std::fs::read_to_string(value)?
                .split("\n")
                .filter_map(|line| {
                    line.split_once(":").map(|(id_str, nums_str)| Notes {
                        id: id_str.parse::<usize>().unwrap(),
                        nums: nums_str
                            .split(",")
                            .map(|x| x.parse::<usize>().unwrap())
                            .collect::<Vec<usize>>(),
                    })
                })
                .collect::<Vec<Notes>>(),
        })
    }
}
