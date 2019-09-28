COMPILER = cargo
ARGS = build

all: debug release doc

debug: src/*
	$(COMPILER) $(ARGS)

release: src/*
	$(COMPILER) $(ARGS) --release

doc: Doc/*
	./Doc/buildDocument.sh