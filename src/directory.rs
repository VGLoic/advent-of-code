use std::{
    self,
    collections::{HashMap, HashSet},
    fs,
};

#[allow(dead_code)]
pub fn find_sum_of_small_diretories() -> Result<usize, Box<dyn std::error::Error>> {
    let content = fs::read_to_string("input-07.txt")?;

    let file_system = parse_input_to_file_system(content)?;

    let path_to_directory_size = file_system.compute_directories_sizes()?;

    let mut sum = 0;
    for (dir_path, dir_size) in &path_to_directory_size {
        if *dir_size <= 100_000 {
            println!(
                "Found directory of small size, path: {:?}, size: {}",
                dir_path, dir_size
            );
            sum += dir_size;
        }
    }

    Ok(sum)
}

pub fn find_smallest_dir_to_delete_for_update() -> Result<usize, Box<dyn std::error::Error>> {
    let content = fs::read_to_string("input-07.txt")?;

    let file_system = parse_input_to_file_system(content)?;

    let path_to_directory_size = file_system.compute_directories_sizes()?;

    let root_size = path_to_directory_size.get(&file_system.root_path).ok_or("Unable to get the size of the root repository")?;

    const TOTAL_DISK_SPACE: usize = 70_000_000;
    const SPACE_REQUIRED_FOR_UPDATE: usize = 30_000_000;

    let remaining_space = TOTAL_DISK_SPACE - root_size;

    let minimum_space_to_free = SPACE_REQUIRED_FOR_UPDATE - remaining_space;

    println!("Minimmum space to free: {}", minimum_space_to_free);

    let mut dir_sizes = path_to_directory_size
        .iter()
        .map(|(_, dir_size)| dir_size)
        .collect::<Vec<_>>();

    dir_sizes.sort_unstable();

    for dir_size in dir_sizes {
        if dir_size >= &minimum_space_to_free {
            return Ok(*dir_size);
        }
    }

    Err("Unable to find the magic directory".into())
}

fn build_child_path(current_path: &str, dir_name: &str) -> String {
    current_path.to_string() + dir_name + "/"
}

fn build_previous_path(current_path: &str) -> String {
    let mut path_items = current_path.trim_end_matches("/").split("/").collect::<Vec<_>>();
    path_items.pop();

    path_items.join("/") + "/"
}

enum LsResult<'a> {
    File { name: &'a str, size: usize },
    Dir(&'a str),
}

impl<'a> LsResult<'a> {
    fn try_from_line(line: &'a str) -> Result<Self, Box<dyn std::error::Error>> {
        let ls_elements = line.trim().trim_end().split(" ").collect::<Vec<&str>>();
        if ls_elements.len() != 2 {
            return Err(format!(
                "Unexpected line after `ls`, expected a format such as `dir <DIR_NAME>` or `<FILE_SIZE> <FILENAME>`, got {}",
                line
            )
            .into());
        }
        match ls_elements[0] {
            "dir" => {
                let dir_name = ls_elements[1];
                if dir_name.is_empty() {
                    return Err("Unexpected `dir` with empty name".into());
                }
                return Ok(LsResult::Dir(dir_name));
            }
            other => {
                let file_name = ls_elements[1];
                if file_name.is_empty() {
                    return Err("Unexpected `file` with empty name".into());
                }
                let file_size = other.parse::<usize>()?;
                return Ok(LsResult::File {
                    name: file_name,
                    size: file_size,
                });
            }
        }
    }
}

enum Cmd<'a> {
    Cd(&'a str),
    Ls,
}

impl<'a> Cmd<'a> {
    fn try_from_line(line: &'a str) -> Result<Self, Box<dyn std::error::Error>> {
        let cmd_elements = line.trim().trim_end().split(" ").collect::<Vec<&str>>();
        if cmd_elements.len() == 1 {
            return Err(format!(
                "Unexpected command line, expected a format such as `$ cd <ARGS>` or `$ ls`, got {}",
                line
            )
            .into());
        }

        let cmd_type = cmd_elements[1];
        match cmd_type {
            "ls" => {
                return Ok(Cmd::Ls);
            }
            "cd" => {
                if cmd_elements.len() != 3 {
                    return Err(format!("Unexpected `cd` command format, expected a format `$ cd <NAME>`, got {line}").into());
                }
                let target_dir_name = cmd_elements[2];
                return Ok(Cmd::Cd(target_dir_name));
            }
            other => {
                return Err(
                    format!("Unexpected command, expected `ls` or `cd`, got {}", other).into(),
                );
            }
        }
    }
}

fn parse_input_to_file_system(
    content: String,
) -> Result<FileSystem, Box<dyn std::error::Error>> {
    let mut file_system = FileSystem::new("/");
    let mut current_path = file_system.root_path.to_string();

    for line in content.lines() {
        let is_command = line.starts_with("$");
        if is_command {
            let cmd = Cmd::try_from_line(line)?;

            match cmd {
                Cmd::Ls => continue,
                Cmd::Cd(target_dir_name) => match target_dir_name {
                    ".." => {
                        println!("Going back to parent of path: {}", current_path);

                        current_path = build_previous_path(&current_path);
                    }
                    "/" => {
                        println!("Going back to root: {}", current_path);

                        current_path = "/".to_string();
                    }
                    dir_name => {
                        if !file_system.has_child(&current_path, dir_name) {
                            file_system.insert_dir_with_parent(&current_path, dir_name)?;
                        }

                        let dir_path = build_child_path(&current_path, dir_name);

                        println!("Going from {} to: {}", current_path, dir_path);

                        current_path = dir_path;
                    }
                },
            };
        } else {
            let ls_result = LsResult::try_from_line(line)?;

            match ls_result {
                LsResult::Dir(dir_name) => {
                    if !file_system.has_child(&current_path, dir_name) {
                        file_system.insert_dir_with_parent(&current_path, dir_name)?;
                    }
                }
                LsResult::File { name, size } => {
                    file_system.add_file(&current_path, name, size)?;
                }
            }
        }
    }
    return Ok(file_system);
}

struct FileSystem {
    root_path: String,
    path_to_directory: HashMap<String, Directory>

}

impl FileSystem {
    fn new(root_path: &str) -> Self {
        let mut path_to_directory = HashMap::new();

        let root_dir = Directory::new();
        path_to_directory.insert(root_path.to_string(), root_dir);

        FileSystem { root_path: root_path.to_string(), path_to_directory }
    }

    fn has_child(&self, parent_path: &str, dir_name: &str) -> bool {
        let child_path: String = build_child_path(parent_path, dir_name);
        self.path_to_directory.contains_key(&child_path)
    }

    fn insert_dir_with_parent(&mut self, parent_path: &str, dir_name: &str) -> Result<String, Box<dyn std::error::Error>> {
        let parent_dir = self.path_to_directory.get_mut(parent_path)
            .ok_or(format!(
                "Unexpected not found repository with path {} in list",
                parent_path
            ))?;
        parent_dir.children_dir_names.insert(dir_name.to_string());

        let child_path: String = build_child_path(parent_path, dir_name);

        self.path_to_directory.insert(
            child_path.clone(),
            Directory::new()
        );

        Ok(child_path)
    }

    fn add_file(&mut self, dir_path: &str, file_name: &str, file_size: usize) -> Result<(), Box<dyn std::error::Error>> {
        let dir = self.path_to_directory.get_mut(dir_path)
            .ok_or(format!(
                "Unexpected not found repository with path {} in list",
                dir_path
            ))?;
        dir.files.insert(file_name.to_string(), file_size);

        Ok(())
    }

    fn compute_directories_sizes(&self) -> Result<HashMap<String, usize>, Box<dyn std::error::Error>> {
        let mut path_to_directory_size = HashMap::new();
        self.compute_directory_size(&self.root_path, &mut path_to_directory_size)?;
        Ok(path_to_directory_size)
    }

    fn compute_directory_size(&self, dir_path: &str, map: &mut HashMap<String, usize>) -> Result<usize, Box<dyn std::error::Error>> {
        let directory = self.path_to_directory
            .get(dir_path)
            .ok_or(format!("Unable to find directory of path {:?}", dir_path))?;

        let mut files_size = 0;
        for (_, file_size) in &directory.files {
            files_size += file_size;
        }

        let mut children_directories_size = 0;
        for child_dir_name in &directory.children_dir_names {
            let child_dir_path = build_child_path(dir_path, child_dir_name);
            children_directories_size += self.compute_directory_size(&child_dir_path, map)?;
        }

        let size = files_size + children_directories_size;

        map.insert(dir_path.to_string(), size);

        return Ok(size);
    }
}

#[derive(Debug)]
struct Directory {
    files: HashMap<String, usize>,
    children_dir_names: HashSet<String>,
}

impl Directory {
    fn new() -> Self {
        Self {
            files: HashMap::new(),
            children_dir_names: HashSet::new(),
        }
    }
}
