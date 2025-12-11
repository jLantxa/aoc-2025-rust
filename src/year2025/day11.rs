use std::collections::HashMap;

use anyhow::{Result, bail};

pub(crate) fn part1(input: &str) -> Result<String> {
    let graph = parse_graph(input);
    let num_paths = count_paths(&graph, "you", "out", &mut HashMap::new());
    Ok(num_paths.to_string())
}

pub(crate) fn part2(input: &str) -> Result<String> {
    bail!("Unimplemented");
}

type Graph<'a> = HashMap<&'a str, Vec<&'a str>>;

fn parse_graph(input: &str) -> Graph<'_> {
    let mut graph = HashMap::new();

    input.trim().lines().for_each(|line| {
        let (src, dst_part) = line.split_once(":").expect("Should have ':' separator");
        graph.insert(
            src,
            dst_part
                .split_whitespace()
                .map(|dst_str| dst_str.trim())
                .collect(),
        );
    });

    graph
}

fn count_paths<'a>(
    graph: &'a Graph,
    src: &'a str,
    dst: &str,
    memo: &mut HashMap<&'a str, u64>,
) -> u64 {
    if src == dst {
        return 1;
    } else if let Some(&count) = memo.get(src) {
        return count;
    }

    let next_nodes = match graph.get(src) {
        Some(nodes) => nodes,
        None => return 0,
    };

    let mut num_paths = 0;
    for &node in next_nodes {
        num_paths += count_paths(graph, node, dst, memo);
    }

    memo.insert(src, num_paths);
    num_paths
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(INPUT)?, 5.to_string());
        Ok(())
    }
}
