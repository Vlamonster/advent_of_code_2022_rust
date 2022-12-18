use std::cell::RefCell;
use std::rc::Rc;

struct File {
    name: String,
    size: usize,
}

struct Directory {
    name: String,
    parent: Option<Rc<RefCell<Directory>>>,
    directories: Vec<Rc<RefCell<Directory>>>,
    files: Vec<File>,
}

impl File {
    pub fn new(name: String, size: usize) -> Self {
        File { name, size }
    }
}

impl Directory {
    pub fn new(name: String) -> Self {
        Directory {
            name,
            parent: None,
            directories: vec![],
            files: vec![],
        }
    }

    pub fn size(&self) -> usize {
        self.files.iter().map(|file| file.size).sum::<usize>()
            + self
            .directories
            .iter()
            .map(|directory| directory.borrow().size())
            .sum::<usize>()
    }
}

fn main() {
    let root = Rc::new(RefCell::new(Directory::new("/".into())));
    let mut file_system = Vec::new();
    let mut current_directory = Rc::clone(&root);
    file_system.push(Rc::clone(&root));

    'outer: for line in include_str!("input.txt").lines() {
        match line {
            "$ cd .." => {
                let current_clone = Rc::clone(&current_directory);
                current_directory = Rc::clone(current_clone.borrow().parent.as_ref().unwrap());
            }
            "$ cd /" => {
                current_directory = Rc::clone(&root);
            }
            "$ ls" => {
                // do nothing
            }
            _ if &line[0..4] == "$ cd" => {
                let current_clone = Rc::clone(&current_directory);
                for directory in &current_clone.borrow().directories {
                    if directory.borrow().name == line[5..] {
                        directory.borrow_mut().parent = Some(Rc::clone(&current_directory));
                        current_directory = Rc::clone(directory);
                    }
                }
            }
            _ if &line[0..3] == "dir" => {
                for directory in &current_directory.borrow().directories {
                    if directory.borrow().name == line[4..] {
                        continue 'outer;
                    }
                }
                let new_directory = Rc::new(RefCell::new(Directory::new((&line[4..]).into())));
                current_directory
                    .borrow_mut()
                    .directories
                    .push(Rc::clone(&new_directory));
                file_system.push(Rc::clone(&new_directory));
            }
            _ => {
                let (file_size, file_name) = line.split_once(' ').unwrap();
                let file_size = file_size.parse::<usize>().unwrap();
                let current_clone = Rc::clone(&current_directory);
                for file in &current_clone.borrow().files {
                    if file.name == file_name {
                        continue 'outer;
                    }
                }
                current_directory
                    .borrow_mut()
                    .files
                    .push(File::new(file_name.into(), file_size));
            }
        }
    }

    let needed_space = root.borrow().size() - 40000000;

    print!(
        "{}",
        file_system
            .iter()
            .map(|directory| directory.borrow().size())
            .filter(|directory_size| *directory_size >= needed_space)
            .min()
            .unwrap()
    );
}
