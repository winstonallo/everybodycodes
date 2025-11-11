#[derive(Debug)]
pub enum Participant {
    Novice(Profession),
    Knight(Profession),
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Profession {
    Swordfighting,
    Archery,
    Magic,
}

#[derive(Debug)]
pub struct Notes {
    pub participants: Vec<Participant>,
}

impl TryFrom<&str> for Notes {
    type Error = std::io::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self {
            participants: value
                .as_bytes()
                .iter()
                .filter_map(|b| match b {
                    b'A' => Some(Participant::Knight(Profession::Swordfighting)),
                    b'a' => Some(Participant::Novice(Profession::Swordfighting)),
                    b'B' => Some(Participant::Knight(Profession::Archery)),
                    b'b' => Some(Participant::Novice(Profession::Archery)),
                    b'C' => Some(Participant::Knight(Profession::Magic)),
                    b'c' => Some(Participant::Novice(Profession::Magic)),
                    _ => None,
                })
                .collect(),
        })
    }
}
