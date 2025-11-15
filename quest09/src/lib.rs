#[derive(Debug)]
pub struct DNASequence {
    pub id: usize,
    pub sequence: Vec<u8>,
}

#[derive(Debug)]
pub struct Notes {
    pub sequences: Vec<DNASequence>,
}

impl TryFrom<&str> for Notes {
    type Error = std::io::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self {
            sequences: value
                .split("\n")
                .filter(|line| !line.is_empty())
                .map(|v| {
                    let (id, sequence) = &v
                        .split_once(":")
                        .iter()
                        .filter_map(|(i, s)| {
                            if let Ok(id) = i.parse::<usize>() {
                                Some((id, s.as_bytes().to_vec()))
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<(usize, Vec<u8>)>>()[0];
                    DNASequence {
                        id: *id,
                        sequence: sequence.clone(),
                    }
                })
                .collect(),
        })
    }
}
