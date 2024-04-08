# Usage

- `raw_mem_mapper = { git = "https://github.com/dxshie/RawMemMapper"}`

```rust
    struct_pad_aligned! {
        #[derive(DebugPadLess)]
        pub struct CBaseEntity {
            m_pGameSceneNode pub game_scene_node: usize, // ptr address
            m_iHealth pub health: i32,
            m_iTeamNum pub team: u8,
            m_fFlags pub flags: i32,
            pub abs_velocity: [f32; 2],
            m_MoveType pub move_type: i32,
            m_vecViewOffset pub vec_view_offset: [f32; 3],
            m_pWeaponServices pub weapon_service: usize,
            m_vOldOrigin pub v_old_origin: [f32; 3],
            m_pClippingWeapon pub weapon: usize,
            m_angEyeAngles pub eye_angles: [f32; 2],
        }
    }
```
