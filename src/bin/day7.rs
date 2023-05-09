use std::collections::HashMap;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};

const FILE_SYSTEM_CAPACITY: usize = 70000000;
const UPDATE_SIZE: usize = 30000000;

struct FileSystem {
    files: RefCell<HashMap<String, RefCell<usize>>>,
    current_dir: RefCell<Vec<String>>,
}

impl FileSystem {
    fn new() -> Self {
        FileSystem {
            files: RefCell::new(HashMap::new()),
            current_dir: RefCell::new(Vec::new())
        }
    }

    fn from_outputs(outputs: &str) -> Self {
        let filesystem = FileSystem::new();
        filesystem.add_data("dir ");
        for output in outputs.lines() {
            if output.contains("$ cd") {
                filesystem.change_directory(&output[2..].split_once(' ').unwrap().1);
            } else if !output.contains("$ ls") {
                filesystem.add_data(output);
            }
        }
        filesystem
    }

    fn add_data(&self, s: &str) {
        let (left, name) =  s.split_once(' ').expect("Invalid data input given");
        if left == "dir" {
            if self.path().chars().last().unwrap() == '/' {
                self.files.borrow_mut().insert(self.path().to_owned() + name, RefCell::new(0));
            } else {
                self.files.borrow_mut().insert(self.path().to_owned() + "/" + name, RefCell::new(0));
            }
        } else if let Ok(file_size) = left.parse::<usize>() {
            for path in self.current_dir.borrow().iter() {
                *self.files.borrow().get(&path.to_owned()).expect("directory should exist").borrow_mut().deref_mut() += file_size;
            }
        } else {
            panic!("Invalid data input given");
        }
    }

    fn path(&self) -> String {
        self.current_dir.borrow().last().unwrap_or(&"/".to_string()).to_owned()
    }

    fn change_directory(&self, dir: &str) {
        match dir {
            "/" => {
                self.current_dir.borrow_mut().clear();
                self.current_dir.borrow_mut().push(String::from("/"));
            }
            ".." => { self.current_dir.borrow_mut().pop(); }
            _ => {
                let mut path = self.current_dir.borrow().last().expect("path should exist").to_owned();
                if path.chars().last().unwrap() != '/' {
                    path.push('/');
                }
                let new_path = path + dir;
                self.current_dir.borrow_mut().push(new_path)
            }
        }
    }

    fn get_size(&self, path: &str) -> usize {
        *self.files.borrow().get(path).expect("should be valid path").borrow().deref()
    }
}

#[test]
fn day_7_part_1() {
    let test_outputs = std::fs::read_to_string("input/day7test").unwrap();

    let expected_values = [
        ("/", 48381165),
        ("/d", 24933642),
        ("/a", 94853),
        ("/a/e", 584)
    ];

    let filesystem = FileSystem::from_outputs(test_outputs.as_str());

    let mut total = 0;
    for (path, size) in expected_values {
        println!("Directory {} is expected to have size {}", path, size);
        let calc_size = filesystem.get_size(path);
        assert_eq!(size, calc_size);
        if calc_size < 100000 { total += calc_size }
    }
    assert_eq!(95437, total);
}

#[test]
fn day_7_part_2() {
    let test_outputs = std::fs::read_to_string("input/day7test").unwrap();
    let expected_deletion = String::from("/d");
    let expected_size = 24933642;
    let filesystem = FileSystem::from_outputs(test_outputs.as_str());
    let unused = FILE_SYSTEM_CAPACITY - filesystem.get_size("/");
    let required_space = UPDATE_SIZE - unused;
    let files =  filesystem.files.borrow();
    let to_delete = files.keys()
        .filter(|path| {
            filesystem.get_size(path) > required_space
        }).min_by(|a, b| {
            filesystem.get_size(a).cmp(&filesystem.get_size(b))
        }).expect("Did not find a folder that fulfills the requirements");
    assert_eq!(&expected_deletion, to_delete);
    assert_eq!(expected_size, filesystem.get_size(to_delete));
}

#[test]
fn day_7_validate_size() {
    let outputs = std::fs::read_to_string("input/day7").unwrap();
    let mut expected_sum = 0;
    for line in outputs.lines() {
        if let Ok(n) = line.split_whitespace().next().unwrap().parse::<usize>() {
            expected_sum += n;
        }
    }
    let filesystem = FileSystem::from_outputs(&outputs);
    assert_eq!(expected_sum, filesystem.get_size("/"));
}

fn main() {
    let filesystem = FileSystem::from_outputs(std::fs::read_to_string("input/day7").unwrap().as_str());
    println!("The total size of all directories smaller than 100000 is {}", {
        filesystem.files.borrow()
            .values()
            .map(|v| v.borrow().to_owned())
            .filter(|&x| x < 100000)
            .sum::<usize>()
    });

    println!("The folder that should be deleted to make room for the update has a capacity of {}", {
        let used = filesystem.get_size("/");
        let unused = FILE_SYSTEM_CAPACITY - used;
        let required_space = UPDATE_SIZE - unused;
        let files = filesystem.files.borrow();

        files.keys()
            .map(|path| filesystem.get_size(path))
            .filter(|&size| size > required_space)
            .min()
            .expect("there should be a folder that meets the requirements")
    })
}