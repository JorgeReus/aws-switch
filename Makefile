export GOBIN=$(PWD)/bin

ifndef $(GOPATH)
    export GOPATH=$(shell go env GOPATH)
endif

ifndef $(GOROOT)
    export GOROOT=$(shell go env GOROOT)
endif

$(mkdir bin)

.PHONY: clean install

install:
	cd src; go install

clean:
	rm -rf bin

