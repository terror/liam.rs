gen:
	./gen.sh

dev:
	open -a "Google Chrome" ./docs/index.html

typo:
	typos

fix-typo:
	typos --write-changes
