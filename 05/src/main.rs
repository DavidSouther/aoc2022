#[derive(Debug)]
struct Ship<'a> {
    stacks: &'a [&'a [char]],
}

impl Ship {
    pub fn r#move(&mut self, count: usize, from: usize, to: usize) {
        let to = self.stacks[to];
        let from = self.stacks[from];
        let from_stack = from.take((from.len() - count..));
        self.stacks[to].append(.collect());
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {}
