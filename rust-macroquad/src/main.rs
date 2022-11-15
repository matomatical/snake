use std::collections::VecDeque;
use macroquad::prelude::*;
use fastrand;

// GRID SIZE
// size of one grid square in pixels  (geq 1)
const UNIT: i32 = 24;
// number of grid squares across grid (geq 1)
const GRID: i32 = 16;
// radius of grid in pixels
// (from the centre of the grid to the centre of a border square)
const RAD: f32  = ((GRID-1)*UNIT) as f32 / 2.;
// minimum sreen width / height
const MIN_SCREEN: f32 = (UNIT * GRID) as f32;

// TIMING
const STEPS_PER_SECOND: i32 = 8;
const SECONDS_PER_STEP: f64 = 1. / STEPS_PER_SECOND as f64;

// GAME STATE
type Point = (i32, i32); // todo: vector library
const UP   : Point = (0, -1);
const DOWN : Point = (0,  1);
const LEFT : Point = (-1, 0);
const RIGHT: Point = ( 1, 0);
struct Snake {
    head: Point,
    body: VecDeque<Point>,
    dnow: Point,
    dque: VecDeque<Point>,
}
struct State {
    snake: Snake,
    apple: Point,
    score: i32,
}

#[macroquad::main("Snake")]
async fn main() {
    let mut past = get_time() - SECONDS_PER_STEP - 1.;
    let mut state = init();
    loop {
        // sample input as much as possible
        let exit = input(&mut state);
        //println!("{:?}", state.snake.dque.make_contiguous());
        if exit { break; }
        // update only on logical frames
        let time = get_time();
        if time - past > SECONDS_PER_STEP {
            past = time;
            let over = update(&mut state);
            if over { break; }
        }
        render(&state);
        next_frame().await
    }
    println!("score: {}", state.score);
}

fn init() -> State {
    State {
        snake:
            Snake {
                head: (GRID / 2, GRID / 2),
                //body: VecDeque::from([(GRID / 2, GRID / 2)]),
                body: VecDeque::from([(0,1), (2,1), (3,4)]),
                dnow: LEFT,
                dque: VecDeque::new(),
            },
        apple: (GRID / 4, 3 * GRID / 4),
        score: 0,
    } // return;
}

fn render(state: &State) {
    clear_background(BLACK);
    for i in 0..GRID { for j in 0..GRID { draw_point((i,j), LIGHTGRAY); } }

    for p in &state.snake.body { draw_point(*p, DARKGREEN); }
    draw_point(state.snake.head, GREEN);
    
    draw_point(state.apple, RED);
    
    // TODO draw the score?
}

fn input(state: &mut State) -> bool {
    // movement
    if is_key_down(KeyCode::Right) {
        state.snake.dque.push_back(RIGHT);
    } else if is_key_down(KeyCode::Left) {
        state.snake.dque.push_back(LEFT);
    } else if is_key_down(KeyCode::Up) {
        state.snake.dque.push_back(UP);
    } else if is_key_down(KeyCode::Down) {
        state.snake.dque.push_back(DOWN);
    }
    // exit by pressing Q?
    return is_key_down(KeyCode::Q);
}

fn update(state: &mut State) -> bool {
    // process one valid instruction from the queue
    while ! state.snake.dque.is_empty() {
        let d = state.snake.dque.pop_front().unwrap(); // see loop guard
        if d != state.snake.dnow && (-d.0, -d.1) != state.snake.dnow {
            state.snake.dnow = d;
            break;
        }
    }
    // move head forward in the current direction, with wraparound
    state.snake.head = (
        wrap(state.snake.head.0 + state.snake.dnow.0, GRID),
        wrap(state.snake.head.1 + state.snake.dnow.1, GRID),
    );
    // move body along, remembering tail in case we need to replace it too,
    // and check for snake collisions
    let tail = state.snake.body.pop_back().unwrap(); // never empty
    if state.snake.body.contains(&state.snake.head) {
        return true;
    }
    // after the collision check, it's safe to add the snake head back to
    // the body (and we have to do it now for apple spawning)
    state.snake.body.push_front(state.snake.head);
    // check for collision with apples
    if state.snake.head == state.apple {
        state.score += 1;
        // grow snake
        state.snake.body.push_back(tail);
        // respawn apple
        while state.snake.body.contains(&state.apple) {
            state.apple = (
                wrap(fastrand::i32(..), GRID),
                wrap(fastrand::i32(..), GRID),
            );
        }
    }
    false // game not over
}

fn wrap(i: i32, n: i32) -> i32 {
    ((i % n) + n) % n
}

// my integer-based drawing library?
fn draw_point(p: Point, col: Color) {
    let (x, y) = p;
    let px_size = UNIT as f32;
    let px_right= screen_width()/2. - RAD + (x*UNIT) as f32 - px_size / 2.;
    let px_down = screen_height()/2.- RAD + (y*UNIT) as f32 - px_size / 2.;
    draw_rectangle(
        px_right, px_down, px_size, px_size, col,
    )
}

