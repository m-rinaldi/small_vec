# small_vec

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://travis-ci.com/m-rinaldi/small_vec.svg?branch=main)](https://travis-ci.com/m-rinaldi/small_vec)

A *dynamic array* or vector supporting *small buffer optimization*.

`SmallVec<T, N>` is, in essence, just an `enum` with two variants:

- `Local`: contains a built-array (`[T; N]`) and its current size (`usize`), i.e., a buffer allocated locally inside the `SmallVec` itself.
- `Remote`: a `Vec<T>`, i.e., a remote buffer allocated on the heap.


The capacity of the local buffer is specified at compile time as a constant generic argument.



