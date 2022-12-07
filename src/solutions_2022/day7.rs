use std::collections::HashMap;

use crate::SolutionType;

type Id = usize;

pub fn day7(input: &str) -> SolutionType {
    let mut state = {
        let mut entries = HashMap::new();
        entries.insert(
            0,
            Entry::Dir(Dir {
                name: "/".into(),
                size: 0,
                parent: usize::MAX,
                children: Vec::new(),
            }),
        );
        State {
            id: 1,
            workdir: 0,
            entries,
        }
    };

    for line in input.trim().lines() {
        let line = line.trim();

        if line.starts_with("$ cd ") {
            state.change_dir(&line[5..]);
        } else if !line.starts_with('$') {
            let size = line.split(' ').next().unwrap();
            let name = line.split(' ').nth(1).unwrap();

            if size == "dir" && !state.exists_dir(name) {
                state.new_dir(name.to_string());
            }

            if size != "dir" {
                state.new_file(name.to_string(), size.parse().unwrap());
            }
        }
    }

    let mut tot_size = 0;
    for entry in state.entries.into_values() {
        if let Entry::Dir(dir) = entry {
            if dir.size < 100_000 {
                tot_size += dir.size;
            }
        }
    }

    SolutionType::Int(tot_size as i64)
}

pub fn day7_part2(input: &str) -> SolutionType {
    let mut state = {
        let mut entries = HashMap::new();
        entries.insert(
            0,
            Entry::Dir(Dir {
                name: "/".into(),
                size: 0,
                parent: usize::MAX,
                children: Vec::new(),
            }),
        );
        State {
            id: 1,
            workdir: 0,
            entries,
        }
    };

    for line in input.trim().lines() {
        let line = line.trim();

        if line.starts_with("$ cd ") {
            state.change_dir(&line[5..]);
        } else if !line.starts_with('$') {
            let size = line.split(' ').next().unwrap();
            let name = line.split(' ').nth(1).unwrap();

            if size == "dir" && !state.exists_dir(name) {
                state.new_dir(name.to_string());
            }

            if size != "dir" {
                state.new_file(name.to_string(), size.parse().unwrap());
            }
        }
    }

    let used_space = *state.entries[&0].size();
    let needed_space = 30_000_000 - (70_000_000 - used_space);
    let mut min = usize::MAX;

    for entry in state.entries.into_values() {
        if let Entry::Dir(dir) = entry {
            if dir.size >= needed_space && dir.size < min {
                min = dir.size;
            }
        }
    }

    SolutionType::Int(min as i64)
}

#[derive(Debug, Clone)]
struct Dir {
    name: String,
    size: usize,
    parent: Id,
    children: Vec<Id>,
}

#[derive(Debug, Clone)]
struct File {
    name: String,
    parent: Id,
    size: usize,
}

#[derive(Debug, Clone)]
enum Entry {
    Dir(Dir),
    File(File),
}

impl Entry {
    fn parent(&self) -> Id {
        match self {
            Self::Dir(dir) => dir.parent,
            Self::File(file) => file.parent,
        }
    }

    fn size(&self) -> &usize {
        match self {
            Self::Dir(dir) => &dir.size,
            Self::File(file) => &file.size,
        }
    }

    fn size_mut(&mut self) -> &mut usize {
        match self {
            Self::Dir(dir) => &mut dir.size,
            Self::File(file) => &mut file.size,
        }
    }

    fn name(&self) -> &str {
        match self {
            Self::File(file) => &file.name,
            Self::Dir(dir) => &dir.name,
        }
    }

    fn as_dir(&self) -> &Dir {
        if let Self::Dir(dir) = self {
            dir
        } else {
            panic!("Not a directory");
        }
    }

    fn as_dir_mut(&mut self) -> &mut Dir {
        if let Self::Dir(dir) = self {
            dir
        } else {
            panic!("Not a directory");
        }
    }
}

struct State {
    id: usize,
    workdir: Id,
    entries: HashMap<Id, Entry>,
}
impl State {
    fn new_child(&mut self, entry: Entry) {
        self.entries
            .get_mut(&self.workdir)
            .unwrap()
            .as_dir_mut()
            .children
            .push(self.id);
        self.entries.insert(self.id, entry);
        self.id += 1;
    }

    fn child_named(&self, name: &str) -> Option<Id> {
        for child in &self.entries[&self.workdir].as_dir().children {
            if self.entries[child].name() == name {
                return Some(*child);
            }
        }

        None
    }

    pub fn new_dir(&mut self, name: String) {
        self.new_child(Entry::Dir(Dir {
            name,
            size: 0,
            parent: self.workdir,
            children: vec![],
        }));
    }

    pub fn new_file(&mut self, name: String, size: usize) {
        self.new_child(Entry::File(File {
            name,
            size,
            parent: self.workdir,
        }));

        let mut parent = self.workdir;
        while parent != usize::MAX {
            *self.entries.get_mut(&parent).unwrap().size_mut() += size;
            parent = self.entries[&parent].parent();
        }
    }

    pub fn exists_dir(&self, name: &str) -> bool {
        self.child_named(name).is_some()
    }

    pub fn change_dir(&mut self, path: &str) {
        self.workdir = match path {
            "/" => 0,
            ".." => self.entries[&self.workdir].parent(),
            _ => self.child_named(path).unwrap(),
        }
    }
}

#[test]
#[cfg(test)]
fn test() {
    let input = "$ cd /
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

    assert_eq!(day7(input), SolutionType::Int(95437));
    assert_eq!(day7_part2(input), SolutionType::Int(24933642));
}
