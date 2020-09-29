export GOBIN=$(PWD)/bin

ifndef $(GOPATH)
    export GOPATH=$(shell go env GOPATH)
endif

ifndef $(GOROOT)
    export GOROOT=$(shell go env GOROOT)
endif

build:
	@go build

