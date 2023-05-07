use std::error::Error;

pub fn solve(input: Maze) -> Option<usize> {
    let start_pos = (0, 1);
    let goal = (input.max_x(), input.max_y() - 1);

    let mut open =
        std::collections::HashMap::<(usize, usize), (usize, usize, Option<Direction>)>::new();
    let mut closed = std::collections::HashMap::<(usize, usize), usize>::new();

    open.insert(start_pos, (0, 0, None));

    loop {
        let next = open
            .iter()
            .min_by(|l, r| l.1 .0.cmp(&r.1 .0))
            .map(|(p, (f, g, d))| (*p, (*f, *g, *d)));

        println!("NEXT: {:?}", next);

        if let Some((pos, (of, og, d))) = next {
            if pos == goal {
                return Some(og);
            }

            for n in neighbours(pos, &input) {
                let g = g(d, n.1, og);
                let h = h(n.1, n.0, goal);
                let f = g + h;

                match closed.get(&n.0) {
                    Some(c) if *c < f => continue,
                    _ => {}
                }

                match open.get(&n.0) {
                    Some((o, _, _)) if *o < f => continue,
                    _ => {}
                }

                println!("Adding {:?} with f {}", n.1, f);

                open.insert(n.0, (f, g, Some(n.1)));
            }

            open.remove(&pos);
            closed.insert(pos, of);
        } else {
            return None;
        }
    }
}

// A* "g" function determining the cost of moving to the current spot.
// It does it by comparing the previous direction on the path with the one required
// to move to the currently examined neighbour
fn g(
    previous_direction: Option<Direction>,
    neighbour_direction: Direction,
    corners_so_far: usize,
) -> usize {
    match previous_direction {
        Some(p) if p != neighbour_direction => corners_so_far + 1,
        _ => corners_so_far,
    }
}

// A* "h" - a heuristic trying to guess the remaining cost to get to the goal
// This version does it by taking the minimal number of corners required to get to the goal assuming there are no walls in the maze
fn h(d: Direction, (pos_x, pos_y): Pos, (goal_x, goal_y): Pos) -> usize {
    match (d, pos_x, pos_y) {
        (Direction::Right, _, y) if y == goal_y => 0,
        (Direction::Right, _, y) if y != goal_y => 1,

        (Direction::Up, x, y) if x == goal_x && y > goal_y => 0,
        (Direction::Up, x, y) if x == goal_x && y < goal_y => 3,
        (Direction::Up, x, y) if x != goal_x && y > goal_y => 1,
        (Direction::Up, x, y) if x != goal_x && y < goal_y => 3,
        (Direction::Up, x, y) if x != goal_x && y == goal_y => 2,

        (Direction::Down, x, y) if x == goal_x && y < goal_y => 0,
        (Direction::Down, x, y) if x == goal_x && y > goal_y => 3,
        (Direction::Down, x, y) if x != goal_x && y < goal_y => 1,
        (Direction::Down, x, y) if x != goal_x && y > goal_y => 3,
        (Direction::Down, x, y) if x != goal_x && y == goal_y => 2,

        (Direction::Left, _, y) if y == goal_y => 3,
        _ => 2,
    }
}

type Maze = Vec<Vec<bool>>;
type Pos = (usize, usize);

trait MazeExt {
    fn max_y(&self) -> usize;
    fn max_x(&self) -> usize;
}

impl MazeExt for Maze {
    fn max_y(&self) -> usize {
        self.len() - 1
    }

    fn max_x(&self) -> usize {
        self[0].len() - 1
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn neighbours((x, y): Pos, maze: &Maze) -> impl Iterator<Item = (Pos, Direction)> + '_ {
    [
        Direction::Down,
        Direction::Up,
        Direction::Right,
        Direction::Left,
    ]
    .into_iter()
    .filter_map(move |d| match d {
        Direction::Up if y > 0 && maze[y - 1][x] => Some(((x, y - 1), Direction::Up)),
        Direction::Down if y < maze.max_y() && maze[y + 1][x] => {
            Some(((x, y + 1), Direction::Down))
        }
        Direction::Left if x > 0 && maze[y][x - 1] => Some(((x - 1, y), Direction::Left)),
        Direction::Right if x < maze.max_x() && maze[y][x + 1] => {
            Some(((x + 1, y), Direction::Right))
        }
        _ => None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_input(s: &str) -> Maze {
        s.split_whitespace()
            .map(|row| row.chars().map(|c| c != '0').collect())
            .collect()
    }

    #[test]
    fn test_example_from_task_description() {
        let input = "000000000
                           ....0...0
                           0.0.0.0.0
                           0.0...0.0
                           0.00000.0
                           0....00.0
                           0.00.....
                           000000000";

        assert_eq!(solve(to_input(input)), Some(4));
    }

    #[test]
    fn no_solution() {
        let input = "000000000
                           ....0...0
                           0.0.0.0.0
                           0.0...0.0
                           0.00000.0
                           0....00.0
                           0.00...0.
                           000000000";

        assert_eq!(solve(to_input(input)), None);
    }

    #[test]
    fn no_walls() {
        let input = "000000000
                           .00000000
                           .00000000
                           .00000000
                           .00000000
                           .00000000
                           .0000000.
                           .........";

        assert_eq!(solve(to_input(input)), Some(2));
    }

    #[test]
    fn no_corner_on_first_step() {
        let input = "000000000
                           ........0
                           .000000.0
                           .000000.0
                           .000000.0
                           .000000.0
                           .........
                           000000000";

        assert_eq!(solve(to_input(input)), Some(1));
    }

    #[test]
    fn no_maze() {
        let input = ".........
                           .........
                           .........
                           .........
                           .........
                           .........
                           .........
                           .........";

        assert_eq!(solve(to_input(input)), Some(1));
    }
}
