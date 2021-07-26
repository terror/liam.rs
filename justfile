gen:
	./gen.sh

dev:
	open -a 'Google Chrome' ./docs/index.html

typo:
	typos

fix-typo:
	typos --write-changes

publish name dir='posts':
	mv ./drafts/{{name}}.md ./{{dir}}
	./gen.sh
	@echo Done!
