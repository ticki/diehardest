Diehardest
==========

> A new approach to randomness testing.

Diehardest is a small library providing strong tools to rate quality of pseudorandom streams.
It works with two components:

1. A number of transformations which will weaken weak RNGs.
2. A collection of analytical tools, which rates the transformed streams.

In contrast to many other randomness tests, diehardest is stream-aware, making it able to
detect many positional patterns that other tests cannot.
