use std::collections::BTreeMap;

/*
B-Trees represent a fundamental compromise between cache-efficiency and actually minimizing
the amount of work performed in a search. In theory, a binary search tree (BST) is the
optimal choice for a sorted map, as a perfectly balanced BST performs the theoretical
minimum amount of comparisons necessary to find an element (log2n). However, in practice
the way this is done is very inefficient for modern computer architectures.
In particular, every element is stored in its own individually heap-allocated node.
This means that every single insertion triggers a heap-allocation, and every single
comparison should be a cache-miss. Since these are both notably expensive things to do
in practice, we are forced to at very least reconsider the BST strategy.

A B-Tree instead makes each node contain B-1 to 2B-1 elements in a contiguous array.
By doing this, we reduce the number of allocations by a factor of B, and improve cache efficiency
in searches. However, this does mean that searches will have to do more comparisons on average.
The precise number of comparisons depends on the node search strategy used. For optimal cache
efficiency, one could search the nodes linearly. For optimal comparisons, one could search
the node using binary search. As a compromise, one could also perform a linear search that
initially only checks every ith element for some choice of i.

Currently, our implementation simply performs naive linear search.
This provides excellent performance on small nodes of elements which are cheap to compare.
However in the future we would like to further explore choosing the optimal search strategy based
on the choice of B, and possibly other factors. Using linear search, searching for a random
element is expected to take B * log(n) comparisons, which is generally worse than a BST.
In practice, however, performance is excellent.
*/

fn main() {
    let mut movies = BTreeMap::new();

    movies.insert("Office Space", 10);
    movies.insert("Pulp Fiction", 5);
    movies.insert("The Godfather", 3);
    movies.insert("The Blues Brothers", 1);

    if !movies.contains_key("Les Misérables") {
        println!("We've got {} reviews, but Les Misérables ain't one.", movies.len());
    }

    movies.remove("The Blues Brothers");
}