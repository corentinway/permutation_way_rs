
![Build Passing](https://api.travis-ci.org/corentinway/permutation_way_rs.svg?branch=master)

# About

This is an implementation of the [Steinhaus Johnson Trotter algorithm with Even's speedup](https://en.wikipedia.org/wiki/Steinhaus%E2%80%93Johnson%E2%80%93Trotter_algorithm#Even's_speedup).
This algorithm compute all permutations of a given input vector.

Given a vector of `[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]` we expect to have `3,628,800` permutations.
They was found in `800.475710` milliseconds without any threads. The test with CPU Intel(R) Core(TM) i5 @ 6.30 GHz and 16 Gb of RAM


The roadmap bellow suggest some path to make it faster again.

# Roadmap

* use of generics : for the moment, the accepted input is an array of `i32`.
* finally I will use threads to have it faster.

I may had a `max` options in order to stop the computation of permutation. I may be a number of a function
that test some conditions. An `interrupt` function may be added to stop the research of the next permutations.