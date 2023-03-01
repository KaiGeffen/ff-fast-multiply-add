# ff-fast-multiply-add
Hack project highlighting implementations of algorithms to add and multiply finite fields quickly

Based off of description of Barrett and Montgomery reduction detailed by Risc0:
https://www.youtube.com/watch?v=hUl8ZB6hpUM&t=0s

Tests show naive and Barret reduction for finite field modular algebra.
Benchmarks attempt to see the result of using a more efficient reduction implementation (Barrett) by avoiding having to divide (Expensive) achieved by bitshifting and precomputing key values.

Further explanation video:
https://www.youtube.com/watch?v=ZW7hjIuDp_U
