use std::collections::{BTreeMap, VecDeque};

pub enum Entry {
    Dir(Dir),
    File(File),
}

impl Entry {
    fn size(&self) -> usize {
        match self {
            Entry::Dir(d) => d.size(),
            Entry::File(f) => f.size(),
        }
    }

    fn is_dir(&self) -> bool {
        matches!(self, Entry::Dir(_))
    }

    // fn iter(&self) -> _ {
    //     match self {
    //         Entry::Dir(d) => d.entries.values().map(|e| e),
    //         Entry::File(_) => std::iter::once::<&'a Entry>(self),
    //     }
    // }
}

pub struct File {
    size: usize,
}

impl File {
    pub fn new(size: usize) -> Self {
        File { size }
    }

    fn size(&self) -> usize {
        self.size
    }
}

#[derive(Default)]
pub struct Dir {
    entries: BTreeMap<String, Entry>,
}

impl Dir {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_entry(&mut self, name: String, entry: Entry) -> () {
        self.entries.entry(name.clone()).or_insert(entry);
    }

    pub fn find_mut(&mut self, name: &String) -> Option<&mut Entry> {
        self.entries.get_mut(name)
    }

    pub fn size(&self) -> usize {
        self.entries.iter().fold(0, |a, (_, e)| a + e.size())
    }
}

pub struct FS {
    root: Entry,
    cd: Vec<String>,
}

impl FS {
    pub fn new() -> Self {
        FS {
            root: Entry::Dir(Default::default()),
            cd: Default::default(),
        }
    }

    pub fn cd(&mut self, path: String) {
        if path == ".." {
            self.cd.pop();
        } else if path == "/" {
            // Nothing
        } else {
            self.cd.push(path.clone());
        }
    }

    pub fn add_entry(&mut self, name: String, entry: Entry) {
        let mut dir = &mut self.root;
        for p in self.cd.iter() {
            match dir {
                Entry::Dir(d) => dir = d.find_mut(&p).unwrap(),
                Entry::File(_) => todo!(),
            }
        }
        match dir {
            Entry::Dir(dir) => dir.add_entry(name, entry),
            Entry::File(_) => todo!(),
        }
    }

    pub fn parse(history: &str) -> Self {
        let mut fs = FS::new();

        history.lines().for_each(|line| {
            // eprintln!("{line}");
            if line.starts_with("$") {
                let parts: Vec<&str> = line.split(" ").collect();
                if parts[1] == "cd" {
                    fs.cd(parts[2].to_string());
                }
            } else {
                let parts: Vec<&str> = line.split(" ").collect();
                let size = parts[0];
                let name = parts[1].to_string();
                let entry = if size == "dir" {
                    Entry::Dir(Default::default())
                } else {
                    Entry::File(File::new(size.parse::<usize>().unwrap()))
                };
                fs.add_entry(name, entry);
            }
        });

        fs
    }

    pub fn sum_smallest(&self) -> usize {
        let mut sum = 0;

        let mut stack: VecDeque<&Entry> = VecDeque::new();
        stack.push_back(&self.root);

        while !stack.is_empty() {
            let next = stack.pop_front();
            match next {
                Some(Entry::Dir(d)) => {
                    d.entries.values().for_each(|e| stack.push_back(e));
                    let q = d.size();
                    if q < 100_000 {
                        sum += q
                    }
                }
                _ => {}
            }
        }

        sum
    }

    pub fn find_ideal(&self) -> usize {
        let unused = 70000000 - self.root.size();
        let missing = 30000000 - unused;
        let mut best = usize::MAX;

        eprintln!("Looking for {missing} bytes");

        let mut stack: VecDeque<&Entry> = VecDeque::new();
        stack.push_back(&self.root);

        while !stack.is_empty() {
            let next = stack.pop_front();
            match next {
                Some(Entry::Dir(d)) => {
                    d.entries.values().for_each(|e| stack.push_back(e));
                    let q = d.size();
                    if q >= missing && q < best {
                        eprintln!("Improving {q}");
                        best = q;
                    }
                }
                _ => {}
            }
        }

        best
    }

    pub fn iter(&self) -> EntryIter<'_> {
        todo!()
        // EntryIter {
        //     entries: Box::new(std::iter::once(&self.root)),
        //     parent: None,
        // }
    }
}

// pub struct EntryIter<'a>(&'a Entry);
pub struct EntryIter<'a> {
    entries: Box<dyn Iterator<Item = &'a Entry>>,
    parent: Option<Box<EntryIter<'a>>>,
}

impl<'a> Iterator for EntryIter<'a> {
    type Item = &'a Entry;

    fn next(&mut self) -> Option<Self::Item> {
        match self.entries.next() {
            None => match self.parent.take() {
                None => None,
                Some(parent) => {
                    *self = *parent;
                    self.next()
                }
            },
            Some(entry) => {
                // if let Entry::Dir(dir) = entry {
                //     *self = EntryIter {
                //         entries: Box::new(std::iter::empty()),
                //         // entries: dir.entries.values(),
                //         parent: Some(Box::new(std::mem::take(self))),
                //     }
                // }
                Some(entry)
            }
        }
    }
}

impl Default for EntryIter<'_> {
    fn default() -> Self {
        todo!()
        // EntryIter {
        //     entries: Box::new(std::iter::empty()),
        //     parent: None,
        // }
    }
}

pub fn dir_size(fs: &FS, size: usize) -> usize {
    fs.iter()
        .filter(|e| e.is_dir() && e.size() <= size)
        .fold(0, |a, e| a + e.size())
}

fn main() {
    let fs = FS::parse(include_str!("../data"));
    // let size = dir_size(&fs, 100_000);
    let size = fs.find_ideal();
    eprintln!("{size}");
}

#[cfg(test)]
mod test {
    use crate::FS;

    const TEST_DATA: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn parse_input() {
        FS::parse(TEST_DATA);
    }

    #[test]
    fn sum_smallest() {
        let fs = FS::parse(TEST_DATA);
        let sum = fs.sum_smallest();
        assert_eq!(sum, 95437);
    }

    #[test]
    fn ideal() {
        let fs = FS::parse(TEST_DATA);
        let ideal = fs.find_ideal();
        assert_eq!(ideal, 24933642);
    }
}
