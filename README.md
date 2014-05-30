2048-rust-ai
============

A toy implementation of 2048 as well as a modified expectimax implementation to play it.

Unlike traditional implementations of expectimax, when expanding the expected layers instead of searching down each node,
my implementation takes a set number of samples and expands those nodes. This reduces the computation when just starting out 
and moves don't affect much but does the full search as the board gets more dense.

When expanding 6 layers down (3 max, and 3 expected) in my trial of 100 games, it is able to get 1024 95% of the time. 2048 72% of the time
4096 18% of the time. This ran in a little over 20 min on my i7 desktop.
