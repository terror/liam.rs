---
title: minic
date: 2025-01-14
repo: terror/minic
topics: ['Java', 'Compilers']
lead: A compiler for a C-like language.
image: minic.png
---

**minic** is a compiler for a C-like language, written in Java.

It supports a wide range of features like functions, arrays, structs, recursion,
etc.

To build the web playground I compiled to WASM and exported a single function
`compile` that returns information about the program, e.g. tokens, ast, errors,
etc.

The compiler targets MIPS assembly, and I couldn't find any good existing
simulators I could easily use in the browser, so I wrote my own.

This project was written as part of a
[compiler design course](https://www.mcgill.ca/study/2024-2025/courses/comp-520){target="\_blank"}
offered at [McGill University](https://www.mcgill.ca/){target="\_blank"}

Links: [GitHub](https://github.com/terror/minic){target="\_blank"}
