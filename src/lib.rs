#![allow(non_camel_case_types, unused_assignments)]

use std::{
    convert::TryInto,
    fmt::Debug,
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

extern "C" fn path_finder_heuristic(path_finder: &mut PathFinder, cell: i32) -> i32 {
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
    let mut i: i32 = 0;
    i = path_finder.end;
    while i != path_finder.start {
        if path_finder.parents[i as usize] != path_finder.start {
            path_finder.state[path_finder.parents[i as usize] as usize] =
                (path_finder.state[path_finder.parents[i as usize] as usize] as c_int
                    | 0x8 as c_int) as u8
        }
        i = path_finder.parents[i as usize]
    }
}

#[no_mangle]
pub extern "C" fn path_finder_fill(path_finder: &mut PathFinder) {
    let mut row: i32 = 0;
    row = 0 as c_int;
    while row < path_finder.rows {
        let mut col: i32 = 0;
        col = 0 as c_int;
        while col < path_finder.cols {
            if path_finder.fill_func.expect("non-null function pointer")(
                &mut *path_finder,
                col,
                row,
            ) as c_int
                == 1 as c_int
            {
                path_finder.state[(row * path_finder.cols + col) as usize] =
                    (path_finder.state[(row * path_finder.cols + col) as usize] as c_int
                        | 0x1 as c_int) as u8
            } else {
                path_finder.state[(row * path_finder.cols + col) as usize] =
                    (path_finder.state[(row * path_finder.cols + col) as usize] as c_int
                        & !(0x1 as c_int)) as u8
            }
            col += 1
        }
        row += 1
    }
}

#[no_mangle]
pub extern "C" fn path_finder_begin(path_finder: &mut PathFinder) {
    path_finder.state[path_finder.start as usize] =
        (path_finder.state[path_finder.start as usize] as c_int | 0x2 as c_int) as u8;
}

#[no_mangle]
pub extern "C" fn path_finder_find_step(path_finder: &mut PathFinder, data: *mut c_void) -> u8 {
    let mut run: u8 = 0;
    let mut current: i32 = 0;
    let mut count: i32 = 0;
    run = 1 as c_int as u8;
    current = 0 as c_int;
    count = path_finder.cols * path_finder.rows;
    current = path_finder_lowest_in_open_set(path_finder);
    if current == path_finder.end {
        path_finder_reconstruct_path(path_finder);
        run = 0 as c_int as u8;
        path_finder.has_path = 1 as c_int as u8
    } else if path_finder_open_set_is_empty(path_finder) as c_int == 1 as c_int {
        run = 0 as c_int as u8;
        path_finder.has_path = 0 as c_int as u8
    } else {
        let mut neighbors: [i32; 4] = [0; 4];
        let mut j: i32 = 0;
        let mut tmp_g_score: i32 = 0;
        path_finder.state[current as usize] =
            (path_finder.state[current as usize] as c_int & !(0x2 as c_int)) as u8;
        path_finder.state[current as usize] =
            (path_finder.state[current as usize] as c_int | 0x4 as c_int) as u8;
        /* Left */
        if current % path_finder.cols == 0 as c_int {
            neighbors[0 as c_int as usize] = -(1 as c_int)
        } else {
            neighbors[0 as c_int as usize] = current - 1 as c_int
        }
        /* Top */
        neighbors[1 as c_int as usize] = current - path_finder.cols;
        /* Right */
        if (current + 1 as c_int) % path_finder.cols == 0 as c_int {
            neighbors[2 as c_int as usize] = -(1 as c_int)
        } else {
            neighbors[2 as c_int as usize] = current + 1 as c_int
        }
        /* Bottom */
        neighbors[3 as c_int as usize] = current + path_finder.cols;
        /* Neighbors */
        tmp_g_score = 0 as c_int;
        j = 0 as c_int;
        while j < 4 as c_int {
            let mut n: i32 = 0;
            n = neighbors[j as usize];
            if n > -(1 as c_int)
                && n < count
                && path_finder.state[n as usize] as c_int & 0x4 as c_int == 0 as c_int
            {
                if path_finder.state[n as usize] as c_int & 0x1 as c_int == 0 as c_int {
                    path_finder.state[n as usize] =
                        (path_finder.state[n as usize] as c_int | 0x4 as c_int) as u8
                } else {
                    tmp_g_score = path_finder.g_score[current as usize] + 1 as c_int;
                    if path_finder.state[n as usize] as c_int & 0x2 as c_int == 0 as c_int
                        || tmp_g_score < path_finder.g_score[n as usize]
                    {
                        path_finder.parents[n as usize] = current;
                        path_finder.g_score[n as usize] = tmp_g_score;
                        path_finder.f_score[n as usize] =
                            tmp_g_score + path_finder_heuristic(path_finder, n);
                        if path_finder.score_func.is_some() {
                            path_finder.f_score[n as usize] +=
                                path_finder.score_func.expect("non-null function pointer")(
                                    path_finder,
                                    n / path_finder.cols,
                                    n % path_finder.cols,
                                    data,
                                )
                        }
                        path_finder.state[n as usize] =
                            (path_finder.state[n as usize] as c_int | 0x2 as c_int) as u8
                    }
                }
            }
            j += 1
        }
    }
    run
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
}
