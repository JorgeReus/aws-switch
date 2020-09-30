ifndef $(GOPATH)
    export GOPATH=$(shell go env GOPATH)
endif

ifndef $(GOROOT)
    export GOROOT=$(shell go env GOROOT)
endif

$(mkdir bin)

GOLINUX := GOOS=linux GOARCH=amd64 GOBIN=$(PWD)/bin
GOWIN := GOOS=windows GOARCH=amd64

.PHONY: clean install

install:
	cd src; $(GOLINUX) go install; $(GOWIN) go install

clean:
	rm -rf bin
