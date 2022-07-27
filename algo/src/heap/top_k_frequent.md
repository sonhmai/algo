# Notes

What does `&' K` returning by `HashMap::iter()` mean?

```rust 
// Example
for (&n, &c) in counts.iter() {
    heap.push(Element{num: n, count: c})
}
```

`'a` means the lifetime a