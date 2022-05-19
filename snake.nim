import os
import std/deques
import std/random

import illwill

import vec
    

# CONFIG

const
    GAME_GRID_SIZE          = 16
    GAME_FRAMES_PER_STEP    = 5

# MODEL

type
    GameState = enum
        Gameplay, Gameover
    Snake = object
        head: vec8 # position
        body: Deque[vec8]
        dnow: vec8 # current direction
        dque: Deque[vec8]
    Apple = object
        pos: vec8
    Game = object
        snake: Snake
        apple: Apple
        score: uint8
        state: GameState
        message: string

proc model_init(): Game =
    result = Game(
        snake: Snake(
            head: v8(8, 8),
            body: toDeque([v8(8, 8)]),
            dnow: v8_left,
            dque: initDeque[vec8](),
        ),
        apple: Apple(pos: v8(4, 12)),
        score: 0,
        state: Gameplay,
    )

proc control(model: var Game, key: Key) =
    let dnew : vec8 = case key
    of Key.Up, Key.W:       v8_up
    of Key.Down, Key.S:     v8_down
    of Key.Left, Key.A:     v8_left
    of Key.Right, Key.D:    v8_right
    else:                   v8_zero
    if dnew != v8_zero:
        model.snake.dque.addLast(dnew)
    
proc evolve(model: var Game, frame: int) =
    if frame mod GAME_FRAMES_PER_STEP != 0:
        return
    # process one instruction from the queue
    while model.snake.dque.len > 0:
        let d = model.snake.dque.popFirst
        if d != model.snake.dnow and d != -model.snake.dnow:
            model.snake.dnow = d
            break
    # move head forward in the current direction, with wraparound
    model.snake.head += model.snake.dnow
    if model.snake.head.x < 0:
        model.snake.head.x += GAME_GRID_SIZE
    if model.snake.head.y < 0:
        model.snake.head.y += GAME_GRID_SIZE
    if model.snake.head.x >= GAME_GRID_SIZE:
        model.snake.head.x -= GAME_GRID_SIZE
    if model.snake.head.y >= GAME_GRID_SIZE:
        model.snake.head.y -= GAME_GRID_SIZE
    # move body along, remembering tail in case we need to replace it too,
    # and check for snake collisions
    let tail = model.snake.body.popLast
    if model.snake.head in model.snake.body:
        model.state = Gameover
    # after the collision check, it's safe to add the snake head back to
    # the body (and we have to do it now for apple spawning)
    model.snake.body.addFirst(model.snake.head)
    # check for collision with apples
    if model.snake.head == model.apple.pos:
        model.score += 1
        # grow snake
        model.snake.body.addLast(tail)
        # respawn apple
        while model.apple.pos in model.snake.body:
            model.apple.pos = v8(
                cast[int8](rand(GAME_GRID_SIZE-1)),
                cast[int8](rand(GAME_GRID_SIZE-1)),
            )
    
# VIEW

proc view_init(tb : var TerminalBuffer) =
    # draw the boundary around the box
    tb.setForegroundColor(fgBlack, true)
    tb.drawRect(0, 0, 33, 17)
    # draw the instructions
    tb.write( 0, GAME_GRID_SIZE+2,
        fgYellow, "WASD", fgWhite, " : turn")
    tb.write(GAME_GRID_SIZE*2 - 8, GAME_GRID_SIZE+2,
        fgYellow, "ESC", fgWhite, " : quit")

proc view_update(model: Game, tb: var TerminalBuffer) =
    template blit(v: vec8, cc: string) = tb.write(1+2*v.x, 1+v.y, cc)
    # clear screen
    tb.fill(1, 1, 32, 16)
    # apple
    tb.setForegroundColor(fgRed, true)
    blit(model.apple.pos, "@@")
    # snake (body)
    tb.setForegroundColor(fgWhite, true)
    for pos in model.snake.body.items:
        blit(pos, "██")
    # snake (head)
    if model.state == Gameover:
        tb.setForegroundColor(fgRed, true)
    blit(model.snake.head, "██")
    # ui update
    tb.write(GAME_GRID_SIZE, GAME_GRID_SIZE + 2,
        fgWhite, $model.score)
    if model.state == Gameover:
        tb.write( 0, GAME_GRID_SIZE+2,
                fgYellow, "GAME OVER   ")

proc main() =
    var tb = newTerminalBuffer(terminalWidth(), terminalHeight())
    # initialisation of model
    var frame : int
    var model : Game = model_init()
    # initialisation of view
    view_init(tb)
    var key = Key.None
    while key != Key.Escape and key != Key.Q:
        sleep(20)
        key = getKey()
        # game loop
        if model.state == Gameover:
            continue # (wait for quit signal)
        model.control(key)
        frame += 1
        model.evolve(frame)
        view_update(model, tb)
        tb.write(35, 0, fgWhite, "frame: ", fgGreen, $frame)
        tb.write(35, 1, fgWhite, "score: ", fgGreen, $model.score)
        tb.write(35, 2, fgWhite, "snake.head: ", fgGreen, $model.snake.head)
        tb.write(35, 3, fgWhite, "snake.dnow: ", fgGreen, $model.snake.dnow)
        tb.write(35, 4, fgWhite, "snake.dque: ", fgGreen, $model.snake.dque)
        tb.write(35, 5, fgWhite, "apple.pos:  ", fgGreen, $model.apple.pos)
        tb.write(35, 6, fgWhite, $model.message)
        tb.display()
        # game loop again!


when isMainModule:
    # initialise illwill
    proc exit_proc() {.noconv.} =
        illwillDeinit()
        showCursor()
        quit(0)
    setControlCHook(exit_proc)
    illwillInit(fullscreen=true)
    hideCursor()
    # initialize random number generator with new seed
    randomize()
    # initialise program
    main()
    # clean up illwill
    exit_proc()
