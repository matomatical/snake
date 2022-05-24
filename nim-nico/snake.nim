import std/deques
import std/random
import nico

import vec

const
    SCREEN_SIZE = 128
    GRID_SIZE = 16
    CELL_SIZE = SCREEN_SIZE div GRID_SIZE
    STEPS_PER_SECOND = 8
    SECONDS_PER_STEP = 1 / STEPS_PER_SECOND

type
    Snake = object
        head: vec8 # position
        body: Deque[vec8]
        dnow: vec8 # current direction
        dque: Deque[vec8]
    State = object
        gameover: bool
        snake: Snake
        apple: vec8
        score: int
        timer: float

# global state
var state: State

proc new_game() =
    state = State(
        gameover: false,
        snake: Snake(
            head: v8(GRID_SIZE div 2, GRID_SIZE div 2),
            body: toDeque([v8(GRID_SIZE div 2, GRID_SIZE div 2)]),
            dnow: v8_left,
            dque: initDeque[vec8](),
        ),
        apple: v8(GRID_SIZE div 4, 3 * GRID_SIZE div 4),
        score: 0,
        timer: 0,
    )
    
# PROCESS INPUT
proc control() =
    if state.gameover:
        if btnp(pcA):
            new_game()
    else:
        if btnp(pcUp):
            state.snake.dque.addLast(v8_up)
        if btnp(pcDown):
            state.snake.dque.addLast(v8_down)
        if btnp(pcLeft):
            state.snake.dque.addLast(v8_left)
        if btnp(pcRight):
            state.snake.dque.addLast(v8_right)

# UPDATE STATE EVERY LOGICAL FRAME
proc evolve() =
    if state.gameover:
        return
    # process one instruction from the queue
    while state.snake.dque.len > 0:
        let d = state.snake.dque.popFirst
        if d != state.snake.dnow and d != -state.snake.dnow:
            state.snake.dnow = d
            break
    # move head forward in the current direction, with wraparound
    state.snake.head += state.snake.dnow
    if state.snake.head.x < 0:
        state.snake.head.x += GRID_SIZE
    if state.snake.head.y < 0:
        state.snake.head.y += GRID_SIZE
    if state.snake.head.x >= GRID_SIZE:
        state.snake.head.x -= GRID_SIZE
    if state.snake.head.y >= GRID_SIZE:
        state.snake.head.y -= GRID_SIZE
    # move body along, remembering tail in case we need to replace it too,
    # and check for snake collisions
    let tail = state.snake.body.popLast
    if state.snake.head in state.snake.body:
        state.gameover = true
    # after the collision check, it's safe to add the snake head back to
    # the body (and we have to do it now for apple spawning)
    state.snake.body.addFirst(state.snake.head)
    # check for collision with apples
    if state.snake.head == state.apple:
        state.score += 1
        # grow snake
        state.snake.body.addLast(tail)
        # respawn apple
        while state.apple in state.snake.body:
            state.apple = v8(
                cast[int8](rand(GRID_SIZE-1)),
                cast[int8](rand(GRID_SIZE-1)),
            )

proc draw() =
    template blit(v: vec8) =
        boxfill(CELL_SIZE*v.x, CELL_SIZE*v.y, CELL_SIZE, CELL_SIZE)
    cls()
    # background
    setColor(0)
    boxfill(0, 0, screenWidth, screenHeight)
    # apple
    setColor(8)
    blit(state.apple)
    # snake (body)
    setColor(7)
    for pos in state.snake.body.items:
        blit(pos)
    # snake (head)
    if state.gameover: setColor(8)
    blit(state.snake.head)
    # score
    printc($state.score, screenWidth div 2, 0)

proc init() =
    loadFont(0, "font.png")
    setFont(0)
    new_game()

proc update(dt: Pfloat) =
    control()
    state.timer += dt
    if state.timer > SECONDS_PER_STEP:
        evolve()
        state.timer -= SECONDS_PER_STEP

when isMainModule:
    randomize()
    nico.init("far", "snake")
    fixedSize(true)
    integerScale(true)
    nico.createWindow("snake", SCREEN_SIZE, SCREEN_SIZE, 4)
    nico.run(init, update, draw)
