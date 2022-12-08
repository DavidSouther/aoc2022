pub struct Forest(Vec<Vec<u8>>);

impl Forest {
    pub fn visible(&self, row: usize, col: usize) -> bool {
        let height = self.0[row][col];
        let left = self.0[row][0..col]
            .iter()
            .filter(|i| **i >= height)
            .next()
            .is_none();
        let right = self.0[row][col + 1..]
            .iter()
            .filter(|i| **i >= height)
            .next()
            .is_none();

        let top = self.0[0..row]
            .iter()
            .filter(|v| v[col] >= height)
            .next()
            .is_none();
        let bottom = self.0[row + 1..]
            .iter()
            .filter(|v| v[col] >= height)
            .next()
            .is_none();

        left || right || top || bottom
    }

    pub fn scenic_up(&self, row: usize, col: usize) -> usize {
        let tallest = self.0[row][col];
        let mut count = 0;
        for r in (0..row).rev() {
            count += 1;
            let height = self.0[r][col];
            if height >= tallest {
                return count;
            }
        }
        count
    }

    pub fn scenic_down(&self, row: usize, col: usize) -> usize {
        let tallest = self.0[row][col];
        let mut count = 0;
        for r in row + 1..self.0.len() {
            count += 1;
            let height = self.0[r][col];
            if height >= tallest {
                return count;
            }
        }
        count
    }

    pub fn scenic_right(&self, row: usize, col: usize) -> usize {
        let tallest = self.0[row][col];
        let mut count = 0;
        for c in col + 1..self.0.len() {
            count += 1;
            let height = self.0[row][c];
            if height >= tallest {
                return count;
            }
        }
        count
    }

    pub fn scenic_left(&self, row: usize, col: usize) -> usize {
        let tallest = self.0[row][col];
        let mut count = 0;
        for c in (0..col).rev() {
            count += 1;
            let height = self.0[row][c];
            if height >= tallest {
                return count;
            }
        }
        count
    }

    pub fn scenic(&self, row: usize, col: usize) -> usize {
        self.scenic_up(row, col)
            * self.scenic_down(row, col)
            * self.scenic_left(row, col)
            * self.scenic_right(row, col)
    }

    pub fn most_scenic(&self) -> usize {
        let mut c = 0;
        for row in 0..self.0.len() {
            for col in 0..self.0.len() {
                let scenic = self.scenic(row, col);
                if scenic > c {
                    eprintln!("More scenic at {row}, {col} ({scenic})");
                    c = scenic
                }
            }
        }
        c
    }

    pub fn number_visible(&self) -> usize {
        let mut c = 0;
        for row in 0..self.0.len() {
            for col in 0..self.0.len() {
                if self.visible(row, col) {
                    c += 1;
                }
            }
        }
        c
    }
}

impl Forest {
    pub fn parse(grid: &str) -> Self {
        let trees = grid
            .lines()
            .map(|l| {
                l.trim()
                    .split("")
                    .filter(|t| !t.is_empty())
                    .map(|t| t.parse().unwrap())
                    .collect()
            })
            .collect();

        Forest(trees)
    }
}

fn main() {
    let forest = Forest::parse(include_str!("../data"));
    // let visible = forest.number_visible();
    let scenic = forest.most_scenic();
    println!("{scenic}");
}

#[cfg(test)]
mod test {
    use crate::Forest;

    const TEST_DATA: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn part_1() {
        let forest = Forest::parse(TEST_DATA);
        let visible = forest.number_visible();
        assert_eq!(visible, 21);
    }

    #[test]
    fn part_2() {
        let forest = Forest::parse(TEST_DATA);

        let up = forest.scenic_up(1, 2);
        assert_eq!(up, 1);
        let down = forest.scenic_down(1, 2);
        assert_eq!(down, 2);

        let left = forest.scenic_left(1, 2);
        assert_eq!(left, 1);
        let right = forest.scenic_right(1, 2);
        assert_eq!(right, 2);

        let scenic = forest.scenic(1, 2);
        assert_eq!(scenic, 4);

        let up = forest.scenic_up(3, 2);
        assert_eq!(up, 2);
        let down = forest.scenic_down(3, 2);
        assert_eq!(down, 1);

        let left = forest.scenic_left(3, 2);
        assert_eq!(left, 2);
        let right = forest.scenic_right(3, 2);
        assert_eq!(right, 2);

        let scenic = forest.scenic(3, 2);
        assert_eq!(scenic, 8);

        let visible = forest.most_scenic();
        assert_eq!(visible, 8);
    }
}
