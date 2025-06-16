# flatten_structs

Allows inlining fields into another struct.
This derive processes `#[flatten]` attributes and inlines the field definitions
for those types, the names of flattened fields are ignored. Note that this
macro generates a new macro with the same name as the derived type that is
exported with `pub(crate)`. This generated macro can be used to inspect the
field definitions of the derived type. This mechanism is what allows flattening
types so any types that will later be flattened needs to also use this derive
macro.
Due to declarative macro limitations any `#[flatten]` attributes need to be
the first attribute for a field or it won't be found by the macro.
The code used in this library is originally from [here][macro_source].

In most cases you should not need this macro. But if you have different structs, which
share a lot of fields and you want to avoid duplicating their declaration and avoid an
indirection this is useful.

[macro_source]: <https://users.rust-lang.org/t/is-implementing-a-derive-macro-for-converting-nested-structs-to-flat-structs-possible/65839/3>

```rust
flatten_structs!(
    #[allow(unused)]
    pub struct BaseStruct {
        enable: bool,
        #[flatten]
        nested: NestedStruct,
    }
);

flatten_structs!(
    #[allow(unused)]
    struct NestedStruct {
        value_0: f32,
        value_1: f32,
    }
);
// Will expand the BaseStruct to this
let base_struct = BaseStruct {
    enable: true,
    value_0: 0.0,
    value_1: 0.0,
};
```

License: MIT
