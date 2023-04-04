# `conway-polyhedron-notation-parser`

As you may have barely deduced from its name, this is a parser for
[Conway polyhedron notation](https://en.wikipedia.org/wiki/Conway_polyhedron_notation).

The functionality in this crate is now included in the
[`polyhedron-ops`](https://crates.io/crates/polyhedron-ops) crate (behind the
`parser` feature). I.e. don't expect updates but rather that this repository may be archived soonish.

You can use this crate to dump Conway notation strings to [Wavefront `.obj` files](https://en.wikipedia.org/wiki/Wavefront_.obj_file). E.g. to convert `kaD` to geomtry this way:

```
cargo run -- kaD
```

Will export a the [kleetrope](https://en.wikipedia.org/wiki/Kleetope) of a
[rectified](https://en.wikipedia.org/wiki/Rectification_(geometry))
[dodecahedron](https://en.wikipedia.org/wiki/Dodecahedron) to
`./polyhedron-kaD.obj`.

If no parameters are given it exports a [rhombicosidodecahedron](https://en.wikipedia.org/wiki/Rhombicosidodecahedron).