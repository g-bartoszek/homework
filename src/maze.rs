use std::cmp::Ordering;

pub fn solve(input: Maze) -> Option<usize> {
    let start_pos = (0, 1);
    let goal = (input.max_x(), input.max_y() - 1);

    let mut open = std::collections::BinaryHeap::<Next>::new();
    let mut closed = std::collections::HashMap::<Pos, usize>::new();

    open.push(Next {
        pos: start_pos,
        ..Default::default()
    });

    loop {
        let next = open.pop();

        if let Some(next) = next {
            if next.pos == goal {
                return Some(next.g);
            }

            for (pos, direction) in neighbours(next.pos, &input) {
                let g = g(next.direction, direction, next.g);
                let h = h(direction, pos, goal);
                let f = g + h;

                match closed.get(&pos) {
                    Some(c) if *c < f => continue,
                    _ => {}
                }

                if open.iter().find(|o| o.pos == pos && o.f < f).is_some() {
                    continue;
                }

                open.push(Next { pos, direction: Some(direction), g, f } );
            }

            closed.insert(next.pos, next.f);
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

/// Represents a A* algorithm path candidate
#[derive(PartialEq, Eq, Default)]
struct Next {
    pub pos: Pos,
    /// Previous direction used to detect corners on the path
    pub direction: Option<Direction>, 
    /// A* "g" - corners taken so far
    pub g: usize, 
    /// A* "f" - estimated total cost to get to the goal using this candidate
    pub f: usize, 
}

impl Ord for Next {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f.cmp(&self.f)
    }

}

impl PartialOrd for Next {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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
