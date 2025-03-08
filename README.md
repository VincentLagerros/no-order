# No-Order
This crate provides a transparent helper wrapper for excluding a value from any ordering or hashing, similar to `std::cmp::Reverse`. Simply put, all `NoOrder<T>` instance are always equals to any other `NoOrder<T>` instance. This allows the compiler to to skip comparing or hashing the value, making some algorithms faster. This is especially useful for sorting larger structs in tuples, representing data like `String` or `Vec`, as they are both expensive to compare and hash.

Additionally also allows putting data types that do not implement `Ord` or `Hash` (like `f32`) in datastructures like `BinaryHeap`, `HashSet` or `BTreeSet`. 

Discussion: https://internals.rust-lang.org/t/pre-rfc-std-noorder/22514

## Examples

An example usecase for this would be in algorithms like Dijkstra's, where a BinaryHeap does not have to sort the vertex. This makes the algorithm ~25% faster, as the order you visit the vertices of the same `path_weight` does not matter. This also applies to `A*` varitations.

```rust
while let Some((path_weight, NoOrder(vertex))) = dijkstra.pop() {
    for &(next_vertex, weight) in &graph[vertex] {
        let new_weight = path_weight + weight;
        // ... Go to the next vertex
    }
}
```

NoOrder could also be used to mark some variable as not relevant when sorting, and to use the derive macro to avoid the boilerplate of a implementing `PartialEq, Eq, PartialOrd, Ord`.

```rust
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Player {
    score : u32,
    name : NoOrder<String>,
}
```

Another use case would be to avoid hashing unhashable data types, where we are only really intrested in hashing some fields. All NoOrder instances are equal to each other, so the hashing implementation simply ignores NoOrder.

```rust
#[derive(Hash)]
struct Enemy {
    id : u32,
    x : NoOrder<f64>,
    y : NoOrder<f64>,
}
```


## Note
This can be also be used with `sort()`, but you should instead use `sort_by()`. In the same way, it may also be more preferable to use `HashMap`/`BTreeMap` compared to a `HashSet`/`BTreeSet` with a `NoOrder` tuple, depending on your use case.
