---
title: llmbda
date: 2023-10-23
repo: SamZhang02/llmbda
topics: ['Research', 'NLP', 'Python']
lead: A large language model based propositional logic deduction assistant.
image: llmbda.png
---

**llmbda** was a research project I worked on for a natural language processing
course I took at McGill University.

Alongside two friends, we came up with a plan to investigate how good large
language models could be at parsing natural language into logical expressions.

For instance, we wanted to explore turning an expression like this:

`COMP 202 and (COMP 250 or COMP 206) or permission of instructor`

into this:

`COMP 202 ∧ (COMP 250 ∨ COMP 206)`

We explored a zero-shot, few-shot and fine-tuned alignment for various available
language models, using the most accurate strategy in production for
[mcgill.courses](https://mcgill.courses/){target="\_blank"} course graph
generation.

Links: [GitHub](https://github.com/SamZhang02/llmbda){target="\_blank"}
