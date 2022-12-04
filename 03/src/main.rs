use std::collections::BTreeSet;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub struct Item {
    score: u8,
}

impl TryFrom<&u8> for Item {
    type Error = ();

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            97..=122 => Ok(Item { score: value - 96 }),
            65..=90 => Ok(Item {
                score: value - 64 + 26,
            }),
            _ => Err(()),
        }
    }
}

#[derive(Clone)]
pub struct Rucksack {
    left: BTreeSet<Item>,
    right: BTreeSet<Item>,
}

impl Rucksack {
    pub fn parse(value: &str) -> Result<Self, ()> {
        let value = value.trim();
        let (left, right) = value.split_at(value.len() / 2);
        eprintln!("{left} {right}");
        let left: BTreeSet<Item> = left
            .as_bytes()
            .iter()
            .map(|c| c.try_into().unwrap())
            .collect();
        let right: BTreeSet<Item> = right
            .as_bytes()
            .iter()
            .map(|c| c.try_into().unwrap())
            .collect();
        Ok(Rucksack { left, right })
    }
}

impl TryFrom<&str> for Rucksack {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Rucksack::parse(value)
    }
}

impl Rucksack {
    pub fn priority(&self) -> u32 {
        let overlap = self.left.intersection(&self.right);
        overlap.fold(0, |a, i| a + i.score as u32)
    }

    pub fn make_party(value: &str) -> Vec<Self> {
        value.split("\n").map(|s| s.try_into().unwrap()).collect()
    }

    pub fn by_group(value: &str) -> Vec<(Rucksack, Rucksack, Rucksack)> {
        Rucksack::make_party(value)
            .chunks(3)
            .map(|c| (c[0].clone(), c[1].clone(), c[2].clone()))
            .collect()
    }

    pub fn find_badge(elves: &(Rucksack, Rucksack, Rucksack)) -> Item {
        let (a, b, c) = elves;
        let a: BTreeSet<Item> = a.right.union(&a.left).map(|i| i.clone()).collect();
        let b: BTreeSet<Item> = b.right.union(&b.left).map(|i| i.clone()).collect();
        let c: BTreeSet<Item> = c.right.union(&c.left).map(|i| i.clone()).collect();

        let s: BTreeSet<Item> = a.intersection(&b).map(|i| i.clone()).collect();
        let s: BTreeSet<Item> = s.intersection(&c).map(|i| i.clone()).collect();

        s.iter().next().unwrap().to_owned()
    }
}

pub fn sum_rucksacks(sacks: Vec<Rucksack>) -> u32 {
    sacks.iter().fold(0, |a, s| a + s.priority())
}

const INV_STR: &str = include_str!("../data");

fn main() {
    // let inventory: Vec<Rucksack> = Rucksack::make_party(INV_STR);
    // let answer = sum_rucksacks(inventory);
    let groups = Rucksack::by_group(INV_STR);
    let sum = groups
        .iter()
        .fold(0u32, |a, g| a + Rucksack::find_badge(g).score as u32);
    eprintln!("{sum}");
}

#[cfg(test)]
mod test {
    use crate::{sum_rucksacks, Rucksack};

    fn assert_priority(value: &str, priority: u32) {
        let rucksack: Rucksack = value.try_into().unwrap();
        assert_eq!(rucksack.priority(), priority);
    }

    #[test]
    fn makes_rucksack() {
        assert_priority("vJrwpWtwJgWrhcsFMMfFFhFp", 16);
        assert_priority("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", 38);
        assert_priority("PmmdzqPrVvPwwTWBwg", 42);
        assert_priority("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", 22);
        assert_priority("ttgJtRGJQctTZtZT", 20);
        assert_priority("CrZsJsPPZsGzwwsLwLmpwMDw", 19);
    }

    #[test]
    fn make_party() {
        let rucksacks = Rucksack::make_party(
            "vJrwpWtwJgWrhcsFMMfFFhFp
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    PmmdzqPrVvPwwTWBwg
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    ttgJtRGJQctTZtZT
    CrZsJsPPZsGzwwsLwLmpwMDw",
        );

        assert_eq!(rucksacks.len(), 6);

        assert_eq!(sum_rucksacks(rucksacks), 157);
    }

    #[test]
    fn find_badge() {
        let groups = Rucksack::by_group(
            "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg",
        );

        let badge = Rucksack::find_badge(&groups[0]);
        assert_eq!(badge.score, 18);
    }

    #[test]
    fn find_badges() {
        let groups = Rucksack::by_group(
            "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw",
        );

        let sum = groups
            .iter()
            .fold(0u32, |a, g| a + Rucksack::find_badge(g).score as u32);

        assert_eq!(sum, 70);
    }
}
