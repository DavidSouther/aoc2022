use std::collections::HashSet;

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Pos {
    row: i32,
    col: i32,
}

#[derive(Debug, Default)]
pub struct Rope(Vec<Pos>);

// fn snap(head: &Pos, tail: &mut Pos) {
fn snap(vec: &mut Vec<Pos>, tail: usize) {
    let head = vec[tail - 1].clone();
    let tail = vec.get_mut(tail).unwrap();

    let dx = tail.row - head.row;
    if dx < -1 {
        tail.row = head.row - 1;
        tail.col = head.col;
    } else if dx > 1 {
        tail.row = head.row + 1;
        tail.col = head.col;
    }
    let dy = tail.col - head.col;
    if dy < -1 {
        tail.col = head.col - 1;
        tail.row = head.row;
    } else if dy > 1 {
        tail.col = head.col + 1;
        tail.row = head.row;
    }
}

impl Rope {
    pub fn new(len: usize) -> Self {
        let mut vec = Vec::with_capacity(len);
        for _ in 0..len {
            vec.push(Default::default())
        }
        Rope(vec)
    }

    pub fn tail(&self) -> &Pos {
        &self.0.last().unwrap()
    }

    pub fn right(&mut self) {
        self.0[0].row += 1;
    }

    pub fn left(&mut self) {
        self.0[0].row -= 1;
    }

    pub fn up(&mut self) {
        self.0[0].col += 1;
    }

    pub fn down(&mut self) {
        self.0[0].col -= 1;
    }

    pub fn r#move(&mut self, dir: &Dir) {
        match dir {
            Dir::Left => self.left(),
            Dir::Right => self.right(),
            Dir::Up => self.up(),
            Dir::Down => self.down(),
        }

        for i in 1..self.0.len() {
            snap(&mut self.0, i);
        }
    }
}

pub enum Dir {
    Left,
    Right,
    Up,
    Down,
}

impl From<char> for Dir {
    fn from(v: char) -> Self {
        match v {
            'L' => Dir::Left,
            'R' => Dir::Right,
            'U' => Dir::Up,
            'D' => Dir::Down,
            _ => todo!(),
        }
    }
}

struct Move {
    dir: Dir,
    count: usize,
}

impl From<&str> for Move {
    fn from(value: &str) -> Self {
        Move {
            dir: value.chars().nth(0).unwrap().into(),
            count: value.chars().skip(2).collect::<String>().parse().unwrap(),
        }
    }
}

pub fn track_tail(moves: &str) -> usize {
    let mut rope = Rope::new(10);
    // let mut rope = Rope::new(2);
    let mut tail_track: HashSet<Pos> = HashSet::new();

    for line in moves.lines() {
        let r#move: Move = line.into();
        for _ in 0..r#move.count {
            rope.r#move(&r#move.dir);
            tail_track.insert(rope.tail().clone());
            // eprintln!("{rope:?} {}", tail_track.len());
        }
    }

    tail_track.len()
}

fn main() {
    let moves = include_str!("../data");
    let spots = track_tail(moves);
    println!("{spots}");
}

#[cfg(test)]
mod test {
    use crate::track_tail;

    const TEST_DATA: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    pub fn rope_tail() {
        let spots = track_tail(TEST_DATA);
        // assert_eq!(spots, 13);
        assert_eq!(spots, 1);
    }

    #[test]
    pub fn long_rope_tail() {
        let spots = track_tail(
            "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20",
        );
        assert_eq!(spots, 36);
    }
}
