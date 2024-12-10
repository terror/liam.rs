---
title: degenerate
date: 2021-12-10
repo: terror/degenerate
topics: ['Rust', 'WASM', 'WebGL', 'Graphics']
lead: A generative art computer.
image: degenerate.png
---

**degenerate** is a programmable generative art engine that runs in the browser
that can be programmed with Rust or JavaScript.

Internally, it operates as a chain of image filters. The output of each filter
is used as the input of the next filter. Programs configure the state of the
current image filter and call `render()` to apply it.

Programs that are written in JavaScript are sent to a Web Worker for execution.
The program then sends back a series of `Filter` objects from the worker, which
are used to configure the renderer that runs in the main thread. The renderer
renders to a full-page `<canvas>` element using
[WebGL](https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API){target="\_blank"}.

I worked on this primarily in collaboration with
[Casey Rodarmor](https://rodarmor.com/){target="\_blank"}.

Links: [GitHub](https://github.com/casey/degenerate){target="\_blank"},
[Manual](https://degenerate.computer/man/introduction.html){target="\_blank"},
[Website](https://degenerate.computer/){target="\_blank"}
