#![allow(non_camel_case_types, unused_assignments)]

use ::libc;
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct path_finder {
    pub cols: int32_t,
    pub rows: int32_t,
    pub start: int32_t,
    pub end: int32_t,
    pub has_path: uint8_t,
    pub state: [uint8_t; 1024],
    pub parents: [int32_t; 1024],
    pub g_score: [int32_t; 1024],
    pub f_score: [int32_t; 1024],
    pub fill_func:
        Option<unsafe extern "C" fn(_: *mut path_finder, _: int32_t, _: int32_t) -> uint8_t>,
    pub score_func: Option<
        unsafe extern "C" fn(
            _: *mut path_finder,
            _: int32_t,
            _: int32_t,
            _: *mut libc::c_void,
        ) -> int32_t,
    >,
    pub data: *mut libc::c_void,
}
unsafe extern "C" fn path_finder_heuristic(
    path_finder: *mut path_finder,
    cell: int32_t,
) -> int32_t {
    let mut cell_y: int32_t = 0;
    let mut cell_x: int32_t = 0;
    let mut end_y: int32_t = 0;
    let mut end_x: int32_t = 0;
    let mut dx: int32_t = 0;
    let mut dy: int32_t = 0;
    cell_y = cell / (*path_finder).cols;
    cell_x = cell - cell_y * (*path_finder).cols;
    end_y = (*path_finder).end / (*path_finder).cols;
    end_x = (*path_finder).end - end_y * (*path_finder).cols;
    if cell_x > end_x {
        dx = cell_x - end_x
    } else {
        dx = end_x - cell_x
    }
    if cell_y > end_y {
        dy = cell_y - end_y
    } else {
        dy = end_y - cell_y
    }
    return dx + dy;
}
unsafe extern "C" fn path_finder_open_set_is_empty(path_finder: *mut path_finder) -> uint8_t {
    let mut empty: uint8_t = 0;
    let mut i: int32_t = 0;
    empty = 1 as libc::c_int as uint8_t;
    i = 0 as libc::c_int;
    while i < (*path_finder).cols * (*path_finder).rows && empty as libc::c_int == 1 as libc::c_int
    {
        if (*path_finder).state[i as usize] as libc::c_int & 0x2 as libc::c_int
            == 0x2 as libc::c_int
        {
            empty = 0 as libc::c_int as uint8_t
        }
        i = i + 1 as libc::c_int
    }
    return empty;
}
unsafe extern "C" fn path_finder_lowest_in_open_set(path_finder: *mut path_finder) -> int32_t {
    let mut lowest_f: int32_t = 0;
    let mut current_lowest: int32_t = 0;
    let mut count: int32_t = 0;
    let mut i: int32_t = 0;
    count = (*path_finder).cols * (*path_finder).rows;
    lowest_f = count;
    current_lowest = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < count {
        if (*path_finder).state[i as usize] as libc::c_int & 0x2 as libc::c_int
            == 0x2 as libc::c_int
        {
            if (*path_finder).f_score[i as usize] < lowest_f {
                lowest_f = (*path_finder).f_score[i as usize];
                current_lowest = i
            }
        }
        i = i + 1 as libc::c_int
    }
    return current_lowest;
}
unsafe extern "C" fn path_finder_reconstruct_path(mut path_finder: *mut path_finder) {
    let mut i: int32_t = 0;
    i = (*path_finder).end;
    while i != (*path_finder).start {
        if (*path_finder).parents[i as usize] != (*path_finder).start {
            (*path_finder).state[(*path_finder).parents[i as usize] as usize] =
                ((*path_finder).state[(*path_finder).parents[i as usize] as usize] as libc::c_int
                    | 0x8 as libc::c_int) as uint8_t
        }
        i = (*path_finder).parents[i as usize]
    }
}
#[no_mangle]
pub unsafe extern "C" fn path_finder_fill(mut path_finder: *mut path_finder) {
    let mut row: int32_t = 0;
    row = 0 as libc::c_int;
    while row < (*path_finder).rows {
        let mut col: int32_t = 0;
        col = 0 as libc::c_int;
        while col < (*path_finder).cols {
            if (*path_finder).fill_func.expect("non-null function pointer")(path_finder, col, row)
                as libc::c_int
                == 1 as libc::c_int
            {
                (*path_finder).state[(row * (*path_finder).cols + col) as usize] =
                    ((*path_finder).state[(row * (*path_finder).cols + col) as usize]
                        as libc::c_int
                        | 0x1 as libc::c_int) as uint8_t
            } else {
                (*path_finder).state[(row * (*path_finder).cols + col) as usize] =
                    ((*path_finder).state[(row * (*path_finder).cols + col) as usize]
                        as libc::c_int
                        & !(0x1 as libc::c_int)) as uint8_t
            }
            col = col + 1 as libc::c_int
        }
        row = row + 1 as libc::c_int
    }
}
#[no_mangle]
pub unsafe extern "C" fn path_finder_begin(mut path_finder: *mut path_finder) {
    (*path_finder).state[(*path_finder).start as usize] =
        ((*path_finder).state[(*path_finder).start as usize] as libc::c_int | 0x2 as libc::c_int)
            as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn path_finder_find_step(
    mut path_finder: *mut path_finder,
    data: *mut libc::c_void,
) -> uint8_t {
    let mut run: uint8_t = 0;
    let mut current: int32_t = 0;
    let mut count: int32_t = 0;
    run = 1 as libc::c_int as uint8_t;
    current = 0 as libc::c_int;
    count = (*path_finder).cols * (*path_finder).rows;
    current = path_finder_lowest_in_open_set(path_finder);
    if current == (*path_finder).end {
        path_finder_reconstruct_path(path_finder);
        run = 0 as libc::c_int as uint8_t;
        (*path_finder).has_path = 1 as libc::c_int as uint8_t
    } else if path_finder_open_set_is_empty(path_finder) as libc::c_int == 1 as libc::c_int {
        run = 0 as libc::c_int as uint8_t;
        (*path_finder).has_path = 0 as libc::c_int as uint8_t
    } else {
        let mut neighbors: [int32_t; 4] = [0; 4];
        let mut j: int32_t = 0;
        let mut tmp_g_score: int32_t = 0;
        (*path_finder).state[current as usize] = ((*path_finder).state[current as usize]
            as libc::c_int
            & !(0x2 as libc::c_int)) as uint8_t;
        (*path_finder).state[current as usize] =
            ((*path_finder).state[current as usize] as libc::c_int | 0x4 as libc::c_int) as uint8_t;
        /* Left */
        if current % (*path_finder).cols == 0 as libc::c_int {
            neighbors[0 as libc::c_int as usize] = -(1 as libc::c_int)
        } else {
            neighbors[0 as libc::c_int as usize] = current - 1 as libc::c_int
        }
        /* Top */
        neighbors[1 as libc::c_int as usize] = current - (*path_finder).cols;
        /* Right */
        if (current + 1 as libc::c_int) % (*path_finder).cols == 0 as libc::c_int {
            neighbors[2 as libc::c_int as usize] = -(1 as libc::c_int)
        } else {
            neighbors[2 as libc::c_int as usize] = current + 1 as libc::c_int
        }
        /* Bottom */
        neighbors[3 as libc::c_int as usize] = current + (*path_finder).cols;
        /* Neighbors */
        tmp_g_score = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while j < 4 as libc::c_int {
            let mut n: int32_t = 0;
            n = neighbors[j as usize];
            if n > -(1 as libc::c_int)
                && n < count
                && (*path_finder).state[n as usize] as libc::c_int & 0x4 as libc::c_int
                    == 0 as libc::c_int
            {
                if (*path_finder).state[n as usize] as libc::c_int & 0x1 as libc::c_int
                    == 0 as libc::c_int
                {
                    (*path_finder).state[n as usize] =
                        ((*path_finder).state[n as usize] as libc::c_int | 0x4 as libc::c_int)
                            as uint8_t
                } else {
                    tmp_g_score = (*path_finder).g_score[current as usize] + 1 as libc::c_int;
                    if (*path_finder).state[n as usize] as libc::c_int & 0x2 as libc::c_int
                        == 0 as libc::c_int
                        || tmp_g_score < (*path_finder).g_score[n as usize]
                    {
                        (*path_finder).parents[n as usize] = current;
                        (*path_finder).g_score[n as usize] = tmp_g_score;
                        (*path_finder).f_score[n as usize] =
                            tmp_g_score + path_finder_heuristic(path_finder, n);
                        if (*path_finder).score_func.is_some() {
                            (*path_finder).f_score[n as usize] = (*path_finder).f_score[n as usize]
                                + (*path_finder)
                                    .score_func
                                    .expect("non-null function pointer")(
                                    path_finder,
                                    n / (*path_finder).cols,
                                    n % (*path_finder).cols,
                                    data,
                                )
                        }
                        (*path_finder).state[n as usize] =
                            ((*path_finder).state[n as usize] as libc::c_int | 0x2 as libc::c_int)
                                as uint8_t
                    }
                }
            }
            j = j + 1 as libc::c_int
        }
    }
    return run;
}
/*
Copyright (C) 2017 Felipe Ferreira da Silva

This software is provided 'as-is', without any express or implied warranty. In
no event will the authors be held liable for any damages arising from the use of
this software.

Permission is granted to anyone to use this software for any purpose, including
commercial applications, and to alter it and redistribute it freely, subject to
the following restrictions:

  1. The origin of this software must not be misrepresented; you must not claim
     that you wrote the original software. If you use this software in a
     product, an acknowledgment in the product documentation would be
     appreciated but is not required.
  2. Altered source versions must be plainly marked as such, and must not be
     misrepresented as being the original software.
  3. This notice may not be removed or altered from any source distribution.
*/
/* Bit flags */
#[no_mangle]
pub unsafe extern "C" fn path_finder_find(path_finder: *mut path_finder, data: *mut libc::c_void) {
    path_finder_begin(path_finder);
    while path_finder_find_step(path_finder, data) as libc::c_int == 1 as libc::c_int {}
}
#[no_mangle]
pub unsafe extern "C" fn path_finder_get_heuristic_score(
    path_finder: *mut path_finder,
    col: int32_t,
    row: int32_t,
) -> int32_t {
    return (*path_finder).f_score[(row * (*path_finder).cols + col) as usize];
}
#[no_mangle]
pub unsafe extern "C" fn path_finder_is_passable(
    path_finder: *mut path_finder,
    col: int32_t,
    row: int32_t,
) -> uint8_t {
    return ((*path_finder).state[(row * (*path_finder).cols + col) as usize] as libc::c_int
        & 0x1 as libc::c_int
        == 0x1 as libc::c_int) as libc::c_int as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn path_finder_is_closed(
    path_finder: *mut path_finder,
    col: int32_t,
    row: int32_t,
) -> uint8_t {
    return ((*path_finder).state[(row * (*path_finder).cols + col) as usize] as libc::c_int
        & 0x4 as libc::c_int
        == 0x4 as libc::c_int) as libc::c_int as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn path_finder_is_open(
    path_finder: *mut path_finder,
    col: int32_t,
    row: int32_t,
) -> uint8_t {
    return ((*path_finder).state[(row * (*path_finder).cols + col) as usize] as libc::c_int
        & 0x2 as libc::c_int
        == 0x2 as libc::c_int) as libc::c_int as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn path_finder_is_path(
    path_finder: *mut path_finder,
    col: int32_t,
    row: int32_t,
) -> uint8_t {
    return ((*path_finder).state[(row * (*path_finder).cols + col) as usize] as libc::c_int
        & 0x8 as libc::c_int
        == 0x8 as libc::c_int) as libc::c_int as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn path_finder_is_start(
    path_finder: *mut path_finder,
    col: int32_t,
    row: int32_t,
) -> uint8_t {
    return (row * (*path_finder).cols + col == (*path_finder).start) as libc::c_int as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn path_finder_is_end(
    path_finder: *mut path_finder,
    col: int32_t,
    row: int32_t,
) -> uint8_t {
    return (row * (*path_finder).cols + col == (*path_finder).end) as libc::c_int as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn path_finder_set_start(
    mut path_finder: *mut path_finder,
    col: int32_t,
    row: int32_t,
) {
    (*path_finder).start = row * (*path_finder).cols + col;
}
#[no_mangle]
pub unsafe extern "C" fn path_finder_set_end(
    mut path_finder: *mut path_finder,
    col: int32_t,
    row: int32_t,
) {
    (*path_finder).end = row * (*path_finder).cols + col;
}
#[no_mangle]
pub unsafe extern "C" fn path_finder_clear_path(mut path_finder: *mut path_finder) {
    let mut i: int32_t = 0;
    i = 0 as libc::c_int;
    while i < 1024 as libc::c_int {
        (*path_finder).state[i as usize] = ((*path_finder).state[i as usize] as libc::c_int
            & !(0x2 as libc::c_int | 0x4 as libc::c_int | 0x8 as libc::c_int))
            as uint8_t;
        (*path_finder).parents[i as usize] = 0 as libc::c_int;
        (*path_finder).g_score[i as usize] = 0 as libc::c_int;
        (*path_finder).f_score[i as usize] = 0 as libc::c_int;
        i = i + 1 as libc::c_int
    }
    (*path_finder).has_path = 0 as libc::c_int as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn path_finder_initialize(mut path_finder: *mut path_finder) {
    let mut i: int32_t = 0;
    i = 0 as libc::c_int;
    while i < 1024 as libc::c_int {
        (*path_finder).parents[i as usize] = 0 as libc::c_int;
        (*path_finder).g_score[i as usize] = 0 as libc::c_int;
        (*path_finder).f_score[i as usize] = 0 as libc::c_int;
        (*path_finder).state[i as usize] = 0x1 as libc::c_int as uint8_t;
        i = i + 1 as libc::c_int
    }
    (*path_finder).rows = 0 as libc::c_int;
    (*path_finder).cols = 0 as libc::c_int;
    (*path_finder).start = 0 as libc::c_int;
    (*path_finder).end = 0 as libc::c_int;
    (*path_finder).has_path = 0 as libc::c_int as uint8_t;
}
