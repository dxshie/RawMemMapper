# Usage

- `raw_mem_mapper = { git = "https://github.com/dxshie/RawMemMapper"}`

```rust
    struct_pad_aligned! {
        #[derive(DebugPadLess)]
        pub struct Foo {
            Node pub node: usize,
            Health pub health: i32,
            Team pub team: u8,
            Falgs pub flags: i32,
            pub velocity: [f32; 2],
            0x10C pub move_type: i32,
        }
    }
```
