# make test                     run the tests
# make install                  install for yourself, into ~/.local
# sudo make install PREFIX=/usr/local   install for everyone
PREFIX ?= $(HOME)/.local
LIBDIR = $(PREFIX)/lib/tix
BINDIR = $(PREFIX)/bin
COMPDIR = $(PREFIX)/share/bash-completion/completions

.PHONY: test install uninstall lint

test:
	bash tests.sh

lint:
	shellcheck tix tixlib.sh tests.sh tix-completion.bash

install:
	install -d $(LIBDIR) $(BINDIR) $(COMPDIR)
	install -m 755 tix $(LIBDIR)/tix
	install -m 644 tixlib.sh $(LIBDIR)/tixlib.sh
	ln -sf $(LIBDIR)/tix $(BINDIR)/tix
	install -m 644 tix-completion.bash $(COMPDIR)/tix

uninstall:
	rm -rf $(LIBDIR) $(BINDIR)/tix $(COMPDIR)/tix
