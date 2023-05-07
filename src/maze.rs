pub fn solve(input: Maze) -> Option<usize> {
    let start_pos = (0, 1);
    let goal = (input[0].len() - 1, input.len() - 2);

    let mut open = std::collections::HashMap::<(usize, usize), (usize, Option<Direction>)>::new();
    let mut closed = std::collections::HashMap::<(usize, usize), usize>::new();

    open.insert(start_pos, (0, None));

    loop {
        println!("OPEN: {open:?}");
        println!("CLOSED: {closed:?}");

        let next = open
            .iter()
            .min_by(|l, r| l.1.0.cmp(&r.1.0))
            .map(|(p, (f, d))| (*p, (*f, *d)));


        println!("NEXT: {next:?}");

        if let Some((pos, (v,d))) = next {
            if pos == goal {
                return Some(v);
            }

            for n in neighbours(pos, &input) {

                let g: usize = if d.is_none() || Some(n.1) == d {
                    v
                } else {
                    v + 1
                };

                let h = 0;

                let f = g + h;

                match closed.get(&n.0) {
                    Some(c) if *c < f => continue,
                    _ => {}
                }

                match open.get(&n.0) {
                    Some((o, _)) if *o < f => continue,
                    _ => {}
                }

                println!("N: {:?} {:?}", n.0, n.1);
                open.insert(n.0, (f, Some(n.1)));
            }

            open.remove(&pos);
            closed.insert(pos, v);
        } else {
            return None;
        }
    }

    Some(0)
}

type Maze = Vec<Vec<bool>>;
type Pos = (usize, usize);

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
        Direction::Down if y < maze.len() - 1 && maze[y + 1][x] => {
            Some(((x, y + 1), Direction::Down))
        }
        Direction::Left if x > 0 && maze[y][x - 1] => Some(((x - 1, y), Direction::Left)),
        Direction::Right if x < maze[0].len() - 1 && maze[y][x + 1] => {
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
}
