# Bruteforce test for Hackers Delight

For a nice overview of the programs see 
["Synthesis of Loop-free Programs"](https://susmitjha.github.io/papers/pldi11.pdf#page=8)


Note: The programs are not guaranteed to be correct at this point
but they are filtered using some inputs
here, one would throw the candidates into a SMT solver to verify correctness

This program uses a bruteforce approach to generate all possible programs and filters them
=> all combinations |> filter

A much more efficient approach would be the enumeration approach with deduplication using
input-output examples, this would filter out programs like add(x,y) and add(y,x) which are equivalent
but it might rule out correct ones accidentally this way
in the end, all candidates (possibly with equivalence closure) are thrown into SMT
and if no solution is found, the generated counter example(s) are added to the input-output examples
see my other synthesis repositories for this approach
it is a bit more complicated but much more efficient