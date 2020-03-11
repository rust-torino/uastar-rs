#![allow(non_camel_case_types, unused_mut, unused_variables, unused_assignments)]

use libc;
use uastar::*;

extern "C" {
    #[no_mangle]
    fn rand() -> libc::c_int;
    #[no_mangle]
    fn srand(__seed: libc::c_uint);
    #[no_mangle]
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __useconds_t = libc::c_uint;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[no_mangle]
pub static mut passable_chance: uint32_t = 0;
unsafe extern "C" fn fill_cb(
    mut path_finder: *mut path_finder,
    mut col: int32_t,
    mut row: int32_t,
) -> uint8_t {
    let mut is_passable: uint8_t = 0;
    is_passable = 0 as libc::c_int as uint8_t;
    /* Fill the map randomly with passable cells */
    if rand() as libc::c_double / 2147483647 as libc::c_int as libc::c_double
        <= passable_chance as libc::c_double / 100.0f64
    {
        is_passable = 1 as libc::c_int as uint8_t
    }
    return is_passable;
}
unsafe extern "C" fn print_map(
    mut path_finder: *mut path_finder,
    mut print_open_and_closed: uint8_t,
) {
    let mut row: int32_t = 0;
    let mut col: int32_t = 0;
    printf(
        b"  Passable chance: %u\n\x00" as *const u8 as *const libc::c_char,
        passable_chance,
    );
    printf(
        b"            Start: \'%c\' (or \'%c\' if fall in a wall)\n\x00" as *const u8
            as *const libc::c_char,
        'S' as i32,
        's' as i32,
    );
    printf(
        b"              End: \'%c\' (or \'%c\' if fall in a wall)\n\x00" as *const u8
            as *const libc::c_char,
        'E' as i32,
        'e' as i32,
    );
    printf(
        b"        Open path: \'%c\'\n\x00" as *const u8 as *const libc::c_char,
        'O' as i32,
    );
    printf(
        b"      Closed path: \'%c\'\n\x00" as *const u8 as *const libc::c_char,
        'X' as i32,
    );
    printf(
        b"             Path: \'%c\'\n\x00" as *const u8 as *const libc::c_char,
        '*' as i32,
    );
    printf(
        b"       Unpassable: \'%c\'\n\x00" as *const u8 as *const libc::c_char,
        '#' as i32,
    );
    printf(b"Map:\n\x00" as *const u8 as *const libc::c_char);
    col = 0 as libc::c_int;
    while col < (*path_finder).cols + 2 as libc::c_int {
        printf(b"#\x00" as *const u8 as *const libc::c_char);
        col = col + 1 as libc::c_int
    }
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
    row = 0 as libc::c_int;
    while row < (*path_finder).rows {
        col = 0 as libc::c_int;
        printf(b"#\x00" as *const u8 as *const libc::c_char);
        while col < (*path_finder).cols {
            if path_finder_is_start(path_finder, col, row) as libc::c_int == 1 as libc::c_int {
                if path_finder_is_passable(path_finder, col, row) as libc::c_int == 1 as libc::c_int
                {
                    printf(b"%c\x00" as *const u8 as *const libc::c_char, 'S' as i32);
                } else {
                    printf(b"%c\x00" as *const u8 as *const libc::c_char, 's' as i32);
                }
            } else if path_finder_is_end(path_finder, col, row) as libc::c_int == 1 as libc::c_int {
                if path_finder_is_passable(path_finder, col, row) as libc::c_int == 1 as libc::c_int
                {
                    printf(b"%c\x00" as *const u8 as *const libc::c_char, 'E' as i32);
                } else {
                    printf(b"%c\x00" as *const u8 as *const libc::c_char, 'e' as i32);
                }
            } else if path_finder_is_passable(path_finder, col, row) as libc::c_int
                == 0 as libc::c_int
            {
                printf(b"%c\x00" as *const u8 as *const libc::c_char, '#' as i32);
            } else if path_finder_is_path(path_finder, col, row) as libc::c_int == 1 as libc::c_int
            {
                printf(b"%c\x00" as *const u8 as *const libc::c_char, '*' as i32);
            } else if print_open_and_closed as libc::c_int == 1 as libc::c_int
                && path_finder_is_open(path_finder, col, row) as libc::c_int == 1 as libc::c_int
                && path_finder_is_closed(path_finder, col, row) as libc::c_int == 0 as libc::c_int
            {
                printf(b"%c\x00" as *const u8 as *const libc::c_char, 'O' as i32);
            } else if print_open_and_closed as libc::c_int == 1 as libc::c_int
                && path_finder_is_closed(path_finder, col, row) as libc::c_int == 1 as libc::c_int
            {
                printf(b"%c\x00" as *const u8 as *const libc::c_char, 'X' as i32);
            } else {
                printf(b" \x00" as *const u8 as *const libc::c_char);
            }
            col += 1
        }
        printf(b"#\n\x00" as *const u8 as *const libc::c_char);
        row += 1
    }
    col = 0 as libc::c_int;
    while col < (*path_finder).cols + 2 as libc::c_int {
        printf(b"#\x00" as *const u8 as *const libc::c_char);
        col = col + 1 as libc::c_int
    }
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
    if (*path_finder).has_path != 0 {
        printf(b"A path was found!\n\n\x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"No path was found!\n\n\x00" as *const u8 as *const libc::c_char);
    };
}

unsafe fn find_path(
    show_progress: uint8_t,
    chance: uint32_t,
    seed: libc::c_uint,
    s_col: libc::c_int,
    s_row: libc::c_int,
    e_col: libc::c_int,
    e_row: libc::c_int,
    width: libc::c_int,
    height: libc::c_int,
) -> libc::c_int {
    let mut path_finder: path_finder = path_finder {
        cols: 0,
        rows: 0,
        start: 0,
        end: 0,
        has_path: 0,
        state: [0; 1024],
        parents: [0; 1024],
        g_score: [0; 1024],
        f_score: [0; 1024],
        fill_func: None,
        score_func: None,
        data: 0 as *mut libc::c_void,
    };

    passable_chance = chance;
    srand(seed);
    if width < 1 as libc::c_int || height < 1 as libc::c_int || height * width > 1024 as libc::c_int
    {
        printf(b"Failed due width or height smaller than 1 or the number of cells (width * height) is larger than %u.\n\x00"
                       as *const u8 as *const libc::c_char,
                   1024 as libc::c_int);
    } else if s_col < 0 as libc::c_int
        || s_col > width - 1 as libc::c_int
        || e_col < 0 as libc::c_int
        || e_col > width - 1 as libc::c_int
        || s_row < 0 as libc::c_int
        || s_row > height - 1 as libc::c_int
        || e_row < 0 as libc::c_int
        || e_row > height - 1 as libc::c_int
    {
        puts(b"Invalid coordinates of start or end.\x00" as *const u8 as *const libc::c_char);
    } else {
        path_finder_initialize(&mut path_finder);
        path_finder.cols = width;
        path_finder.rows = height;
        path_finder.fill_func = Some(
            fill_cb as unsafe extern "C" fn(_: *mut path_finder, _: int32_t, _: int32_t) -> uint8_t,
        );
        path_finder.score_func = None;
        path_finder_fill(&mut path_finder);
        path_finder_set_start(&mut path_finder, s_col, s_row);
        path_finder_set_end(&mut path_finder, e_col, e_row);
        if show_progress as libc::c_int == 0 as libc::c_int {
            path_finder_find(&mut path_finder, 0 as *mut libc::c_void);
        } else {
            path_finder_begin(&mut path_finder);
            while path_finder_find_step(&mut path_finder, 0 as *mut libc::c_void) as libc::c_int
                == 1 as libc::c_int
            {
                /* Print progress map */
                print_map(&mut path_finder, 1 as libc::c_int as uint8_t);
                usleep(25000 as libc::c_int as __useconds_t);
            }
        }
        /* Print final map */
        print_map(&mut path_finder, 0 as libc::c_int as uint8_t);
    }
    return 0 as libc::c_int;
}

#[test]
fn original() {
    assert_eq!(unsafe { find_path(0, 80, 12345, 0, 0, 23, 11, 24, 13) }, 0);
}
