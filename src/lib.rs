#![allow(non_camel_case_types, unused_assignments)]

use std::{
    convert::{TryFrom, TryInto},
    fmt::Debug,
    iter,
    os::raw::{c_int, c_void},
    ptr::null_mut,
};

pub const PATH_FINDER_MAX_CELLS: usize = 1024;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PathFinder {
    pub cols: i32,
    pub rows: i32,
    pub start: i32,
    pub end: i32,
    pub has_path: u8,
    pub state: [u8; PATH_FINDER_MAX_CELLS],
    pub parents: [i32; PATH_FINDER_MAX_CELLS],
    pub g_score: [i32; PATH_FINDER_MAX_CELLS],
    pub f_score: [i32; PATH_FINDER_MAX_CELLS],
    pub fill_func: Option<fn(path_finder: &mut PathFinder, col: i32, row: i32) -> u8>,
    #[allow(clippy::type_complexity)]
    pub score_func:
        Option<fn(path_finder: &mut PathFinder, col: i32, row: i32, data: *mut c_void) -> i32>,
    pub data: *mut c_void,
}

impl PathFinder {
    pub fn cell(&self, col: i32, row: i32) -> CellRef<'_> {
        CellRef::new(self, self.cell_index(col, row))
    }

    pub fn get(&self, cell_index: usize) -> CellRef<'_> {
        if cell_index >= self.size() {
            panic!("cell index out of bounds");
        }
        CellRef::new(self, cell_index)
    }

    pub fn get_mut(&mut self, cell_index: usize) -> CellMut<'_> {
        if cell_index >= self.size() {
            panic!("cell index out of bounds");
        }
        CellMut::new(self, cell_index)
    }

    pub fn col_and_row_from_index(&self, cell_index: usize) -> [i32; 2] {
        if cell_index >= self.size() {
            panic!("cell index out of bounds");
        }

        let cols: usize = self.cols.try_into().unwrap();
        let col = cell_index % cols;
        let row = cell_index / cols;

        [col.try_into().unwrap(), row.try_into().unwrap()]
    }

    pub fn cell_index(&self, col: i32, row: i32) -> usize {
        if col >= self.cols {
            panic!("col {} is above the limit of cols ({})", col, self.cols);
        }

        if row >= self.rows {
            panic!("row {} is above the limit of rows ({})", row, self.rows);
        }

        usize::try_from(row).unwrap() * usize::try_from(self.cols).unwrap()
            + usize::try_from(col).unwrap()
    }

    pub fn size(&self) -> usize {
        usize::try_from(self.cols).unwrap() * usize::try_from(self.cols).unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CellRef<'a> {
    pub state: &'a u8,
    pub parent: &'a i32,
    pub g_score: &'a i32,
    pub f_score: &'a i32,
}

impl<'a> CellRef<'a> {
    pub fn new(path_finder: &'a PathFinder, index: usize) -> Self {
        let PathFinder {
            state,
            parents,
            g_score,
            f_score,
            ..
        } = path_finder;

        let state = &state[index];
        let parent = &parents[index];
        let g_score = &g_score[index];
        let f_score = &f_score[index];

        Self {
            state,
            parent,
            g_score,
            f_score,
        }
    }

    pub fn to_cell(&self) -> Cell {
        let state = *self.state;
        let parent = *self.parent;
        let g_score = *self.g_score;
        let f_score = *self.f_score;

        Cell {
            state,
            parent,
            g_score,
            f_score,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CellMut<'a> {
    pub state: &'a mut u8,
    pub parent: &'a mut i32,
    pub g_score: &'a mut i32,
    pub f_score: &'a mut i32,
}

impl<'a> CellMut<'a> {
    pub fn new(path_finder: &'a mut PathFinder, index: usize) -> Self {
        let PathFinder {
            state,
            parents,
            g_score,
            f_score,
            ..
        } = path_finder;

        let state = &mut state[index];
        let parent = &mut parents[index];
        let g_score = &mut g_score[index];
        let f_score = &mut f_score[index];

        Self {
            state,
            parent,
            g_score,
            f_score,
        }
    }

    pub fn to_cell(&self) -> Cell {
        let state = *self.state;
        let parent = *self.parent;
        let g_score = *self.g_score;
        let f_score = *self.f_score;

        Cell {
            state,
            parent,
            g_score,
            f_score,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Cell {
    pub state: u8,
    pub parent: i32,
    pub g_score: i32,
    pub f_score: i32,
}

impl PartialEq<Cell> for CellRef<'_> {
    fn eq(&self, other: &Cell) -> bool {
        (*self.state).eq(&other.state)
            && (*self.parent).eq(&other.parent)
            && (*self.g_score).eq(&other.g_score)
            && (*self.f_score).eq(&other.f_score)
    }
}

impl<'a> PartialEq<Cell> for CellMut<'a> {
    fn eq(&self, other: &Cell) -> bool {
        (*self.state).eq(&other.state)
            && (*self.parent).eq(&other.parent)
            && (*self.g_score).eq(&other.g_score)
            && (*self.f_score).eq(&other.f_score)
    }
}

impl Debug for PathFinder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PathFinder")
            .field("cols", &self.cols)
            .field("rows", &self.rows)
            .field("start", &self.start)
            .field("end", &self.end)
            .field("has_path", &self.has_path)
            .field("state", &(&self.state as &[_]))
            .field("parents", &(&self.parents as &[_]))
            .field("g_score", &(&self.g_score as &[_]))
            .field("f_score", &(&self.f_score as &[_]))
            .finish()
    }
}

impl Default for PathFinder {
    fn default() -> Self {
        Self {
            cols: Default::default(),
            rows: Default::default(),
            start: Default::default(),
            end: Default::default(),
            has_path: Default::default(),
            state: [0; PATH_FINDER_MAX_CELLS],
            parents: [0; PATH_FINDER_MAX_CELLS],
            g_score: [0; PATH_FINDER_MAX_CELLS],
            f_score: [0; PATH_FINDER_MAX_CELLS],
            fill_func: Default::default(),
            score_func: Default::default(),
            data: null_mut(),
        }
    }
}

impl PartialEq for PathFinder {
    fn eq(&self, other: &PathFinder) -> bool {
        self.cols.eq(&other.cols)
            && self.rows.eq(&other.rows)
            && self.start.eq(&other.start)
            && self.end.eq(&other.end)
            && self.has_path.eq(&other.has_path)
            && self
                .state
                .iter()
                .zip(other.state.iter())
                .all(|(a, b)| a == b)
            && self
                .parents
                .iter()
                .zip(other.parents.iter())
                .all(|(a, b)| a == b)
            && self
                .g_score
                .iter()
                .zip(other.g_score.iter())
                .all(|(a, b)| a == b)
            && self
                .f_score
                .iter()
                .zip(other.f_score.iter())
                .all(|(a, b)| a == b)
    }
}

extern "C" fn path_finder_heuristic(path_finder: &PathFinder, cell: i32) -> i32 {
    let cell_y = cell / path_finder.cols;
    let cell_x = cell - cell_y * path_finder.cols;
    let end_y = path_finder.end / path_finder.cols;
    let end_x = path_finder.end - end_y * path_finder.cols;
    let dx = if cell_x > end_x {
        cell_x - end_x
    } else {
        end_x - cell_x
    };
    let dy = if cell_y > end_y {
        cell_y - end_y
    } else {
        end_y - cell_y
    };
    dx + dy
}

extern "C" fn path_finder_open_set_is_empty(path_finder: &PathFinder) -> u8 {
    use std::ops::Not;

    path_finder
        .state
        .iter()
        .take((path_finder.cols * path_finder.rows) as usize)
        .any(|state| state & 0x2 == 0x2)
        .not()
        .into()
}

extern "C" fn path_finder_lowest_in_open_set(path_finder: &PathFinder) -> i32 {
    path_finder
        .state
        .iter()
        .zip(path_finder.f_score.iter())
        .enumerate()
        .take((path_finder.cols * path_finder.rows) as usize)
        .filter(|(_, (&state, _))| state & 0x2 == 0x2)
        .map(|(index, (_, f_score))| (index, f_score))
        .min_by_key(|(_, &f_score)| f_score)
        .map(|(index, _)| index)
        .unwrap_or(0)
        .try_into()
        .unwrap()
}

extern "C" fn path_finder_reconstruct_path(path_finder: &mut PathFinder) {
    let &mut PathFinder {
        start,
        end,
        ref parents,
        ref mut state,
        ..
    } = path_finder;

    iter::successors(Some(end), |&index| Some(parents[index as usize]))
        .take_while(|&index| index != start)
        .skip(1)
        .for_each(|index| state[index as usize] |= 0x8);
}

#[no_mangle]
pub extern "C" fn path_finder_fill(path_finder: &mut PathFinder) {
    let fill_func = path_finder.fill_func.expect("non-null function pointer");
    let size: usize = (path_finder.rows * path_finder.cols).try_into().unwrap();

    let mut index_iter = 0..size.min(path_finder.state.len());
    for row in 0..path_finder.rows {
        for col in 0..path_finder.cols {
            let index = index_iter.next().unwrap();

            if fill_func(path_finder, col, row) == 0 {
                path_finder.state[index] &= !0x1;
            } else {
                path_finder.state[index] |= 0x1;
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn path_finder_begin(path_finder: &mut PathFinder) {
    path_finder.state[path_finder.start as usize] |= 0x2;
}

#[no_mangle]
pub extern "C" fn path_finder_find_step(path_finder: &mut PathFinder, data: *mut c_void) -> u8 {
    let count = path_finder.size();
    let current = path_finder_lowest_in_open_set(path_finder);

    if current == path_finder.end {
        path_finder_reconstruct_path(path_finder);
        path_finder.has_path = 1;
        0
    } else if path_finder_open_set_is_empty(path_finder) == 1 {
        path_finder.has_path = 0;
        0
    } else {
        let current_index: usize = current.try_into().unwrap();

        path_finder.state[current_index] = (path_finder.state[current_index] & !0x2) | 0x4;

        let neighbors = {
            let left = if current % path_finder.cols == 0 {
                -1
            } else {
                current - 1
            };
            let top = current - path_finder.cols;
            let right = if (current + 1) % path_finder.cols == 0 {
                -1
            } else {
                current + 1
            };
            let bottom = current + path_finder.cols;

            [left, top, right, bottom]
        };

        let cols: usize = path_finder.cols.try_into().unwrap();
        let g_score = path_finder.g_score[current_index] + 1;
        let score_func = path_finder.score_func;

        neighbors
            .iter()
            .filter_map(|&n| usize::try_from(n).ok())
            .filter(|&n| n < count)
            .for_each(|n| {
                let cell = path_finder.get_mut(n);
                if *cell.state & 0x4 == 0 {
                    if *cell.state & 0x1 == 0 {
                        *cell.state |= 0x4;
                    } else if *cell.state & 0x2 == 0 || g_score < *cell.g_score {
                        *cell.parent = current;
                        *cell.g_score = g_score;
                        let heuristics = path_finder_heuristic(path_finder, n.try_into().unwrap());

                        let cell = path_finder.get_mut(n);
                        *cell.f_score = g_score + heuristics;

                        let cell = match score_func {
                            Some(score_func) => {
                                let score = score_func(
                                    path_finder,
                                    (n % cols).try_into().unwrap(),
                                    (n / cols).try_into().unwrap(),
                                    data,
                                );

                                let cell = path_finder.get_mut(n);
                                *cell.f_score += score;

                                cell
                            }
                            None => cell,
                        };

                        *cell.state |= 0x2;
                    }
                }
            });

        1
    }
}
#[no_mangle]
pub extern "C" fn path_finder_find(path_finder: &mut PathFinder, data: *mut c_void) {
    path_finder_begin(path_finder);
    while path_finder_find_step(path_finder, data) as c_int == 1 as c_int {}
}

#[no_mangle]
pub extern "C" fn path_finder_get_heuristic_score(
    path_finder: &mut PathFinder,
    col: i32,
    row: i32,
) -> i32 {
    path_finder.f_score[(row * path_finder.cols + col) as usize]
}

#[no_mangle]
pub extern "C" fn path_finder_is_passable(path_finder: &mut PathFinder, col: i32, row: i32) -> u8 {
    (path_finder.state[(row * path_finder.cols + col) as usize] as c_int & 0x1 as c_int
        == 0x1 as c_int) as c_int as u8
}

#[no_mangle]
pub extern "C" fn path_finder_is_closed(path_finder: &mut PathFinder, col: i32, row: i32) -> u8 {
    (path_finder.state[(row * path_finder.cols + col) as usize] as c_int & 0x4 as c_int
        == 0x4 as c_int) as c_int as u8
}

#[no_mangle]
pub extern "C" fn path_finder_is_open(path_finder: &mut PathFinder, col: i32, row: i32) -> u8 {
    (path_finder.state[(row * path_finder.cols + col) as usize] as c_int & 0x2 as c_int
        == 0x2 as c_int) as c_int as u8
}

#[no_mangle]
pub extern "C" fn path_finder_is_path(path_finder: &mut PathFinder, col: i32, row: i32) -> u8 {
    (path_finder.state[(row * path_finder.cols + col) as usize] as c_int & 0x8 as c_int
        == 0x8 as c_int) as c_int as u8
}

#[no_mangle]
pub extern "C" fn path_finder_is_start(path_finder: &mut PathFinder, col: i32, row: i32) -> u8 {
    (row * path_finder.cols + col == path_finder.start) as c_int as u8
}

#[no_mangle]
pub extern "C" fn path_finder_is_end(path_finder: &mut PathFinder, col: i32, row: i32) -> u8 {
    (row * path_finder.cols + col == path_finder.end) as c_int as u8
}

#[no_mangle]
pub extern "C" fn path_finder_set_start(path_finder: &mut PathFinder, col: i32, row: i32) {
    path_finder.start = row * path_finder.cols + col;
}

#[no_mangle]
pub extern "C" fn path_finder_set_end(path_finder: &mut PathFinder, col: i32, row: i32) {
    path_finder.end = row * path_finder.cols + col;
}

#[no_mangle]
pub extern "C" fn path_finder_clear_path(path_finder: &mut PathFinder) {
    let mut i: i32 = 0;
    i = 0 as c_int;
    while i < PATH_FINDER_MAX_CELLS as c_int {
        path_finder.state[i as usize] = (path_finder.state[i as usize] as c_int
            & !(0x2 as c_int | 0x4 as c_int | 0x8 as c_int))
            as u8;
        path_finder.parents[i as usize] = 0 as c_int;
        path_finder.g_score[i as usize] = 0 as c_int;
        path_finder.f_score[i as usize] = 0 as c_int;
        i += 1
    }
    path_finder.has_path = 0 as c_int as u8;
}

#[no_mangle]
pub extern "C" fn path_finder_initialize(path_finder: &mut PathFinder) {
    let mut i: i32 = 0;
    i = 0 as c_int;
    while i < PATH_FINDER_MAX_CELLS as c_int {
        path_finder.parents[i as usize] = 0 as c_int;
        path_finder.g_score[i as usize] = 0 as c_int;
        path_finder.f_score[i as usize] = 0 as c_int;
        path_finder.state[i as usize] = 0x1 as c_int as u8;
        i += 1
    }
    path_finder.rows = 0 as c_int;
    path_finder.cols = 0 as c_int;
    path_finder.start = 0 as c_int;
    path_finder.end = 0 as c_int;
    path_finder.has_path = 0 as c_int as u8;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_set_is_empty() {
        let mut path_finder = PathFinder {
            cols: 5,
            rows: 4,
            ..Default::default()
        };

        assert_eq!(path_finder_open_set_is_empty(&path_finder), 1);

        path_finder.state[7] = 2;
        assert_eq!(path_finder_open_set_is_empty(&path_finder), 0);
    }

    #[test]
    fn lowest_in_open_set() {
        let path_finder = PathFinder::default();
        assert_eq!(path_finder_lowest_in_open_set(&path_finder), 0);

        let mut path_finder = PathFinder {
            cols: 5,
            rows: 4,
            ..Default::default()
        };
        assert_eq!(path_finder_lowest_in_open_set(&path_finder), 0);

        path_finder.state[3] = 0x2;
        path_finder.f_score[3] = -5;
        assert_eq!(path_finder_lowest_in_open_set(&path_finder), 3);

        path_finder.state[6] = 0x2;
        path_finder.f_score[6] = -9;
        assert_eq!(path_finder_lowest_in_open_set(&path_finder), 6);

        path_finder.state[11] = 0x2;
        path_finder.f_score[11] = -7;
        assert_eq!(path_finder_lowest_in_open_set(&path_finder), 6);

        path_finder.f_score[11] = -10;
        assert_eq!(path_finder_lowest_in_open_set(&path_finder), 11);
    }

    #[test]
    fn reconstruct_path() {
        /*
         * Representation
         *
         * /-----\
         * | >>>v|
         * | S v<|
         * |v<<< |
         * |>>>E |
         * \-----/
         */

        let mut path_finder = PathFinder {
            cols: 5,
            rows: 4,
            start: 6,
            end: 18,
            ..Default::default()
        };

        let parents = &mut path_finder.parents;
        parents[18] = 17;
        parents[17] = 16;
        parents[16] = 15;
        parents[15] = 10;
        parents[10] = 11;
        parents[11] = 12;
        parents[12] = 13;
        parents[13] = 8;
        parents[8] = 9;
        parents[9] = 4;
        parents[4] = 3;
        parents[3] = 2;
        parents[2] = 1;
        parents[1] = 6;

        path_finder_reconstruct_path(&mut path_finder);
        path_finder
            .parents
            .iter()
            .copied()
            .zip(path_finder.state.iter().copied())
            .enumerate()
            .for_each(|(index, (parent_index, state))| {
                if parent_index == 0 || index as i32 == path_finder.end {
                    assert_eq!(state, 0);
                } else {
                    assert_eq!(state, 0x8);
                }
            });
    }

    #[test]
    fn fill() {
        let mut path_finder = PathFinder {
            cols: 5,
            rows: 4,
            ..Default::default()
        };
        const IMPASSABLES: [usize; 6] = [1, 2, 3, 5, 8, 13];
        let fill_func = |path_finder: &mut PathFinder, col: i32, row: i32| -> u8 {
            let index = (row * path_finder.cols + col).try_into().unwrap();
            IMPASSABLES.binary_search(&index).is_err() as u8
        };
        path_finder.fill_func = Some(fill_func);

        let size = path_finder.cols * path_finder.rows;
        let test_states = move |path_finder: &PathFinder| {
            path_finder
                .state
                .iter()
                .take(size as usize)
                .copied()
                .enumerate()
                .for_each(|(index, state)| {
                    if IMPASSABLES.binary_search(&index).is_err() {
                        assert_eq!(state, 0x1);
                    } else {
                        assert_eq!(state, 0);
                    }
                })
        };

        path_finder_fill(&mut path_finder);
        test_states(&path_finder);

        path_finder.state.iter_mut().for_each(|state| *state = 0x1);
        path_finder_fill(&mut path_finder);
        test_states(&path_finder);
    }

    #[test]
    fn begin() {
        let mut path_finder = PathFinder {
            start: 13,
            ..Default::default()
        };

        path_finder_begin(&mut path_finder);
        assert_eq!(path_finder.state[13], 0x2);
        path_finder
            .state
            .iter()
            .copied()
            .take(13)
            .chain(path_finder.state.iter().copied().skip(14))
            .for_each(|state| assert_eq!(state, 0));
    }

    #[test]
    fn find_step_at_end() {
        /*
         * Representation
         *
         * /-----\
         * |     |
         * | S   |
         * | v   |
         * | >>E |
         * \-----/
         */

        let mut path_finder = PathFinder {
            cols: 5,
            rows: 4,
            start: 6,
            end: 18,
            ..Default::default()
        };

        const PATH_INDICES: [usize; 4] = [11, 16, 17, 18];
        let parents = &mut path_finder.parents;
        parents[11] = 6;
        parents[16] = 11;
        parents[17] = 16;
        parents[18] = 17;

        PATH_INDICES
            .iter()
            .for_each(|&index| path_finder.state[index] = 0x2);
        path_finder.f_score[18] = -10;

        let run = path_finder_find_step(&mut path_finder, null_mut());
        assert_eq!(run, 0);
        assert_eq!(path_finder.has_path, 1);
        path_finder
            .state
            .iter()
            .take((path_finder.cols * path_finder.rows).try_into().unwrap())
            .copied()
            .enumerate()
            .for_each(|(index, state)| {
                if path_finder.end == index.try_into().unwrap() {
                    assert_eq!(state, 0x2);
                } else if PATH_INDICES.binary_search(&index).is_ok() {
                    assert_eq!(state, 0xa);
                } else {
                    assert_eq!(state, 0);
                }
            });
    }

    #[test]
    fn find_step_empty_set() {
        let mut path_finder = PathFinder {
            cols: 5,
            rows: 4,
            start: 6,
            end: 18,
            ..Default::default()
        };
        path_finder.f_score[18] = -10;

        let run = path_finder_find_step(&mut path_finder, null_mut());
        assert_eq!(run, 0);
        assert_eq!(path_finder.has_path, 0);
        assert!(path_finder.state.iter().copied().all(|state| state == 0));
    }

    #[test]
    fn find_step() {
        let mut path_finder = create_complex_map();
        path_finder_fill(&mut path_finder);
        path_finder_begin(&mut path_finder);

        check_next_step(&mut path_finder);
        assert_eq!(
            path_finder.cell(6, 5),
            Cell {
                state: 5,
                ..Default::default()
            }
        );

        for _step in 0..312 {
            assert!(check_next_step(&mut path_finder));
        }

        assert!(!check_next_step(&mut path_finder));
    }

    fn check_next_step(path_finder: &mut PathFinder) -> bool {
        let current_cell_index = path_finder_lowest_in_open_set(path_finder);
        let current_cell = path_finder
            .get(current_cell_index.try_into().unwrap())
            .to_cell();
        let [col, row] = path_finder.col_and_row_from_index(current_cell_index.try_into().unwrap());
        let g_score = current_cell.g_score + 1;
        let score_func = path_finder.score_func.unwrap();

        const OFFSETS: [[i32; 2]; 4] = [[-1, 0], [0, 1], [1, 0], [0, -1]];
        let mut old_cells = [
            Cell::default(),
            Cell::default(),
            Cell::default(),
            Cell::default(),
        ];

        OFFSETS
            .iter()
            .zip(old_cells.iter_mut())
            .for_each(|(&[x, y], old_cell)| {
                let col = match col.checked_add(x) {
                    Some(col) if col >= 0 && col < path_finder.cols => col,
                    _ => return,
                };

                let row = match row.checked_add(y) {
                    Some(row) if row >= 0 && row < path_finder.rows => row,
                    _ => return,
                };

                *old_cell = path_finder.cell(col, row).to_cell();
            });

        if path_finder_find_step(path_finder, null_mut()) != 1 {
            return false;
        }

        OFFSETS
            .iter()
            .zip(old_cells.iter())
            .for_each(|(&[x, y], old_cell)| {
                let col = match col.checked_add(x) {
                    Some(col) if col >= 0 && col < path_finder.cols => col,
                    _ => return,
                };

                let row = match row.checked_add(y) {
                    Some(row) if row >= 0 && row < path_finder.rows => row,
                    _ => return,
                };

                let cell = path_finder.cell(col, row);
                if *cell.parent != current_cell_index {
                    return;
                }

                if old_cell.state & 0x1 == 0 {
                    assert_eq!(
                        path_finder.cell(col, row),
                        Cell {
                            state: old_cell.state | 0x4,
                            parent: old_cell.parent,
                            g_score: old_cell.g_score,
                            f_score: old_cell.f_score,
                        }
                    );
                } else {
                    let (g_score, f_score) =
                        if old_cell.state & 0x2 == 0 || g_score < old_cell.g_score {
                            let heuristics = path_finder_heuristic(
                                path_finder,
                                path_finder.cell_index(col, row).try_into().unwrap(),
                            );
                            let score = score_func(path_finder, col, row, null_mut());
                            let f_score = score + heuristics + g_score;

                            (g_score, f_score)
                        } else {
                            (old_cell.g_score, old_cell.f_score)
                        };

                    assert_eq!(
                        path_finder.cell(col, row),
                        Cell {
                            state: 3,
                            parent: current_cell_index,
                            g_score,
                            f_score,
                        }
                    );
                }
            });

        true
    }

    fn create_complex_map() -> PathFinder {
        /*
         * Representation
         *              1         2
         *    0123456789012345678901234
         *   /-------------------------\
         *  0|                         |
         *  1|     fffff               |
         *  2|     fffff               |
         *  3|        #                |
         *  4|        #  ffffff        |
         *  5|      S #  ffffff        |
         *  6|        #                |
         *  7|##############           |
         *  8|          11111          |
         *  9|          f##############|
         * 10|          f      #       |
         *  1|              #81#  E    |
         *  2|              #  #       |
         *  3|              #18######1 |
         *  4|              #          |
         *  5|              #          |
         *   \-------------------------/
         */

        PathFinder {
            cols: 25,
            rows: 16,
            start: 131,
            end: 295,
            fill_func: Some(create_complex_map_fill_func),
            score_func: Some(create_complex_map_score_func),
            ..Default::default()
        }
    }

    struct Pos {
        row: i32,
        col: i32,
    }

    struct Area {
        first: Pos,
        last: Pos,
    }

    fn create_complex_map_fill_func(_path_finder: &mut PathFinder, col: i32, row: i32) -> u8 {
        const WALLS: [Area; 6] = [
            Area {
                first: Pos { row: 3, col: 8 },
                last: Pos { row: 6, col: 8 },
            },
            Area {
                first: Pos { row: 7, col: 0 },
                last: Pos { row: 7, col: 13 },
            },
            Area {
                first: Pos { row: 9, col: 11 },
                last: Pos { row: 9, col: 24 },
            },
            Area {
                first: Pos { row: 11, col: 14 },
                last: Pos { row: 15, col: 14 },
            },
            Area {
                first: Pos { row: 10, col: 17 },
                last: Pos { row: 13, col: 17 },
            },
            Area {
                first: Pos { row: 13, col: 18 },
                last: Pos { row: 13, col: 22 },
            },
        ];

        WALLS
            .iter()
            .find(|wall| {
                wall.first.row <= row
                    && wall.first.col <= col
                    && wall.last.row >= row
                    && wall.last.col >= col
            })
            .is_none() as u8
    }

    fn create_complex_map_score_func(
        _path_finder: &mut PathFinder,
        col: i32,
        row: i32,
        _data: *mut c_void,
    ) -> i32 {
        struct Danger {
            area: Area,
            score: i32,
        }

        const DANGERS: [Danger; 9] = [
            Danger {
                area: Area {
                    first: Pos { row: 1, col: 4 },
                    last: Pos { row: 2, col: 9 },
                },
                score: 0xf,
            },
            Danger {
                area: Area {
                    first: Pos { row: 4, col: 11 },
                    last: Pos { row: 5, col: 16 },
                },
                score: 0xf,
            },
            Danger {
                area: Area {
                    first: Pos { row: 8, col: 10 },
                    last: Pos { row: 8, col: 14 },
                },
                score: 1,
            },
            Danger {
                area: Area {
                    first: Pos { row: 9, col: 10 },
                    last: Pos { row: 10, col: 10 },
                },
                score: 0xf,
            },
            Danger {
                area: Area {
                    first: Pos { row: 11, col: 15 },
                    last: Pos { row: 11, col: 15 },
                },
                score: 8,
            },
            Danger {
                area: Area {
                    first: Pos { row: 11, col: 16 },
                    last: Pos { row: 11, col: 16 },
                },
                score: 1,
            },
            Danger {
                area: Area {
                    first: Pos { row: 13, col: 15 },
                    last: Pos { row: 13, col: 15 },
                },
                score: 1,
            },
            Danger {
                area: Area {
                    first: Pos { row: 13, col: 16 },
                    last: Pos { row: 13, col: 16 },
                },
                score: 8,
            },
            Danger {
                area: Area {
                    first: Pos { row: 13, col: 13 },
                    last: Pos { row: 13, col: 13 },
                },
                score: 1,
            },
        ];

        DANGERS
            .iter()
            .find(|danger| {
                danger.area.first.row <= row
                    && danger.area.first.col <= col
                    && danger.area.last.row >= row
                    && danger.area.last.col >= col
            })
            .map(|danger| danger.score)
            .unwrap_or(0)
    }
}
