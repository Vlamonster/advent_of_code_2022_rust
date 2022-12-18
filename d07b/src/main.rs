struct File {
    name: String,
    size: usize,
}

struct Directory {
    name: String,
    parent: usize,
    directories: Vec<usize>,
    files: Vec<File>,
}

impl File {
    pub fn new(name: String, size: usize) -> Self {
        File { name, size }
    }
}

impl Directory {
    pub fn new(name: String, parent: usize) -> Self {
        Directory {
            name,
            parent,
            directories: vec![],
            files: vec![],
        }
    }

    pub fn size(&self, file_system: &Vec<Directory>) -> usize {
        self.files.iter().map(|file| file.size).sum::<usize>()
            + self
                .directories
                .iter()
                .map(|directory_index| file_system[*directory_index].size(file_system))
                .sum::<usize>()
    }
}

fn main() {
    let mut file_system: Vec<Directory> = Vec::new();
    file_system.push(Directory::new("/".into(), 0));
    let mut current_directory = 0;

    for line in include_str!("input.txt").lines() {
        match line {
            "$ cd .." => {
                current_directory = file_system[current_directory].parent;
            }
            "$ cd /" => {
                current_directory = 0;
            }
            "$ ls" => {
                // do nothing
            }
            _ if &line[0..4] == "$ cd" => {
                for directory_index in &file_system[current_directory].directories {
                    if file_system[*directory_index].name == &line[5..] {
                        current_directory = *directory_index;
                    }
                }
            }
            _ if &line[0..3] == "dir" => {
                file_system.push(Directory::new((&line[4..]).into(), current_directory));
                let directory_index = file_system.len() - 1;
                file_system[current_directory]
                    .directories
                    .push(directory_index);
            }
            _ => {
                let (file_size, file_name) = line.split_once(' ').unwrap();
                let file_size = file_size.parse::<usize>().unwrap();
                file_system[current_directory]
                    .files
                    .push(File::new(file_name.into(), file_size));
            }
        }
    }

    let needed_space = file_system[0].size(&file_system) - 40000000;

    print!(
        "{}",
        file_system
            .iter()
            .map(|directory| directory.size(&file_system))
            .filter(|directory_size| *directory_size >= needed_space)
            .min()
            .unwrap()
    );
}
