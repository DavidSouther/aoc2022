enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn points(&self) -> u32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }
}

impl TryFrom<&str> for RPS {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim() {
            "A" => Ok(RPS::Rock),
            "B" => Ok(RPS::Paper),
            "C" => Ok(RPS::Scissors),
            // "X" => Ok(RPS::Rock),
            // "Y" => Ok(RPS::Paper),
            // "Z" => Ok(RPS::Scissors),
            _ => Err(()),
        }
    }
}

enum Strategy {
    Win,
    Lose,
    Draw,
}

impl Strategy {
    fn points(&self) -> u32 {
        match self {
            Strategy::Win => 6,
            Strategy::Lose => 0,
            Strategy::Draw => 3,
        }
    }

    fn choice(&self, them: &RPS) -> RPS {
        match (self, them) {
            (Strategy::Win, RPS::Rock) => RPS::Paper,
            (Strategy::Win, RPS::Paper) => RPS::Scissors,
            (Strategy::Win, RPS::Scissors) => RPS::Rock,
            (Strategy::Lose, RPS::Rock) => RPS::Scissors,
            (Strategy::Lose, RPS::Paper) => RPS::Rock,
            (Strategy::Lose, RPS::Scissors) => RPS::Paper,
            (Strategy::Draw, RPS::Rock) => RPS::Rock,
            (Strategy::Draw, RPS::Paper) => RPS::Paper,
            (Strategy::Draw, RPS::Scissors) => RPS::Scissors,
        }
    }
}

impl TryFrom<&str> for Strategy {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "X" => Ok(Strategy::Lose),
            "Y" => Ok(Strategy::Draw),
            "Z" => Ok(Strategy::Win),
            _ => Err(()),
        }
    }
}

struct Round {
    pub them: RPS,
    pub us: RPS,
    pub strategy: Strategy,
}

impl Round {
    pub fn points(&self) -> u32 {
        self.strategy.points() + self.us.points()
    }
}

impl TryFrom<&str> for Round {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts = value.split(" ").into_iter();
        let them: RPS = parts.next().unwrap().try_into()?;
        let strategy: Strategy = parts.next().unwrap().try_into()?;
        let us = strategy.choice(&them);
        Ok(Round { us, them, strategy })
    }
}

struct Match {
    rounds: Vec<Round>,
}

impl TryFrom<&str> for Match {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let rounds: Vec<Round> = value.split("\n").map(|v| v.try_into().unwrap()).collect();
        Ok(Match { rounds })
    }
}

impl Match {
    pub fn points(&self) -> u32 {
        self.rounds.iter().map(|r| r.points()).sum()
    }
}

const MATCH_STR: &str = include_str!("../data");

fn main() {
    let r#match: Match = MATCH_STR.try_into().unwrap();
    let points = r#match.points();
    eprintln!("{points}");
}

#[cfg(test)]
mod test {
    use crate::{Match, Round};

    // #[test]
    // fn test_round() {
    //     let round: Round = "A Y".try_into().unwrap();
    //     assert_eq!(round.points(), 8);
    // }

    // #[test]
    // fn test_match() {
    //     let r#match: Match = "A Y\nB X\nC Z".try_into().unwrap();
    //     assert_eq!(r#match.points(), 15);
    // }

    #[test]
    fn test_round() {
        let round: Round = "A Y".try_into().unwrap();
        assert_eq!(round.points(), 4);
    }

    #[test]
    fn test_match() {
        let r#match: Match = "A Y\nB X\nC Z".try_into().unwrap();
        assert_eq!(r#match.points(), 12);
    }
}
