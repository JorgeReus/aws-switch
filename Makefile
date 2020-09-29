export PATH := $(PWD)/bin:$(PATH)

build:
	@mkdir -p bin/
	@go build main.go -o bin
