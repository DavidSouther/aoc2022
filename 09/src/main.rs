use std::{collections::HashSet, fmt::Display};

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Pos {
    row: i32,
    col: i32,
}

#[derive(Default)]
struct Bounds(Pos, Pos);

impl Bounds {
    fn extend(&mut self, point: &Pos) {
        self.0.row = self.0.row.min(point.row);
        self.0.col = self.0.col.min(point.col);
        self.1.row = self.1.row.max(point.row);
        self.1.col = self.1.col.max(point.col);
    }

    fn square(&self) -> i32 {
        (self.1.row - self.0.row).max(self.1.col - self.0.col)
    }
}

#[derive(Debug, Default)]
pub struct Rope {
    knots: Vec<Pos>,
    path: HashSet<Pos>,
}

// fn snap(head: &Pos, tail: &mut Pos) {
fn snap(vec: &mut Vec<Pos>, tail: usize) {
    let head = vec[tail - 1].clone();
    let tail = vec.get_mut(tail).unwrap();

    let drow = tail.row - head.row;
    let dcol = tail.col - head.col;

    if dcol == 0 {
        if drow >= 2 {
            tail.row = head.row + 1;
        }
        if drow <= 2 {
            tail.row = head.row - 1;
        }
    }

    if drow == 0 {
        if drow >= 2 {
            tail.col = head.col + 1;
        }
        if drow <= 2 {
            tail.col = head.col - 1;
        }
    }

    if dcol.abs() >= 2 && drow.abs() >= 1 {
        tail.row = head.row;
        tail.col = head.col - dcol.signum();
    }

    if drow.abs() >= 2 && dcol.abs() >= 1 {
        tail.col = head.col;
        tail.row = head.row - drow.signum();
    }
}

impl Rope {
    pub fn new(len: usize) -> Self {
        let mut vec = Vec::with_capacity(len);
        for _ in 0..len {
            vec.push(Default::default())
        }
        Rope {
            knots: vec,
            path: HashSet::new(),
        }
    }

    pub fn tail(&self) -> &Pos {
        &self.knots.last().unwrap()
    }

    pub fn right(&mut self) {
        self.knots[0].row += 1;
    }

    pub fn left(&mut self) {
        self.knots[0].row -= 1;
    }

    pub fn up(&mut self) {
        self.knots[0].col += 1;
    }

    pub fn down(&mut self) {
        self.knots[0].col -= 1;
    }

    pub fn r#move(&mut self, dir: &Dir) {
        match dir {
            Dir::Left => self.left(),
            Dir::Right => self.right(),
            Dir::Up => self.up(),
            Dir::Down => self.down(),
        }

        for i in 1..self.knots.len() {
            snap(&mut self.knots, i);
        }
    }

    pub fn wiggle(&mut self, moves: &str) {
        for line in moves.lines() {
            let r#move: Move = line.into();
            for _ in 0..r#move.count {
                self.r#move(&r#move.dir);
                self.path.insert(self.tail().clone());
                // eprintln!("{rope:?} {}", tail_track.len());
            }
        }
    }
}

impl Display for Rope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut bounds = Bounds::default();
        self.knots.iter().for_each(|p| bounds.extend(p));
        self.path.iter().for_each(|p| bounds.extend(p));

        let mut board = Vec::<Vec<char>>::new();
        let size = bounds.square();
        for _ in 0..=size {
            let mut line = Vec::<char>::new();
            for _ in 0..=size {
                line.push('.');
            }
            board.push(line);
        }

        for p in self.path.iter() {
            let row = p.row - bounds.0.row;
            let col = p.col - bounds.0.col;
            board[row as usize][col as usize] = '#';
        }

        for i in (0..self.knots.len()).rev() {
            let p = self.knots[i];
            let row = p.row - bounds.0.row;
            let col = p.col - bounds.0.col;
            let c = if i == 0 {
                'H'
            } else {
                char::from_digit(i as u32, 10).unwrap()
            };
            board[row as usize][col as usize] = c;
        }

        let zrow = (0 - bounds.0.row) as usize;
        let zcol = (0 - bounds.0.col) as usize;
        if board[zrow][zcol] == '.' || board[zrow][zcol] == '#' {
            board[zrow][zcol] = 's';
        }

        for r in (0..board.len()).rev() {
            for c in 0..board[r].len() {
                write!(f, "{}", board[c][r])?;
            }
            write!(f, "\n")?;
        }

        write!(f, "\n")
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

pub fn track_tail(size: usize, moves: &str) -> usize {
    let mut rope = Rope::new(size);

    rope.wiggle(moves);

    // eprintln!("{rope}");

    rope.path.len()
}

fn main() {
    let moves = include_str!("../data");
    // let spots = track_tail(2, moves);
    // println!("2: {spots}");
    let spots = track_tail(10, moves);
    println!("10: {spots}");
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
        let spots = track_tail(2, TEST_DATA);
        assert_eq!(spots, 13);
        let spots = track_tail(10, TEST_DATA);
        assert_eq!(spots, 1);
    }

    #[test]
    pub fn long_rope_tail() {
        let spots = track_tail(
            10,
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
