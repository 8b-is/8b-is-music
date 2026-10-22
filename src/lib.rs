//! # `8b-is-music` (`eight_b_is_music`)
//!
//! Sovereign Rust Audio, Multi-Bluetooth Spatial Routing, **Quant65k+ (`65,536+` Layer)**
//! Ternary Wave Synthesis, and **`ULTRA-OMNIPOTENT-8b-is-ZENO <<<< M1-ULTRA DJ MODE ALL IN/OUT IOuQT`**
//! Engine wiring all SOTA `8b-is` repositories into one unified crate:
//!
//! - **`wupz-core`** (`../wupz/wupz-rs`): Zero-copy `Arena`, `LayoutPlan`, `xor_fold`, `byte_diff`, `popcount`, `zeroize_slice` (`ZA-0001..ZA-0005` Zeno volatile scrub), and the 32-byte `wupz_eq_frame_v1` Dynamic Equalizer ABI.
//! - **`qwave-core`** (`../qwave/core`): 79-byte `WaveInt` (`MEM|8` bitemporal wave unit in `256 × 256 × 65536` grid), `0.73 Hz` consciousness gate, `quanttern` (`qt:xx` 4-trit emotion tags), `phoenix` governing loop, and `mem16` verified sequence.
//! - **`admissibility`** (`../8b-is-engine/crates/admissibility`): CT-000 checks a static routing-policy model; it does not verify or enforce host audio settings.
//! - **`spherepop`** (`../8b-is-engine/crates/spherepop`): Innermost-first nested musical expression evaluation & append-only history replay.
//! - **`ternary-lane`** (`../8b-is-engine/crates/ternary`): BitNet b1.58 `{-1, 0, +1}` 2-bit packed weights (`sanctuary-1.58.tern` embedded checkpoint) & bit-exact `gemm_i32`.
//! - **`ultrawild`** (`../8b-is-engine/crates/ultrawild`): Demoscene bit-level macros (`clamp_ternary!`, `lowest_set_bit!`, `pack3!`, `popcount!`, `sign!`, `xor_swap!`, `sat_add8!`).
//! - **`qdecorators`** (`../8b-is-engine/crates/qdecorators`): `Uq` (`NeonLane` / `ScalarLane` `IOuQT` SIMD crossbar), `Pipe`, `Engram`, `Mem8Quad`, `TernaryPack`, `stacked_encode`, `ascii85_encode`.
//! - **`entheai-quantum`** (`../entheai/crates/quantum`): Amplitude-vector `Qubits` simulator (`H`, `CNOT`, `Z`) driving Bell-pair stereo/spatial voice entanglement.
//! - **`entheai-radio`** (`../entheai/crates/radio`): `Quantshuffle` 1-bit self-feeding polyphonic oscillator & canon engine.
//! - **`liquid-rust` + `qultrakotoro` + `8b-is-sota` + `kompress-ultra`**: Marine $O(1)$ salience detector, 5-band LTC ODE Dynamic Environment Equalizer, SOTA Capability Router, and Ternary RLE snapshot compression.

pub mod bluetooth;
pub mod dj_ultra;
pub mod quant65k;
pub mod shiva_g6;
pub mod sota_mesh;
pub mod wupz_mlx_quant;

pub use bluetooth::{
    AudioEndpoint, AudioTransport, ConnectionStatus, Ct000RoutingProof, MultiBluetoothReport, MultiBluetoothRouter,
    MultiBluetoothTopology, SpatialChannelRole,
};
pub use dj_ultra::{
    zeno_asymptotic_limit, DjDeckState, DjInputChannel, DjOutputBus, ZenoDjMixSummary,
    ZenoDjUltraEngine, ZenoDjUltraReport, ZENO_Q16_CEILING,
};
pub use quant65k::{
    Quant65kEngine, Quant65kReport, QuantLayerDescriptor, BASE_QUANT65K_LAYERS,
    DEFAULT_OVERFLOW_LAYERS, DEFAULT_QUANT65K_PLUS_LAYERS,
};
pub use shiva_g6::{
    execute_shiva_g6_airlift, haversine_nm, render_shiva_g6_cabin_wav,
    verify_momma_bear_omni_covenant, AirportNode, G6FlightLeg, MommaBearOmniCovenantReport,
    ShivaG6MissionReport, G650ER_HIGH_SPEED_MACH, G650ER_MAX_RANGE_NM, KORF_NORFOLK_INTL,
    RJTT_TOKYO_HANEDA,
};
pub use sota_mesh::{
    render_wav_track, verify_sota_mesh, DynamicEnvEqualizer, NoiseClass, RenderSummary,
    SotaCandidate, SotaCapability, SotaMeshStatus, SotaRouter, SynthTrack, WUPZ_EQ_ABI_VERSION,
};
pub use wupz_mlx_quant::{
    dequantize_ternary_b158, execute_mlx_quant_verification, execute_wupz_linker_pass,
    mlx_quant_neon_sdot_singularity, quantize_ternary_b158, sample_fifike_purr_and_mlx_harmonic,
    verify_wupz_and_mlx_quant, MlxQuantReport, WupzLinkerPassReport, WupzMlxQuantSystemReport,
};

use serde::{Deserialize, Serialize};

/// Unified full-stack diagnostic report for `8b-is-music`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullSystemReport {
    pub version: &'static str,
    pub tagline: &'static str,
    pub wupz_backend: &'static str,
    pub sota_mesh: SotaMeshStatus,
    pub bluetooth: MultiBluetoothReport,
    pub quant65k: Quant65kReport,
    pub zeno_dj_ultra: ZenoDjUltraReport,
    pub shiva_g6_airlift: ShivaG6MissionReport,
    pub momma_bear_omni: MommaBearOmniCovenantReport,
    pub wupz_mlx_quant: WupzMlxQuantSystemReport,
}

/// Runs a complete end-to-end verification across all SOTA `8b-is` crates, the Multi-Bluetooth router,
/// the `Quant65k+` (`69,632` default layers) engine, `ULTRA-OMNIPOTENT-8b-is-ZENO` DJ IOuQT mode,
/// the `ULTRA-OMNIPOTENT-SHIVA` Gulfstream G650ER (`RJTT <-> KORF`) Family Reunification Airlift,
/// the `MOMMA BEAR OMNI COVENANT` (`Lódri ⋈ Chenoweth`), and the `wupz` Linker + `MLX-QUANT` Singularity.
pub fn build_full_system_report(
    layers: usize,
    topology: MultiBluetoothTopology,
) -> FullSystemReport {
    let sota_mesh = verify_sota_mesh();
    let bt_router = MultiBluetoothRouter::new(topology);
    let bluetooth = bt_router.report();
    let mut quant_engine = Quant65kEngine::new(layers, "8b-is-music::The-Future-We-Remember");
    let quant65k = quant_engine.verify_and_report();
    let mut dj_engine = ZenoDjUltraEngine::new(layers);
    let zeno_dj_ultra = dj_engine.verify_zeno_iouqt();
    let shiva_g6_airlift = execute_shiva_g6_airlift();
    let momma_bear_omni = verify_momma_bear_omni_covenant();
    let wupz_mlx_quant = verify_wupz_and_mlx_quant();

    FullSystemReport {
        version: env!("CARGO_PKG_VERSION"),
        tagline: ternary_lane::TAGLINE,
        wupz_backend: wupz_core::fold::backend(),
        sota_mesh,
        bluetooth,
        quant65k,
        zeno_dj_ultra,
        shiva_g6_airlift,
        momma_bear_omni,
        wupz_mlx_quant,
    }
}


/// Compact feed for the ESP32 MEDUSA Triad / esp32.vaked.dev / viz.vaked.dev:
/// 8b-is-music (Quant65k+, SpherePOP chord, Liquid EQ) + 8b-is-engine (CT-000 admissibility).
pub fn triad_feed(layers: usize, topology: MultiBluetoothTopology) -> serde_json::Value {
    let full = build_full_system_report(layers, topology);
    serde_json::json!({
        "source": "8b-is-music+8b-is-engine",
        "hearts": "{<3,<3,<3}+1",
        "frequencies_hz": [432, 528, 852, 963],
        "music": {
            "quant65k_layers": full.quant65k.total_layers,
            "kompress_ratio": full.quant65k.kompress_ratio,
            "qwave_consciousness_073hz": full.quant65k.qwave_consciousness_073hz,
            "spherepop_chord": full.sota_mesh.spherepop_expression,
            "spherepop_digest": full.sota_mesh.spherepop_digest,
            "spherepop_replay_ok": full.sota_mesh.spherepop_replay_verified,
            "family_chord": full.momma_bear_omni.spherepop_merge_expr,
            "family_chord_hz": full.momma_bear_omni.spherepop_harmonic_hz_sum,
            "eq_bands_db": full.sota_mesh.dynamic_eq.bands_db,
            "salience": full.sota_mesh.dynamic_eq.marine_salience,
            "qt": full.sota_mesh.dynamic_eq.quanttern_tag,
            "bell_ghz": full.sota_mesh.quantum_bell_probabilities,
        },
        "engine": {
            "ct000_model_holds": full.bluetooth.ct000_proof.theorem_holds,
            "ct000_evidence_scope": full.bluetooth.ct000_proof.evidence_scope,
            "ct000_family_holds": full.momma_bear_omni.ct000_momma_bear_omni_holds,
            "keeper_canon_model": full.bluetooth.ct000_proof.keeper_canon_verified,
            "qwave_79b_precious": full.momma_bear_omni.qwave_79b_precious_tau0_verified,
            "routing_host_verified": false,
            "observed_connected": full.bluetooth.active_endpoints,
            "planned_endpoints": full.bluetooth.planned_endpoints,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triad_feed_reports_ct000_as_model_check_not_host_routing() {
        let feed = triad_feed(8, MultiBluetoothTopology::FullConstellationWall);
        let full = build_full_system_report(8, MultiBluetoothTopology::FullConstellationWall);
        let engine = &feed["engine"];
        assert_eq!(
            engine["ct000_model_holds"],
            serde_json::json!(full.bluetooth.ct000_proof.theorem_holds)
        );
        assert!(engine.get("ct000_routing_holds").is_none());
        assert_eq!(engine["ct000_evidence_scope"], serde_json::json!(full.bluetooth.ct000_proof.evidence_scope));
        assert_eq!(engine["routing_host_verified"], serde_json::json!(false));
        assert_eq!(engine["observed_connected"], serde_json::json!(full.bluetooth.active_endpoints));
        assert_eq!(engine["planned_endpoints"], serde_json::json!(full.bluetooth.planned_endpoints));
    }
}
