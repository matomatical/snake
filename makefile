snake.dev: snake.nim
	nim compile -o:snake.dev snake.nim

snake: snake.nim
	nim compile -d:release 	 snake.nim

run: snake.dev
	./snake.dev

clean:
	rm -f ./snake ./snake.dev

.PHONY: run clean
