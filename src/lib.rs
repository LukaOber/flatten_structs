#![cfg_attr(docsrs, feature(doc_cfg))]
/*!
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
use flatten_structs::flatten_structs;

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
*/
#[allow(unused_imports)]
#[doc(hidden)]
pub use paste::paste as __private_codegen_paste;

#[macro_export]
macro_rules! flatten_structs {
    // Entry point:
    (
        $(#[$struct_attr:meta])*
        $vis:vis
        struct
        $name:ident {$(
            $(#[$($field_attr:tt)*])*
            $field_vis:vis $field_name:ident: $field_type:path
        ),* $(,)? }
    ) => {
        // Start recursive macro calls:
        $crate::flatten_structs!{@gather_fields
            expanded_fields = {},
            queued_fields = { $({
                $(#[$($field_attr)*])*
                $field_vis $field_name: $field_type
            })* },
            cx = {
                definition = {
                    $(#[$struct_attr])*
                    $vis
                    struct
                    $name
                },
                dollar = { $ },
            },
        }
    };
    // Handle Queued field (with flatten attribute)
    (@gather_fields
        expanded_fields = $expanded_fields:tt,
        queued_fields = { {
            $(#[doc = $($field_docs:tt)*])*
            #[flatten]
            $(#[$($field_attr:tt)*])*
            $field_vis:vis $field_name:ident: $field_type:path
        } $($queued_fields:tt)* },
        cx = $cx:tt,
    ) => {
        $field_type!{
            call = { $crate::flatten_structs },
            prefix = { @callback },
            cx = {
                flattened_vis = { $field_vis },
                expanded_fields = $expanded_fields,
                queued_fields = { $($queued_fields)* },
                cx = $cx,
            },
        }
    };
    // Callback from "inspection" macro when flattening type
    (@callback
        fields = {$(
            $(#[$($field_attr:tt)*])*
            $field_vis:vis $field_name:ident: $field_type:path,
        )*},
        cx = {
            flattened_vis = { $flattened_vis:vis },
            expanded_fields = { $($expanded_fields:tt)* },
            queued_fields = $queued_fields:tt,
            cx = $cx:tt,
        },
    ) => {
        $crate::flatten_structs!{@gather_fields
            expanded_fields = { $($expanded_fields)* $({
                $(#[$($field_attr)*])*
                $flattened_vis $field_name: $field_type
            })*},
            queued_fields = $queued_fields,
            cx = $cx,
        }
    };
    // Handle Queued field (without flatten attribute)
    (@gather_fields
        expanded_fields = { $($expanded_fields:tt)* },
        queued_fields = { {
            $(#[$($field_attr:tt)*])*
            $field_vis:vis $field_name:ident: $field_type:path
        } $($queued_fields:tt)* },
        cx = $cx:tt,
    ) => {
        $crate::flatten_structs!{@gather_fields
            expanded_fields = { $($expanded_fields)* {
                $(#[$($field_attr)*])*
                $field_vis $field_name: $field_type
            }},
            queued_fields = { $($queued_fields)* },
            cx = $cx,
        }
    };
    // Done, have gathered info about all fields:
    (@gather_fields
        expanded_fields = { $({
            $(#[$field_attr:meta])*
            $field_vis:vis $field_name:ident: $field_type:path
        })* },
        queued_fields = {},
        cx = {
            definition = {
                $(#[$struct_attr:meta])*
                $vis:vis
                struct
                $name:ident
            },
            dollar = { $dollar:tt },
        },
    ) => {
        $(#[$struct_attr])*
        $vis struct $name {$(
            $(#[$field_attr])*
            $field_vis $field_name: $field_type,
        )*}
        $crate::__private_codegen_paste!{
            // Unique name for this macro:
            // This macro allows another macro to query this types fields.
            macro_rules! [<__private_field_inspect_for $name>] {
                (
                    call = { $dollarcall:path },
                    prefix = { $dollar($dollarprefix:tt)* },
                    cx = $dollarcx:tt,
                ) => {
                    $dollarcall! {$dollar($dollarprefix)*
                        fields = {$(
                            $(#[$field_attr])*
                            $field_vis $field_name: $field_type,
                        )*},
                        cx = $dollarcx,
                    }
                };
            }
            // But expose the macro with the same name as the generated
            // type. This works because types, macros and values all
            // have different namespaces so they don't conflict.
            #[allow(unused_imports)]
            pub(crate) use [<__private_field_inspect_for $name>] as $name;
        }
    };
}

#[cfg(test)]
mod test {
    use serde::{Deserialize, Serialize};

    #[test]
    fn flatten_structs() {
        flatten_structs!(
            #[allow(unused)]
            pub struct BaseStruct {
                enable: bool,
                #[flatten]
                n1: NestedStruct1,
                #[flatten]
                n2: NestedStruct2,
            }
        );

        flatten_structs!(
            #[allow(unused)]
            struct NestedStruct1 {
                value: f32,
            }
        );

        flatten_structs!(
            #[allow(unused)]
            struct NestedStruct2 {
                goal: f32,
                #[flatten]
                sn: SubNestedStruct,
            }
        );

        flatten_structs!(
            #[allow(unused)]
            struct SubNestedStruct {
                min: f32,
                max: f32,
            }
        );

        let base_struct = BaseStruct {
            enable: true,
            value: 0.0,
            goal: 10.0,
            min: 0.0,
            max: 20.0,
        };
        assert!(base_struct.enable);
    }

    #[test]
    fn flatten_serde_struct() {
        flatten_structs!(
            #[derive(Serialize, Deserialize)]
            pub struct BaseSerdeStruct {
                enable: bool,
                #[flatten]
                nested: NestedSerdeStruct,
            }
        );

        flatten_structs!(
            #[allow(unused)]
            #[derive(Serialize, Deserialize)]
            struct NestedSerdeStruct {
                min: Option<f32>,
                #[serde(skip_serializing_if = "Option::is_none")]
                max: Option<f32>,
            }
        );

        let base_struct = BaseSerdeStruct {
            enable: false,
            min: Some(0.0),
            max: None,
        };
        let base_struct_json = serde_json::to_string_pretty(&base_struct).unwrap();
        pretty_assertions::assert_eq!(
            r#"{
  "enable": false,
  "min": 0.0
}"#,
            base_struct_json
        );
    }
}
