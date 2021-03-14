# pyo3-pytypes

This is an experimental library for [`pyo3`](https://github.com/pyo3/pyo3) to add support for defining interfaces to Python classes from Rust.

This enables interacting with the Python object in a more natural fashion, as well as subclassing Python types from Rust.

***This ibrary is highly experimental - do not try to use in production yet.*** This library depends on `pyo3` master and additionally the API is likely to change several times before it is considered stable.

Everyone is encouraged to report issues and suggest design ideas.

## Usage

The goal is for this library to provide a macro, `pytype!`, to define an external Python class, ideally in a form which mirrors Python typing syntax.

```rust
pytype! {
    module = "mymod"

    class Foo:
        bar: int

        def baz() -> int:
            ...
}
```

This should lead to a Rust type which can be used `&'py Foo` like pyo3's native references. It also implements sufficient information for pyo3's `#[pyclass]` macro to create subtypes of it (with `#[pyclass(extends = Foo)]`).

The `Foo` type will also have methods `.get_bar(&self) -> &'py PyInt`, `.set_bar(&self) -> &'py PyInt`, and `.baz(&self) -> &'py PyInt`.

**At the moment the only usable part is that `&'py Foo` is usable as a "subclass" of PyAny from Rust.** Expect many bugs and missing features.
