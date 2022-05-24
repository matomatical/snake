snake.dev: snake.nim vec.nim
	nim compile -o:snake.dev snake.nim

snake: snake.nim vec.nim
	nim compile -d:release 	 snake.nim

snake.dev.html: snake.nim vec.nim
	nim compile -d:debug -d:emscripten -o:snake.dev.html snake.nim

snake.html: snake.nim vec.nim
	nim compile -d:danger -d:emscripten -o:snake.html snake.nim

run: snake.dev
	./snake.dev

clean:
	rm -f ./snake ./snake.dev

.PHONY: run clean
