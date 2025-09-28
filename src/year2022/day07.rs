use std::{collections::HashMap, path::PathBuf};

use anyhow::Result;

type Size = u64;

pub(crate) fn part1(input: &str) -> Result<String> {
    let directory_sizes = process_commands(input)?;
    let sum: Size = directory_sizes
        .iter()
        .filter_map(|(_dir, size)| if *size <= 100_000 { Some(size) } else { None })
        .sum();
    Ok(sum.to_string())
}

pub(crate) fn part2(input: &str) -> Result<String> {
    const TOTAL_SYSTEM_SPACE: Size = 70000000;
    const TOTAL_REQUIRED_SPACE: Size = 30000000;

    let directory_sizes = process_commands(input)?;
    let current_space = directory_sizes
        .get(&PathBuf::from("/"))
        .expect("Root should exist");
    let free_space: Size = TOTAL_SYSTEM_SPACE - current_space;
    let space_to_claim: Size = TOTAL_REQUIRED_SPACE - free_space;

    let mut sizes: Vec<Size> = directory_sizes.iter().map(|(_dir, &size)| size).collect();
    sizes.retain(|size| size >= &space_to_claim);
    sizes.sort_unstable();

    let size = sizes
        .first()
        .expect("There should be at least one solution");
    Ok(size.to_string())
}

fn process_commands(input: &str) -> Result<HashMap<PathBuf, Size>> {
    let mut map: HashMap<PathBuf, Size> = HashMap::new();

    let mut current_path = PathBuf::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts[0] == "$" && parts[1] == "cd" {
            let dir = parts[2];

            if dir == ".." {
                current_path.pop();
            } else {
                current_path.push(dir);
                map.entry(current_path.clone()).or_insert_with(|| 0);
            }
        } else if parts[0] == "dir" {
            let dir = parts[1];
            map.entry(current_path.join(dir)).or_insert_with(|| 0);
        } else if parts[0] == "$" && parts[1] == "ls" {
            continue;
        } else {
            // This is a file
            let file_size = parts[0].parse::<Size>().expect("Size should be a number");

            let mut path_tracker = PathBuf::new();
            for component in current_path.components() {
                path_tracker.push(component);

                // Now path_tracker is a unique key (e.g., '/', '/a', '/a/e')
                map.entry(path_tracker.clone())
                    .and_modify(|size| *size += file_size);
            }
        }
    }

    Ok(map)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "\
$ cd /
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
    fn test_part1() -> Result<()> {
        let solution = part1(INPUT)?;
        assert_eq!(solution, "95437");

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let solution = part2(INPUT)?;
        assert_eq!(solution, "24933642");

        Ok(())
    }
}
