
all: clean build

clean: 
	rm -f min_coins

build:
	rustc min_coins.rs -o min_coins
