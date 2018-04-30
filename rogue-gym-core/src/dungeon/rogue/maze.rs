use dungeon::{Coord, Direction};
use error::{GameResult, ResultExt};
use rect_iter::RectRange;
use rng::RngHandle;
use std::collections::HashSet;

/// Dig into the maze in the specified range
/// range: a 2D range you want to dig the maze in
/// rng: random number generator
/// register: a closure which register the coordinates of maze into your dungeon
pub(super) fn dig_maze<F>(
    range: RectRange<i32>,
    rng: &mut RngHandle,
    mut register: F,
) -> GameResult<()>
where
    F: FnMut(Coord) -> GameResult<()>,
{
    let mut used = HashSet::new();
    let start: Coord = range.upper_left().into();
    register(start).chain_err("[dungeon::rogue::maze::dig_maze]")?;
    used.insert(start);
    dig_impl(range, rng, &mut register, &mut used, start)
        .map_err(|e| e.chain("[dungeon::rogue::maze::dig_maze]"))
}

/// implementatiog maze digging as DFS
fn dig_impl<F>(
    range: RectRange<i32>,
    rng: &mut RngHandle,
    register: &mut F,
    used: &mut HashSet<Coord>,
    current_cd: Coord,
) -> GameResult<()>
where
    F: FnMut(Coord) -> GameResult<()>,
{
    loop {
        let range_cloned = range.clone();
        let dig_dir = Direction::iter_variants()
            .take(4)
            .filter(|dir| {
                let nxt = current_cd + dir.to_cd().scale(2, 2);
                range.contains(nxt) && !used.contains(&nxt)
            })
            .enumerate()
            .filter(|(i, _)| rng.does_happen(*i as u32 + 1))
            .last()
            .map(|t| t.1);
        let dig_dir = match dig_dir {
            Some(d) => d,
            None => break,
        };
        for cd in current_cd.dir_iter(dig_dir, |_| false).skip(1).take(2) {
            if used.insert(cd) {
                register(cd).chain_err("[dungeon::rogue::maze::dig_impl]")?;
            }
        }
        let next = current_cd + dig_dir.to_cd().scale(2, 2);
        dig_impl(range_cloned, rng, register, used, next)?;
    }
    Ok(())
}

#[cfg(test)]
mod maze_test {
    use super::*;
    use rect_iter::GetMut2D;
    #[test]
    fn print_maze() {
        let mut rng = RngHandle::new();
        let range = RectRange::from_ranges(20..50, 10..20).unwrap();
        let mut buffer = vec![vec![false; 80]; 24];
        dig_maze(range, &mut rng, |cd| {
            *buffer.get_mut_p(cd) = true;
            Ok(())
        }).unwrap();
        for v in buffer {
            for f in v {
                if f {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!("");
        }
    }
}