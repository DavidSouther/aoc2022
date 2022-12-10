use std::fmt::Display;

pub struct CRT {
    x: i32,
    cycle: i32,
    recordings: Vec<i32>,
    screen: Vec<Vec<char>>,
}

impl Default for CRT {
    fn default() -> Self {
        let mut screen = Vec::<Vec<char>>::with_capacity(6);

        for _ in 0..6 {
            let mut line = Vec::<char>::with_capacity(40);
            for _ in 0..40 {
                line.push(' ');
            }
            screen.push(line);
        }

        Self {
            x: 1,
            cycle: 0,
            recordings: Vec::new(),
            screen,
        }
    }
}

impl CRT {
    fn draw(&mut self) {
        let i = self.cycle / 40;
        let j = self.cycle % 40;

        if (self.x - 1..=self.x + 1).contains(&j) {
            self.screen[i as usize][j as usize] = '#';
        }
    }

    fn cycle(&mut self) {
        self.draw();
        self.cycle += 1;
        if self.cycle == 20 || (self.cycle - 20) % 40 == 0 {
            let value = self.cycle * self.x;
            eprintln!("Cycle {} x {} pushing {value}", self.cycle, self.x);
            self.recordings.push(value);
        }
    }

    fn noop(&mut self) {
        self.cycle();
    }

    fn addx(&mut self, dx: i32) {
        self.cycle();
        self.cycle();
        self.x += dx;
    }

    fn op(&mut self, op: Operation) {
        match op {
            Operation::Noop => self.noop(),
            Operation::Addx(dx) => self.addx(dx),
        }
    }

    pub fn run(&mut self, ops: &str) -> i32 {
        ops.lines().for_each(|line| self.op(line.into()));
        self.recordings.iter().sum()
    }
}

impl Display for CRT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..6 {
            for j in 0..40 {
                write!(f, "{}", self.screen[i][j])?;
            }
            write!(f, "\n")?;
        }
        write!(f, "\n")
    }
}

enum Operation {
    Noop,
    Addx(i32),
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        if value.starts_with("noop") {
            Operation::Noop
        } else if value.starts_with("addx") {
            Operation::Addx(
                value
                    .split(" ")
                    .skip(1)
                    .next()
                    .unwrap()
                    .to_string()
                    .parse()
                    .unwrap(),
            )
        } else {
            todo!()
        }
    }
}

fn main() {
    let mut crt = CRT::default();
    let data = include_str!("../data");
    let sum = crt.run(data);
    println!("{sum}");
    println!("");
    println!("{crt}");
}

#[cfg(test)]
mod test {
    use crate::CRT;

    #[test]
    fn test_x() {
        let mut crt = CRT::default();
        let data = include_str!("../small_data");
        let sum = crt.run(data);

        assert_eq!(crt.recordings, vec![420, 1140, 1800, 2940, 2880, 3960]);
        assert_eq!(sum, 13140);

        let screen = format!("{crt}");
        println!("{crt}");
        assert_eq!(
            screen,
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );
    }
}
