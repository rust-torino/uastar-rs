use once_cell::sync::{Lazy, OnceCell};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::{
    cell::{Cell, RefCell},
    error,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    ptr::null_mut,
    sync::Mutex,
    thread::sleep,
    time::Duration,
};
use uastar::*;

static PASSABLE_CHANCE: Lazy<Mutex<Cell<u32>>> = Lazy::new(|| Mutex::new(Cell::new(0)));

static RNG: OnceCell<Mutex<RefCell<SmallRng>>> = OnceCell::new();

fn fill_cb(_path_finder: &mut PathFinder, _col: i32, _row: i32) -> u8 {
    let mut is_passable = 0u8;
    /* Fill the map randomly with passable cells */
    let rand_value = RNG.get().unwrap().lock().unwrap().borrow_mut().gen::<i32>();

    if rand_value as f64 / 2_147_483_647_f64
        <= PASSABLE_CHANCE.lock().unwrap().get() as f64 / 100.0f64
    {
        is_passable = 1u8
    }

    is_passable
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Pos {
    col: i32,
    row: i32,
}

impl Pos {
    fn new(col: i32, row: i32) -> Pos {
        Pos { col, row }
    }
}

fn find_path(
    show_progress: u8,
    chance: u32,
    seed: u32,
    start: Pos,
    end: Pos,
    width: i32,
    height: i32,
) -> PathFinder {
    let mut path_finder = PathFinder {
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
        data: null_mut(),
    };

    PASSABLE_CHANCE.lock().unwrap().set(chance);
    RNG.set(Mutex::new(RefCell::new(SmallRng::seed_from_u64(
        seed as u64,
    ))))
    .unwrap();
    if width < 1 || height < 1 || height * width > 1024 {
        println!("Failed due width or height smaller than 1 or the number of cells (width * height) is larger than 1024.");
    } else if start.col < 0
        || start.col > width - 1
        || end.col < 0
        || end.col > width - 1
        || start.row < 0
        || start.row > height - 1
        || end.row < 0
        || end.row > height - 1
    {
        println!("Invalid coordinates of start or end.");
    } else {
        path_finder_initialize(&mut path_finder);
        path_finder.cols = width;
        path_finder.rows = height;
        path_finder.fill_func = Some(fill_cb);
        path_finder.score_func = None;
        path_finder_fill(&mut path_finder);
        path_finder_set_start(&mut path_finder, start.col, start.row);
        path_finder_set_end(&mut path_finder, end.col, end.row);
        if show_progress == 0 {
            path_finder_find(&mut path_finder, null_mut());
        } else {
            path_finder_begin(&mut path_finder);
            while path_finder_find_step(&mut path_finder, null_mut()) == 1 {
                sleep(Duration::from_micros(25000))
            }
        }
    }

    path_finder
}

fn from_file(path: &Path) -> Result<PathFinder, Box<dyn error::Error>> {
    let file = BufReader::new(File::open(path)?);
    let mut lines = file.lines();

    let cols = lines.next().unwrap()?.parse()?;
    let rows = lines.next().unwrap()?.parse()?;
    let start = lines.next().unwrap()?.parse()?;
    let end = lines.next().unwrap()?.parse()?;
    let has_path = lines.next().unwrap()?.parse()?;
    let mut state = [0; 1024];
    let mut parents = [0; 1024];
    let mut g_score = [0; 1024];
    let mut f_score = [0; 1024];

    state
        .iter_mut()
        .zip(lines.by_ref().map(|line| line.unwrap().parse().unwrap()))
        .for_each(|(state, value)| *state = value);

    parents
        .iter_mut()
        .zip(lines.by_ref().map(|line| line.unwrap().parse().unwrap()))
        .for_each(|(state, value)| *state = value);

    g_score
        .iter_mut()
        .zip(lines.by_ref().map(|line| line.unwrap().parse().unwrap()))
        .for_each(|(state, value)| *state = value);

    f_score
        .iter_mut()
        .zip(lines.by_ref().map(|line| line.unwrap().parse().unwrap()))
        .for_each(|(state, value)| *state = value);

    Ok(PathFinder {
        cols,
        rows,
        start,
        end,
        has_path,
        state,
        parents,
        g_score,
        f_score,
        fill_func: None,
        score_func: None,
        data: null_mut(),
    })
}

#[test]
fn original() {
    const TEST_OUTPUT_PATH: &str = "tests/original_test_out.txt";

    let output = find_path(0, 80, 12345, Pos::new(0, 0), Pos::new(23, 11), 24, 13);
    let expected_output = from_file(Path::new(TEST_OUTPUT_PATH));
    if Some(output) != expected_output.ok() {
        panic!("path finder different from expected");
    }
}
