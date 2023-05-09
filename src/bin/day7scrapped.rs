// Initial attempt at day 7
// I made the naive mistake of attempting to simulate a filesystem,
// with folders containing no intrinsic size (as described by the problem)
// and files with size being contained in these folder
// This proved to be a fruitless attempt as there was an issue
// with the FileSystem::get_size method that worked fine for the
// test input but not for the actual input that I was unable to diagnose.
// (Someone who isn't dumb like me can probable figure it out)
// The code I ended up with for day 7 reuses a lot of this but
// simplifies the problem down to purely counting the amount of data in each directory.

use std::collections::HashMap;
use std::cell::RefCell;
use std::ops::DerefMut;

const FILE_SYSTEM_CAPACITY: usize = 70000000;
const UPDATE_SIZE: usize = 30000000;

#[derive(Clone, Copy)]
enum FileType {
    File(usize),
    Directory
}

struct FileSystem {
    files: RefCell<HashMap<String, FileType>>,
    current_dir: RefCell<Vec<String>>,
    internal_counter: RefCell<usize> // Like god please help me
}

// File exists to archive a failed attempt, but I don't want its warnings flooding everywhere
// Tests are ignored by default for the same reason
#[allow(dead_code)]
impl FileSystem {
    fn new() -> Self {
        FileSystem {
            files: RefCell::new(HashMap::new()),
            current_dir: RefCell::new(Vec::new()),
            internal_counter: RefCell::new(0)
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
        println!("data is {}", s);
        if left == "dir" {
            self.files.borrow_mut().insert(self.path().to_owned() + name, FileType::Directory);
        } else if let Ok(file_size) = left.parse::<usize>() {
            self.files.borrow_mut().insert(self.path().to_owned() + name, FileType::File(file_size));
            *self.internal_counter.borrow_mut().deref_mut() += file_size;
        } else {
            panic!("Invalid data input given");
        }
    }

    // Enumerates current_dir into full path string
    fn path(&self) -> String {
        let mut path = String::from("/");
        for name in self.current_dir.borrow().iter() {
            path.push_str(name.as_str());
            path.push('/');
        }
        path
    }

    fn change_directory(&self, dir: &str) {
        match dir {
            path if path.contains("/") => {
                self.current_dir.borrow_mut().clear();
                for name in path.split('/').filter(|&c| c != "/" && !c.is_empty()) {
                    self.current_dir.borrow_mut().push(name.to_string());
                }
            }
            ".." => { self.current_dir.borrow_mut().pop(); }
            _ => self.current_dir.borrow_mut().push(dir.to_string())
        }
    }

    fn list_directory(&self) -> String {
        let path_string = self.path();
        let current_path = path_string.as_str();
        let current_depth = FileSystem::path_to_depth(&current_path);
        let mut result = String::new();
        self.files.borrow().iter().filter(|(file_path, _)| {
            file_path.contains(current_path) && FileSystem::path_to_depth(&file_path) == current_depth
        }).for_each(|(path, file_type)| {
            let to_concat = path.as_str().to_owned();
            let s = match file_type {
                FileType::Directory => String::from("\t(dir)\n"),
                FileType::File(size) => {
                    format!("\t(file, size={})\n", size)
                }
            };
            result.push_str(&to_concat);
            result.push_str(&s);
        });
        result
    }

    fn get_child_paths(&self, path: Option<&str>) -> Vec<String> {
        let default = self.path();
        let default_str = default.as_str();
        let current_path = path.unwrap_or(default_str);
        let current_depth = FileSystem::path_to_depth(&current_path);
        self.files.borrow().keys()
        .filter(|file_path| file_path.as_str() != current_path)
        .filter(|file_path| {
            file_path.contains(current_path) && FileSystem::path_to_depth(&file_path) == current_depth + 1
        }).map(|s| s.to_owned()).collect::<Vec<String>>()
    }

    fn get_size(&self, path: Option<&str>) -> usize {
        let default = self.path();
        let default_str = default.as_str();
        let path = path.unwrap_or(default_str);
        match self.files.borrow().get(&path.to_string()) {
            Some(FileType::Directory) => {
                let mut size = 0;
                let child_paths = self.get_child_paths(Some(path));
                for child in child_paths.iter() {
                    // if child != compare { size += self.get_size(Some(&child)); }
                    size += self.get_size(Some(&child));
                }
                size
            }
            Some(FileType::File(size)) => *size,
            None => panic!("Path does not exist in file system")
        }
    }

    fn path_is_directory(&self, path: &str) -> bool {
        match self.files.borrow().get(path) {
            Some(FileType::Directory) => true,
            _ => false
        }
    }

    fn print_directory(&self) {
        for line in self.list_directory().lines() {
            println!("{}", line)
        }
    }

    fn print_child_paths(&self) {
        for path in self.get_child_paths(None) {
            println!("{}", path);
        }
    }

    fn path_to_depth(path: &str) -> usize {
        if path == "/" { return 1; }
        path.chars().into_iter().filter(|&c| c == '/').count() + 1
    }
}

#[test]
#[ignore]
fn day_7_part_1_scrapped() {
    let test_outputs =
"$ cd /
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
    let expected_values = [
        ("/", 48381165),
        ("/d", 24933642),
        ("/a", 94853),
        ("/a/e", 584)
    ];

    let filesystem = FileSystem::from_outputs(test_outputs);

    // println!("Size of folder /a is {}", filesystem.get_size(Some("/a".to_string())));
    // println!("Size of folder /a/e is {}", filesystem.get_size(Some("/a/e".to_string())));

    let mut total = 0;
    for (path, size) in expected_values {
        println!("Directory {} is expected to have size {}", path, size);
        let calc_size = filesystem.get_size(Some(path));
        assert_eq!(size, calc_size);
        if calc_size < 100000 { total += calc_size }
    }
    assert_eq!(95437, total);
}

#[test]
#[ignore]
fn day_7_part_2_scrapped() {
    let test_outputs =
"$ cd /
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
    let expected_deletion = String::from("/d");
    let expected_size = 24933642;
    let filesystem = FileSystem::from_outputs(test_outputs);
    let unused = FILE_SYSTEM_CAPACITY - filesystem.get_size(Some("/"));
    let required_space = UPDATE_SIZE - unused;
    let files =  filesystem.files.borrow();
    let to_delete = files.keys()
        .filter(|path| filesystem.path_is_directory(path))
        .filter(|path| {
            filesystem.get_size(Some(path)) > required_space
        }).min_by(|a, b| {
            filesystem.get_size(Some(a)).cmp(&filesystem.get_size(Some(b)))
        }).expect("Did not find a folder that fulfills the requirements");
    assert_eq!(&expected_deletion, to_delete);
    assert_eq!(expected_size, filesystem.get_size(Some(to_delete)));
}

#[test]
#[ignore]
fn day_7_validate_size_scrapped() {
    let outputs = std::fs::read_to_string("input/day7").unwrap();
    let mut expected_sum = 0;
    for line in outputs.lines() {
        if let Ok(n) = line.split_whitespace().next().unwrap().parse::<usize>() {
            expected_sum += n;
        }
    }
    let filesystem = FileSystem::from_outputs(&outputs);
    assert_eq!(expected_sum, filesystem.get_size(Some("/")), "Internal counter counted {}", filesystem.internal_counter.borrow());
}

fn main() {
    let filesystem = FileSystem::from_outputs(std::fs::read_to_string("input/day7").unwrap().as_str());
    println!("The total size of all directories smaller than 100000 is {}", {
        let mut total = 0;
        for directory in filesystem.files.borrow().keys().filter(|path| filesystem.path_is_directory(path)) {
            let calc = filesystem.get_size(Some(directory));
            if calc < 100000 { total += calc; }
        }
        total
    });

    println!("The folder that should be deleted to make room for the update has a capacity of {}", {
        let used = filesystem.get_size(Some("/"));
        let unused = FILE_SYSTEM_CAPACITY - used;
        let required_space = UPDATE_SIZE - unused;
        let files = filesystem.files.borrow();

        // filesystem.get_size(Some(to_delete))
        let to_delete = files.keys()
            .filter(|path| filesystem.path_is_directory(path))
            .map(|path| filesystem.get_size(Some(path)))
            .filter(|&size| size > required_space)
            .min()
            .expect("Did not find a folder that fulfills the requirements");
        to_delete
    })
}