#[allow(dead_code, non_snake_case, non_upper_case_globals)]
#[cfg(test)]
mod test {
    use raw_mem_mapper::{struct_pad_aligned, DebugPadLess};

    // CEntityInstance
    pub const m_CBodyComponent: usize = 0x38; // CBodyComponent*
    pub const m_NetworkTransmitComponent: usize = 0x40; // CNetworkTransmitComponent
    pub const m_nLastThinkTick: usize = 0x310; // GameTick_t
    pub const m_pGameSceneNode: usize = 0x318; // CGameSceneNode*
    pub const m_pRenderComponent: usize = 0x320; // CRenderComponent*
    pub const m_pCollision: usize = 0x328; // CCollisionProperty*
    pub const m_iMaxHealth: usize = 0x330; // int32_t
    pub const m_iHealth: usize = 0x334; // int32_t
    pub const m_lifeState: usize = 0x338; // uint8_t
    pub const m_bTakesDamage: usize = 0x339; // bool
    pub const m_nTakeDamageFlags: usize = 0x33C; // TakeDamageFlags_t
    pub const m_bIsPlatform: usize = 0x340; // bool
    pub const m_ubInterpolationFrame: usize = 0x341; // uint8_t
    pub const m_hSceneObjectController: usize = 0x344; // CHandle<C_BaseEntity>
    pub const m_nNoInterpolationTick: usize = 0x348; // int32_t
    pub const m_nVisibilityNoInterpolationTick: usize = 0x34C; // int32_t
    pub const m_flProxyRandomValue: usize = 0x350; // float
    pub const m_iEFlags: usize = 0x354; // int32_t
    pub const m_nWaterType: usize = 0x358; // uint8_t
    pub const m_bInterpolateEvenWithNoModel: usize = 0x359; // bool
    pub const m_bPredictionEligible: usize = 0x35A; // bool
    pub const m_bApplyLayerMatchIDToModel: usize = 0x35B; // bool
    pub const m_tokLayerMatchID: usize = 0x35C; // CUtlStringToken
    pub const m_nSubclassID: usize = 0x360; // CUtlStringToken
    pub const m_nSimulationTick: usize = 0x370; // int32_t
    pub const m_iCurrentThinkContext: usize = 0x374; // int32_t
    pub const m_aThinkFunctions: usize = 0x378; // CUtlVector<thinkfunc_t>
    pub const m_nDisableContextThinkStartTick: usize = 0x390; // GameTick_t
    pub const m_flAnimTime: usize = 0x394; // float
    pub const m_flSimulationTime: usize = 0x398; // float
    pub const m_nSceneObjectOverrideFlags: usize = 0x39C; // uint8_t
    pub const m_bHasSuccessfullyInterpolated: usize = 0x39D; // bool
    pub const m_bHasAddedVarsToInterpolation: usize = 0x39E; // bool
    pub const m_bRenderEvenWhenNotSuccessfullyInterpolated: usize = 0x39F; // bool
    pub const m_nInterpolationLatchDirtyFlags: usize = 0x3A0; // int32_t[2]
    pub const m_ListEntry: usize = 0x3A8; // uint16_t[11]
    pub const m_flCreateTime: usize = 0x3C0; // GameTime_t
    pub const m_flSpeed: usize = 0x3C4; // float
    pub const m_EntClientFlags: usize = 0x3C8; // uint16_t
    pub const m_bClientSideRagdoll: usize = 0x3CA; // bool
    pub const m_iTeamNum: usize = 0x3CB; // uint8_t
    pub const m_spawnflags: usize = 0x3CC; // uint32_t
    pub const m_nNextThinkTick: usize = 0x3D0; // GameTick_t
    pub const m_fFlags: usize = 0x3D4; // uint32_t
    pub const m_vecAbsVelocity: usize = 0x3D8; // Vector
    pub const m_vecVelocity: usize = 0x3E8; // CNetworkVelocityVector
    pub const m_vecBaseVelocity: usize = 0x418; // Vector
    pub const m_hEffectEntity: usize = 0x424; // CHandle<C_BaseEntity>
    pub const m_hOwnerEntity: usize = 0x428; // CHandle<C_BaseEntity>
    pub const m_MoveCollide: usize = 0x42C; // MoveCollide_t
    pub const m_MoveType: usize = 0x42D; // MoveType_t
    pub const m_nActualMoveType: usize = 0x42E; // MoveType_t
    pub const m_flWaterLevel: usize = 0x430; // float
    pub const m_fEffects: usize = 0x434; // uint32_t
    pub const m_hGroundEntity: usize = 0x438; // CHandle<C_BaseEntity>
    pub const m_flFriction: usize = 0x43C; // float
    pub const m_flElasticity: usize = 0x440; // float
    pub const m_flGravityScale: usize = 0x444; // float
    pub const m_flTimeScale: usize = 0x448; // float
    pub const m_bAnimatedEveryTick: usize = 0x44C; // bool
    pub const m_flNavIgnoreUntilTime: usize = 0x450; // GameTime_t
    pub const m_hThink: usize = 0x454; // uint16_t
    pub const m_fBBoxVisFlags: usize = 0x460; // uint8_t
    pub const m_bPredictable: usize = 0x461; // bool
    pub const m_bRenderWithViewModels: usize = 0x462; // bool
    pub const m_nSplitUserPlayerPredictionSlot: usize = 0x464; // CSplitScreenSlot
    pub const m_nFirstPredictableCommand: usize = 0x468; // int32_t
    pub const m_nLastPredictableCommand: usize = 0x46C; // int32_t
    pub const m_hOldMoveParent: usize = 0x470; // CHandle<C_BaseEntity>
    pub const m_Particles: usize = 0x478; // CParticleProperty
    pub const m_vecPredictedScriptFloats: usize = 0x4A0; // CUtlVector<float>
    pub const m_vecPredictedScriptFloatIDs: usize = 0x4B8; // CUtlVector<int32_t>
    pub const m_nNextScriptVarRecordID: usize = 0x4E8; // int32_t
    pub const m_vecAngVelocity: usize = 0x4F8; // QAngle
    pub const m_DataChangeEventRef: usize = 0x504; // int32_t
    pub const m_dependencies: usize = 0x508; // CUtlVector<CEntityHandle>
    pub const m_nCreationTick: usize = 0x520; // int32_t
    pub const m_bAnimTimeChanged: usize = 0x539; // bool
    pub const m_bSimulationTimeChanged: usize = 0x53A; // bool
    pub const m_sUniqueHammerID: usize = 0x548; // CUtlString

    pub const m_vecViewOffset: usize = 0xC58; // CNetworkViewOffsetVector
    pub const m_pWeaponServices: usize = 0x1100; // CPlayer_WeaponServices*
    pub const m_vOldOrigin: usize = 0x127C; // Vector
    pub const m_pClippingWeapon: usize = 0x1308; // C_CSWeaponBase*
    pub const m_angEyeAngles: usize = 0x1578; // QAngle

    // if any on these invocations of the macro throw an error
    // all needed case to make this work are not fullfilled
    // cases are:
    // optional visibility
    // optional offset
    // traverse back to the last field that has an offset
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

    struct_pad_aligned! {
        pub struct CBaseEntityWithMoreNoneOffsets {
            m_pGameSceneNode pub game_scene_node: usize, // ptr address
            m_iHealth pub health: i32,
            m_iTeamNum pub team: u8,
            m_fFlags pub flags: i32,
            pub abs_velocity: [f32; 2],
            pub abs_velocity2: [f32; 2],
            pub abs_velocity3: [f32; 2],
            m_MoveType pub move_type: i32,
            m_vecViewOffset pub vec_view_offset: [f32; 3],
            m_pWeaponServices pub weapon_service: usize,
            m_vOldOrigin v_old_origin: [f32; 3],
            m_pClippingWeapon pub weapon: usize,
            m_angEyeAngles eye_angles: [f32; 2],
        }
    }

    struct_pad_aligned! {
        pub struct CBaseEntityWithLit {
            0x200 pub game_scene_node: usize, // ptr address
            m_iHealth pub health: i32,
            m_iTeamNum pub team: u8,
            m_fFlags pub flags: i32,
            pub abs_velocity: [f32; 2],
            m_MoveType pub move_type: i32,
            m_vecViewOffset pub vec_view_offset: [f32; 3],
            0x1100 pub weapon_service: usize,
            m_vOldOrigin pub v_old_origin: [f32; 3],
            m_pClippingWeapon pub weapon: usize,
            m_angEyeAngles pub eye_angles: [f32; 2],
        }
    }

    struct_pad_aligned! {
        pub struct CBaseEntityWithLitAllOffsets {
            0x200 pub test: bool, // ptr address
            0x201 pub test2: i32, // ptr address
            0x205 pub test3: bool, // ptr address
        }
    }

    struct_pad_aligned! {
        pub struct NotFirstOffset {
            pub game_scene_node: usize, // ptr address
            pub health: i32,
            pub team: u8,
            pub flags: i32,
            m_angEyeAngles pub eye_angles: [f32; 2],
        }
    }

    #[test]
    fn test_create_pad_struct() {
        let pad_struct = CBaseEntity {
            game_scene_node: 0,
            health: 0,
            team: 0,
            flags: 0,
            abs_velocity: [0.0, 0.0],
            move_type: 0,
            vec_view_offset: [0.0, 0.0, 0.0],
            weapon_service: 0,
            v_old_origin: [0.0, 0.0, 0.0],
            weapon: 0,
            eye_angles: [0.0, 0.0],
            _pad0: [0; 792],
            _pad1: [0; 20],
            _pad2: [0; 147],
            _pad3: [0; 8],
            _pad5: [0; 77],
            _pad6: [0; 2087],
            _pad7: [0; 1180],
            _pad8: [0; 372],
            _pad9: [0; 128],
            _pad10: [0; 616],
        };
        assert_eq!(pad_struct.move_type, 0);
    }
}
