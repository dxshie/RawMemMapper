#[allow(dead_code, non_snake_case, non_upper_case_globals)]
#[cfg(test)]
mod test {
    use memhelp::{struct_pad_aligned, DebugPadLess};
    const Node: usize = 0x100;

    struct_pad_aligned! {
        #[derive(DebugPadLess)]
        pub struct Bar {
            0x30 pub field: usize, // ptr address
        }
    }

    struct_pad_aligned! {
        #[derive(DebugPadLess)]
        pub struct Foo {
            Node pub node: usize, // ptr address
            0x10C pub health: i32,
            pub abs_velocity: [f32; 2],
        }
    }

    struct_pad_aligned! {
        #[derive(DebugPadLess)]
        pub struct SomeStruct {
            Node pub node: usize, // ptr address
            0x10C pub health: i32,
            pub abs_velocity: [f32; 2],
        }
    }

    struct_pad_aligned! {
        pub struct Test {
            0x200 pub test: bool, // ptr address
            0x201 pub test2: i32, // ptr address
            0x205 pub test3: bool, // ptr address
        }
    }

    struct_pad_aligned! {
        pub struct NotFirstOffset {
            pub node: usize, // ptr address
            pub health: i32,
            pub team: u8,
            pub flags: i32,
            pub eye_angles: [f32; 2],
        }
    }

    #[test]
    fn test_create_pad_struct() {
        let pad_struct = Test {
            _pad0: [0; 512],
            test: false,
            _pad1: [0; 0],
            test2: 0,
            _pad2: [0; 0],
            test3: false,
        };
        assert_eq!(pad_struct.test, false);
    }
}
