---
title: textmap
date: 2023-04-10
repo: terror/textmap
topics: ['Python', 'Spacy', 'NLP']
lead: A heatmap-like visualization for text-based content.
image: textmap.png
---

This project was an experiment to see what it would be like to introduce
similarity-based highlighting to text interfaces.

It works by chunking the input text into sentence vectors using
[Spacy](https://github.com/explosion/spaCy) and then ranking them based on their
cosine similarity to other sentence vectors within the same body of text -
giving them a score that falls within a range which can easily be mapped to a
color.

Links: [GitHub](https://github.com/terror/textmap),
[Website](https://textmap.up.railway.app/)
