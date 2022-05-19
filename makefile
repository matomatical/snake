snake: snake.nim
	nim compile snake.nim

run: snake
	./snake

clean:
	rm -f ./snake

.PHONY: run
