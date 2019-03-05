# Neighbors benchmark using hash sets

One of the computational tasks I faced during my [PhD](http://www.ffconsultancy.com/free/thesis.html) in Chemistry was to identify the
neighbor shells in a crystal starting from a given atom and spreading outwards. In my case the unit cell of the crystal contained
1,000,000,000 atoms because it was a computer simulation of amorphous silicon. This task is easily accomplished using a functional style
with a purely functional `Set` collection using set theoretic operations union (∪) and difference (∖) as follows:

    let rec nth nn i n =
      match n with
      | 0 -> {i}
      | 1 -> nn i
      | n ->
          let prev = nth nn i (n-1)
          let next = Set.fold (fun i s -> nn i ∪ s) {} prev
          next ∖ prev ∖ nth nn i (n-2)

I created this benchmark specifically to quantify the performance of some Rust code. Sadly, I was unable to port this beautiful pure
solution of the problem to Rust because the lack of garbage collection makes it extremely tedious to use purely functional data structures
like `Set`. So I gave in and replaced the pure code with impure code using a mutable hash set instead, resulting in the versions you see
here.

## Results

The performance of the different programs varies enormously depending upon the search parameters and choice of hash function but they
clearly do not show Rust to have "blazing performance". Specifically, the Rust solution is faster in some cases and slower in others.
Consequently, I am not inclined to put much effort into using Rust when languages like OCaml and F# offer such huge advantages
elsewhere.

The problem with the Rust implementation lies in Rust's implementation of hash sets. Rust is designed for security so its hash-based data
structures assume crypto-level hash functions that result in few conflicts. In contrast, hash-based collections like .NET's `Dictionary`
and `HashSet` assume the hash function might be cheap and employ prime-length internal arrays to help avoid collisions. Consequently, the
Rust code underperforms either because the hash function is complicated and slow or because the hash function is too simple resulting in
too many collisions.

## Further work

In the future I would like to revisit this benchmark for two reasons:

* To test Rust's new purely functional collections.
* To test a hash set collection that uses prime-length arrays internally so it can work with cheaper hash functions.
