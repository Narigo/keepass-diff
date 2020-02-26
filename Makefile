.PHONY: alias
alias:
	@echo alias keepass-diff=\'docker run -it --rm -w \`pwd\` -v /home/${USER}:/home/${USER} keepass-diff:latest keepass-diff\'

.PHONY: all
all: build shell

.PHONY: build
build:
	docker build -t keepass-diff:latest .

.PHONY: run
run:
	docker run -it --rm \
		-w `pwd` \
		-v /home/${USER}:/home/${USER} \
		keepass-diff:latest keepass-diff --help # <<-- Add arguments here

.PHONY: shell
shell:
	docker run -it --rm \
		-w `pwd` \
		-v /home/${USER}:/home/${USER} \
		keepass-diff:latest bash

.PHONY: test
test:
	docker run -it --rm \
		-w `pwd` \
		-v /home/${USER}:/home/${USER} \
		keepass-diff:latest keepass-diff \
			test/test.kdbx test/test2.kdbx --passwords demopass

