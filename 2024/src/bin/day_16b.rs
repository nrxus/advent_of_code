use std::{
    cmp,
    collections::{hash_map, BinaryHeap, HashMap, HashSet},
};

fn solve(input: &str) -> usize {
    let mut start = None;
    let mut end = None;

    let walls: HashSet<_> = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes()
                .enumerate()
                .filter(|(_, b)| *b != b'.')
                .map(move |(x, b)| ((x as isize, y as isize), b))
        })
        .filter_map(|(pos, c)| match c {
            b'#' => Some(pos),
            b'S' => {
                assert!(start.replace(pos).is_none());
                None
            }
            b'E' => {
                assert!(end.replace(pos).is_none());
                None
            }
            _ => unreachable!(),
        })
        .collect();

    let start = start.unwrap();
    let end = end.unwrap();

    let start = Node {
        pos: start,
        direction: Direction::East,
    };
    let mut frontier = BinaryHeap::from_iter([(cmp::Reverse(0_u32), start, vec![start.pos])]);
    let mut seen: HashMap<Node, u32> = HashMap::from_iter([(start, 0_u32)]);
    let mut best_path: Option<(u32, HashSet<(isize, isize)>)> = None;

    while let Some((cmp::Reverse(cost), node, paths)) = frontier.pop() {
        match &mut best_path {
            None => {
                if node.pos == end {
                    best_path = Some((cost, HashSet::from_iter(paths)));
                    continue;
                }
            }
            Some((best_cost, old_paths)) => {
                if *best_cost < cost {
                    continue;
                }
                if node.pos == end {
                    old_paths.extend(paths);
                    continue;
                }
            }
        }

        let neighbors = [
            (cost + 1, node.advance()),
            (cost + 1001, node.turn_left().advance()),
            (cost + 1001, node.turn_right().advance()),
        ]
        .into_iter()
        .filter_map(|(cost, node)| {
            if walls.contains(&node.pos) {
                return None;
            }

            match seen.entry(node) {
                hash_map::Entry::Occupied(mut o) => {
                    if *o.get() < cost {
                        None
                    } else {
                        o.insert(cost);
                        Some((cmp::Reverse(cost), node))
                    }
                }
                hash_map::Entry::Vacant(v) => {
                    v.insert(cost);
                    Some((cmp::Reverse(cost), node))
                }
            }
        })
        .map(|(cost, next)| {
            let mut paths = paths.clone();
            paths.push(next.pos);
            (cost, next, paths)
        });

        frontier.extend(neighbors);
    }

    let paths = best_path.unwrap().1;
    paths.len()
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Node {
    pos: (isize, isize),
    direction: Direction,
}

impl Node {
    pub fn advance(self) -> Self {
        let (x, y) = self.pos;

        match self.direction {
            Direction::North => Self {
                pos: (x, y - 1),
                ..self
            },
            Direction::South => Self {
                pos: (x, y + 1),
                ..self
            },
            Direction::East => Self {
                pos: (x + 1, y),
                ..self
            },
            Direction::West => Self {
                pos: (x - 1, y),
                ..self
            },
        }
    }

    pub fn turn_left(self) -> Self {
        match self.direction {
            Direction::North => Self {
                direction: Direction::West,
                ..self
            },
            Direction::South => Self {
                direction: Direction::East,
                ..self
            },
            Direction::East => Self {
                direction: Direction::North,
                ..self
            },
            Direction::West => Self {
                direction: Direction::South,
                ..self
            },
        }
    }

    pub fn turn_right(self) -> Self {
        match self.direction {
            Direction::North => Self {
                direction: Direction::East,
                ..self
            },
            Direction::South => Self {
                direction: Direction::West,
                ..self
            },
            Direction::East => Self {
                direction: Direction::South,
                ..self
            },
            Direction::West => Self {
                direction: Direction::North,
                ..self
            },
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    North,
    South,
    East,
    West,
}

common::read_main!();

#[test]
fn example() {
    assert_eq!(
        solve(
            r"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
"
        ),
        45
    );
}

#[test]
fn example_two() {
    assert_eq!(
        solve(
            r"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
"
        ),
        64
    );
}
