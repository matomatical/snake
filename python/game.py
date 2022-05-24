"""
Idea for AI game: Two player snake. Don't hit the other person. Race to
collect apples. Strict time limit on making moves, but it's cumulative
rather than per-turn. If both players collide at once, then the player with
more apples wins, but otherwise even a small snake can beat a larger one...

Or perhaps the collision cuts the snake, leaving the tail behind, and growing
the eating snake by one? Nah, probably not...
"""

import util
import time
import random

RANGE = range(0, 16)

class State:
    EMPTY = ' '
    APPLE = '@'
    SNAKE = 'O'

class Board:
    def __init__(self):
        self.grid = {(x, y): State.EMPTY for x in RANGE for y in RANGE}
    def __setitem__(self, key, value):
        self.grid[key] = value
    def __getitem__(self, key, value):
        return self.grid[key]
    def __delitem__(self, key):
        self.grid[key] = State.EMPTY
    def __contains__(self, key):
        return key in self.grid
    def __str__(self):
        return (
          "+-" + "--" * len(RANGE) + "+\n| "
          + " |\n| ".join(
              " ".join(self.grid[x, y] for x in RANGE) for y in RANGE
            )
          + " |\n+-" + "--" * len(RANGE) + "+"
        )

class Apple:
    def __init__(self, position):
        self.pos = position

class Snake:
    def __init__(self, positions):
        self.head = positions[0]
        self.chain = util.deque(positions)
    def move1(self, step):
        new_head = (self.head[0]+step[0], self.head[1]+step[1]) # TODO vec
        self.head = new_head
        self.chain.push(new_head)
        return new_head
    def move2(self, eat=False):
        if not eat:
            return self.chain.pull()
    def __contains__(self, position):
        return (position in self.chain)

class Game:
    def __init__(self):
        self.board = Board()
        self.spawn_snake()
        self.spawn_apple()
        self.score = 0

    def spawn_snake(self):
        self.snake = Snake([(8, 8)])
        self.board[8, 8] = State.SNAKE
    
    def spawn_apple(self):
        pos = self.snake.head
        while pos in self.snake:
            pos = tuple(random.sample(RANGE, 2))
        self.apple = Apple(pos)
        self.board[pos] = State.APPLE

    def step(self, step):
        new_head = self.snake.move1(step)
        eat = (self.apple.pos == new_head)
        if eat:
            self.score += 1
            self.snake.move2(eat=True)
            # TODO: check end of game
            self.spawn_apple()
        else:
            old_tail = self.snake.move2(eat=False)
            self.board[old_tail] = State.EMPTY
        self.board[new_head] = State.SNAKE
        # CHECK FOR COLLISIONS WITH SELF AND WALLS

    def render(self):
        print("board:")
        print(self.board)
        print("score:", self.score)

KEYSTEPS = {
    'w': (0, -1),
    'a': (-1, 0),
    's': (0, +1),
    'd': (+1, 0),
}

def main():
    game = Game()
    while True:
        game.render()
        ds = input("wasd? ")
        for d in ds:
            if d == 'q': break
            step = KEYSTEPS[d]
            game.step(step)
        if d == 'q': break
    print('bye')

if __name__ == "__main__":
    main()
