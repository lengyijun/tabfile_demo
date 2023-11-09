all: cs 

cs:
	cargo run --bin cs_stardict
	tabfile cs.tab
	sudo mkdir -p /usr/share/stardict/dic/cs-dict/
	sudo cp cs* /usr/share/stardict/dic/cs-dict/

