# Introduction

<!-- Some introduction to confined Bose gases and their relevance (perhaps some historical bullshit or something, I don't fucking know 😅). -->

Finding the ground state of a confined Bose-gas can be a difficult task to do analytically. As such, we shall implement a variational Monte Carlo solver specialized for the problem at hand, and solve it numerically. This is done in Rust, a fast, safe and modern language.

<!-- I hate these kinds of "listing" paragraphs, but I know they are required... Will try to rewrite later to fit more naturally into the introduction. -->
We will first introduce the theory behind our approach and find an analytical solution for a simplified system. This is done to have a benchmark from which we can compare the performance of our numerical solver. Thereafter we present the methodology behind our codebase and how its implemented. The sections thereafter show our findings and a discussion regarding them.
