export GOBIN=$(PWD)/bin

ifndef $(GOPATH)
    export GOPATH=$(shell go env GOPATH)
endif

ifndef $(GOROOT)
    export GOROOT=$(shell go env GOROOT)
endif

$(mkdir bin)

.PHONY: clean build

build:
	cd src; go build -o ../bin


clean:
	rm -rf bin

