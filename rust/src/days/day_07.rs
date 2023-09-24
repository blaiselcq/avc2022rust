use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

enum PathElement {
    Current,
    Parent,
    Child(String),
}

type Path = Vec<PathElement>;

impl TryFrom<&str> for PathElement {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, ()> {
        match value {
            "." => Ok(Self::Current),
            ".." => Ok(Self::Parent),
            "" => Err(()),
            string => Ok(Self::Child(string.to_string())),
        }
    }
}

fn make_path(value: &str) -> Result<Path, ()> {
    let splitted = value.split('\n').map(PathElement::try_from);
    if splitted.clone().any(|e| e.is_err()) {
        return Err(());
    }
    Ok(splitted.map(|e| e.unwrap()).collect())
}

#[derive(Debug)]
struct File {
    name: String,
    size: u32,
    parent: Weak<RefCell<Dir>>,
}

#[derive(Debug)]
struct Dir {
    name: String,
    children: Vec<DirElement>,
    parent: Option<Weak<RefCell<Dir>>>,
}

impl Dir {
    fn add_child_file(ptr: &Rc<RefCell<Self>>, size: u32, name: &str) {
        let name = name.to_string();
        let file = Rc::new(RefCell::new(File {
            size,
            name,
            parent: Rc::downgrade(ptr),
        }));
        ptr.borrow_mut().children.push(DirElement::File(file));
    }

    fn add_child_dir(ptr: &Rc<RefCell<Self>>, name: &str) {
        let name = name.to_string();
        let dir = Rc::new(RefCell::new(Dir {
            name,
            parent: Some(Rc::downgrade(ptr)),
            children: vec![],
        }));
        ptr.borrow_mut().children.push(DirElement::Dir(dir));
    }

    fn get_total_size(&self) -> u32 {
        self.children.iter().map(|d| d.get_size()).sum()
    }
}

#[derive(Debug)]
enum DirElement {
    File(Rc<RefCell<File>>),
    Dir(Rc<RefCell<Dir>>),
}

impl Clone for DirElement {
    fn clone(&self) -> Self {
        match self {
            DirElement::File(file) => DirElement::File(file.clone()),
            DirElement::Dir(dir) => DirElement::Dir(dir.clone()),
        }
    }
}

impl DirElement {
    fn get_name(&self) -> String {
        match self {
            DirElement::File(file) => file.borrow().name.clone(),
            DirElement::Dir(dir) => dir.borrow().name.clone(),
        }
    }

    fn get_size(&self) -> u32 {
        match self {
            DirElement::File(file) => file.borrow().size,
            DirElement::Dir(dir) => dir.borrow().get_total_size(),
        }
    }

    fn get_child(&self, name: &str) -> Option<DirElement> {
        match self {
            DirElement::File(_) => None,
            DirElement::Dir(dir) => {
                for child in &dir.borrow().children {
                    if child.get_name() == name {
                        return Some(child.clone());
                    }
                }
                None
            }
        }
    }

    fn get_parent(&self) -> Option<Rc<RefCell<Dir>>> {
        match self {
            DirElement::File(file) => file.borrow().parent.upgrade(),
            DirElement::Dir(dir) => {
                if let Some(parent) = &dir.borrow().parent {
                    parent.upgrade()
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Debug)]
struct DirCursor {
    cursor: DirElement,
    root: Rc<RefCell<Dir>>,
    is_ls: bool,
}

impl DirCursor {
    fn create_empty() -> Self {
        let directory = Rc::new(RefCell::new(Dir {
            name: "".to_string(),
            children: vec![],
            parent: None,
        }));
        Self {
            root: directory.clone(),
            cursor: DirElement::Dir(directory),
            is_ls: false,
        }
    }

    fn cd(&mut self, path_str: &str) -> Result<(), ()> {
        if path_str == "/" {
            self.cursor = DirElement::Dir(self.root.clone());
            return Ok(());
        }
        let path = make_path(path_str)?;
        for path_element in path {
            match path_element {
                PathElement::Current => continue,
                PathElement::Parent => {
                    if let Some(parent) = self.cursor.get_parent() {
                        self.cursor = DirElement::Dir(parent);
                    } else {
                        return Err(());
                    }
                }
                PathElement::Child(name) => {
                    if let Some(child) = self.cursor.get_child(&name) {
                        self.cursor = child;
                    } else {
                        return Err(());
                    };
                }
            }
        }
        Ok(())
    }

    fn handle_command_line(&mut self, line: &str) -> Result<(), ()> {
        self.is_ls = false;
        match line.split_whitespace().next() {
            None => Err(()),
            Some("ls") => {
                self.is_ls = true;
                Ok(())
            }
            Some("cd") => self.cd(&line[3..]),
            Some(_) => Err(()),
        }
    }

    fn handle_ls_line(&mut self, line: &str) -> Result<(), ()> {
        if let Some((size_or_dir, name)) = line.split_once(' ') {
            match &self.cursor {
                DirElement::File(_) => {
                    return Err(());
                }
                DirElement::Dir(cursor) => {
                    let mut is_file = false;
                    let size_or_dir = size_or_dir.parse::<u32>().map(|s| {
                        is_file = true;
                        s
                    });
                    if is_file {
                        let size = size_or_dir.unwrap();
                        Dir::add_child_file(cursor, size, name);
                    } else {
                        Dir::add_child_dir(cursor, name);
                    }
                    return Ok(());
                }
            }
        }
        Err(())
    }

    fn handle_line(&mut self, line: &str) -> Result<(), ()> {
        if line.chars().next().is_some_and(|c| c == '$') {
            self.handle_command_line(&line[2..])
        } else {
            if !self.is_ls {
                return Err(());
            }
            self.handle_ls_line(line)
        }
    }
}

fn parse_output(text: &str) -> DirCursor {
    let mut dir = DirCursor::create_empty();
    for line in text
        .split('\n')
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
    {
        dir.handle_line(line).unwrap();
    }

    dir
}

fn get_small_dir_recur(dir: &Rc<RefCell<Dir>>, result: &mut Vec<Rc<RefCell<Dir>>>, limit: u32) {
    if dir.borrow().get_total_size() <= limit {
        result.push(dir.clone());
    }
    for child in &dir.borrow().children {
        match child {
            DirElement::File(_) => {}
            DirElement::Dir(dir) => get_small_dir_recur(dir, result, limit),
        }
    }
}

fn get_all_dir_recur(dir: &Rc<RefCell<Dir>>, result: &mut Vec<Rc<RefCell<Dir>>>) {
    result.push(dir.clone());
    for child in &dir.borrow().children {
        match child {
            DirElement::File(_) => {}
            DirElement::Dir(dir) => get_all_dir_recur(dir, result),
        }
    }
}

pub fn puzzle_1(input: &str) -> String {
    let dir = parse_output(input);
    let mut small_dirs = vec![];
    get_small_dir_recur(&dir.root, &mut small_dirs, 100_000);

    let size: u32 = small_dirs
        .into_iter()
        .map(|d| d.borrow().get_total_size())
        .sum();

    size.to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let target = 30_000_000;
    let fs_total = 70_000_000;

    let dir = parse_output(input);

    let space_left = fs_total - dir.root.borrow().get_total_size();
    if space_left > target {
        return String::new();
    }
    let to_save = target - space_left;

    let mut all_dirs = vec![];
    get_all_dir_recur(&dir.root, &mut all_dirs);

    let mut sizes: Vec<_> = all_dirs
        .into_iter()
        .map(|d| d.borrow().get_total_size())
        .filter(|s| *s >= to_save)
        .collect();
    sizes.sort();
    sizes.first().unwrap().to_string()
}

#[cfg(test)]
mod tests {

    fn get_input() -> String {
        let day = 7;
        let input_file_path = format!("../data/tests/test{:02}.txt", day);
        std::fs::read_to_string(input_file_path).unwrap()
    }

    use super::*;

    #[test]
    fn test_create_empty_dir() {
        let output = "$ cd /";
        let dir = parse_output(output);
        assert_eq!(dir.root.borrow().children.len(), 0);
    }

    #[test]
    fn test_create_dirs() {
        let output = "$ cd /\n$ ls\ndir a\n12345 b\n";
        let dir = parse_output(output);
        assert_eq!(dir.root.borrow().children.len(), 2);
        assert_eq!(dir.root.borrow().children.get(0).unwrap().get_size(), 0);
        assert_eq!(dir.root.borrow().children.get(1).unwrap().get_size(), 12345);
    }

    #[test]
    fn test_puzzle_1() {
        let input = get_input();
        assert_eq!(puzzle_1(&input), "95437");
    }

    #[test]
    fn test_puzzle_2() {
        let input = get_input();
        assert_eq!(puzzle_2(&input), "24933642");
    }
}
