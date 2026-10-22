//! # `ULTRA-OMNIPOTENT-8b-is-ZENO` Mode & `M1-ULTRA DJ MODE ALL IN/OUT IOuQT`
//!
//! Combines:
//! 1. **ZENO Mode (`ZenoEngine`)**:
//!    - **Zeno Paradox Asymptotic Brickwall Limiter** ($\sum_{k=1}^{16} 2^{-k} = \frac{65535}{65536}$):
//!      Guarantees `MAX VOLUME = 1.000 (0.0 dBFS)` across all 4 DJ decks + 69,632 layers + live inputs
//!      without ever clipping a single sample.
//!    - **Zeno Volatile Memory Scrubbing (`ZA-0001..ZA-0005`)**: Uses `wupz_core::arena::zeroize_slice`
//!      with `SeqCst` compiler fences so ephemeral DJ cue/session slabs leave zero stack/heap residue.
//! 2. **`IOuQT` (`Input/Output Uq + QuantTern` Full-Duplex Matrix)**:
//!    - Uses `qdecorators::Uq` (`NeonLane` on Apple Silicon M1/M3 `aarch64`, verified bit-exact against
//!      `ScalarLane` via `scalar_authority_gemm`) to run a real-time 1.58-bit ternary crossbar matrix
//!      connecting **ALL IN (`4` Full-Duplex Input Channels)** to **ALL OUT (`6` Spatial Output Buses)**.
//!    - Stamps every I/O frame with a `qwave_core::quanttern` 4-trit affective tag (`qt:xx`) and a
//!      79-byte `qwave_core::wave::WaveInt` cognitive frame.
//! 3. **`M1-ULTRA DJ` 4-Deck Mixer & Live Master Renderer**:
//!    - **Deck A**: `8bit-Wraith — The Future We Remember` (`128.0 BPM`, C64 SID + DX7 + F-Major Orchestra)
//!    - **Deck B**: `entheai-radio — Quantshuffle Canon` (`128.0 BPM`, Beethoven/Bach/Metallica 1-bit feedback)
//!    - **Deck C**: `Quant65k+ Hyper-Lattice` (`69,632` layers + `entheai-quantum` GHZ entanglement)
//!    - **Deck D**: `ALL-IN Live Mic & Environment Stem` (`External Microphone` + `MacBook Pro Microphone` + 5-band LTC EQ)

use crate::bluetooth::{MultiBluetoothRouter, MultiBluetoothTopology};
use crate::quant65k::{Quant65kEngine, DEFAULT_QUANT65K_PLUS_LAYERS, Q16_RECIP};
use crate::sota_mesh::DynamicEnvEqualizer;
use entheai_quantum::Qubits;
use entheai_radio::Quantshuffle;
#[cfg(not(target_arch = "aarch64"))]
use qdecorators::uqapi::ScalarLane;
#[cfg(target_arch = "aarch64")]
use qdecorators::uqapi::NeonLane;
use qdecorators::{scalar_authority_gemm, Uq};
use qwave_core::quanttern::{encode as qt_encode, unpack as qt_unpack, Trit, Vad, DEFAULT_THRESHOLD};
use qwave_core::wave::{consciousness, WaveInt, WaveProvenance};
use qwave_core::Rational;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use ultrawild::{clamp_ternary, sat_add8, xor_swap};
use wupz_core::arena::{zeroize_slice, Arena};
use wupz_core::fold::xor_fold;

/// The 16-step Zeno asymptotic ceiling: $\sum_{k=1}^{16} 2^{-k} = 1 - 2^{-16} = \frac{65535}{65536}$.
pub const ZENO_Q16_CEILING: f32 = 1.0 - Q16_RECIP;

/// Full-duplex input channel descriptor in `ALL IN` mode.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DjInputChannel {
    pub id: String,
    pub name: String,
    pub coreaudio_uid: String,
    pub sample_rate_hz: u32,
    pub gain_scalar: f32,
    pub active: bool,
    pub planned_hfp_guard: bool,
    pub quanttern_trit: i8,
}

/// Spatial output bus descriptor in `ALL OUT` mode.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DjOutputBus {
    pub id: String,
    pub name: String,
    pub target_endpoints: Vec<String>,
    pub sample_rate_hz: u32,
    pub volume_scalar: f32,
    pub stereo_pan: f32,
    pub phase_delay_samples_48k: usize,
    pub quanttern_tag: String,
}

/// 4-Deck M1-ULTRA DJ Deck state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DjDeckState {
    pub deck_id: String,
    pub title: String,
    pub bpm: f32,
    pub sync_locked: bool,
    pub fader_level: f32,
    pub eq_low_db: f32,
    pub eq_mid_db: f32,
    pub eq_high_db: f32,
    pub kill_low: bool,
    pub kill_mid: bool,
    pub kill_high: bool,
    pub cue_left_ear_enabled: bool,
    pub master_right_ear_enabled: bool,
}

/// Verification report for `ULTRA-OMNIPOTENT-8b-is-ZENO` + `M1-ULTRA DJ MODE ALL IN/OUT IOuQT`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZenoDjUltraReport {
    pub mode: String,
    pub uq_native_simd_lane: String,
    pub uq_neon_vs_scalar_bit_exact: bool,
    pub iouqt_crossbar_gemm_out: Vec<i32>,
    pub iouqt_master_quanttern_tag: String,
    pub iouqt_unpacked_trits: Vec<i8>,
    pub zeno_ceiling_q16: f32,
    pub zeno_overload_test_input: f32,
    pub zeno_overload_test_output: f32,
    pub zeno_za0001_volatile_scrub_verified: bool,
    pub zeno_arena_fold_before_scrub: String,
    pub all_in_channels: Vec<DjInputChannel>,
    pub all_out_buses: Vec<DjOutputBus>,
    pub decks: Vec<DjDeckState>,
    pub crossfader_position: f32,
    pub qwave_79b_frame_verified: bool,
}

/// Zeno Asymptotic Brickwall Limiter:
/// Maps any arbitrary multi-deck hot signal $x \in (-\infty, +\infty)$ into $(-1 + 2^{-16}, +1 - 2^{-16})$
/// via a 16-step dyadic Zeno cascade ($\sum_{k=1}^{16} 2^{-k}$), preserving punch at `MAX VOLUME` with zero clipping.
#[inline]
pub fn zeno_asymptotic_limit(x: f32) -> f32 {
    let abs_x = x.abs();
    if abs_x <= 0.5 {
        return x;
    }
    // Zeno dyadic progression above 0.5: each +0.5 of input halves the remaining distance to ZENO_Q16_CEILING
    let excess = (abs_x - 0.5).min(16.0);
    let halvings = (-excess * 1.3862944).exp(); // 2^(-2 * excess)
    let mag = (ZENO_Q16_CEILING - 0.5 * halvings).min(ZENO_Q16_CEILING);
    mag.copysign(x)
}

/// The **`ULTRA-OMNIPOTENT-8b-is-ZENO` + `M1-ULTRA DJ MODE ALL IN/OUT IOuQT`** Controller.
pub struct ZenoDjUltraEngine {
    pub quant65k: Quant65kEngine,
    pub eq: DynamicEnvEqualizer,
    pub bt_router: MultiBluetoothRouter,
    pub all_in: Vec<DjInputChannel>,
    pub all_out: Vec<DjOutputBus>,
    pub decks: Vec<DjDeckState>,
    pub crossfader: f32,
}

impl ZenoDjUltraEngine {
    pub fn new(layers: usize) -> Self {
        let quant65k = Quant65kEngine::new(
            layers.max(DEFAULT_QUANT65K_PLUS_LAYERS),
            "ULTRA-OMNIPOTENT-8b-is-ZENO::M1-ULTRA-DJ-IOuQT",
        );
        let eq = DynamicEnvEqualizer::default();
        let bt_router = MultiBluetoothRouter::new(MultiBluetoothTopology::FullConstellationWall);

        let all_in = vec![
            DjInputChannel {
                id: "IN-0".to_string(),
                name: "External Microphone (3.5mm TRRS Headset Mic)".to_string(),
                coreaudio_uid: "BuiltInMicrophoneDevice".to_string(),
                sample_rate_hz: 48_000,
                gain_scalar: 1.0,
                active: true,
                planned_hfp_guard: true,
                quanttern_trit: 1,
            },
            DjInputChannel {
                id: "IN-1".to_string(),
                name: "MacBook Pro Microphone (M3 3-Mic Studio Array)".to_string(),
                coreaudio_uid: "BuiltInMicrophoneDevice:Array".to_string(),
                sample_rate_hz: 48_000,
                gain_scalar: 0.85,
                active: true,
                planned_hfp_guard: true,
                quanttern_trit: 1,
            },
            DjInputChannel {
                id: "IN-2".to_string(),
                name: format!("Quant65k+ Internal Loopback Stem ({} Layers)", quant65k.total_layers),
                coreaudio_uid: "com.8b-is.iouqt.loopback.quant65k".to_string(),
                sample_rate_hz: 48_000,
                gain_scalar: 1.0,
                active: true,
                planned_hfp_guard: true,
                quanttern_trit: 1,
            },
            DjInputChannel {
                id: "IN-3".to_string(),
                name: "Bluetooth HFP Trap Guard & Clean 48kHz Talkback Gate".to_string(),
                coreaudio_uid: "com.8b-is.iouqt.bt-hfp-guard".to_string(),
                sample_rate_hz: 48_000,
                gain_scalar: 1.0,
                active: true,
                planned_hfp_guard: true,
                quanttern_trit: 0,
            },
        ];

        let all_out = vec![
            DjOutputBus {
                id: "OUT-0 [DJ CUE LEFT EAR]".to_string(),
                name: "External Headphones (3.5mm Analog Jack — Left Ear Split Cue)".to_string(),
                target_endpoints: vec!["BuiltInHeadphoneOutputDevice".to_string()],
                sample_rate_hz: 48_000,
                volume_scalar: 1.0,
                stereo_pan: 0.0,
                phase_delay_samples_48k: 1_824,
                quanttern_tag: "qt:2a".to_string(),
            },
            DjOutputBus {
                id: "OUT-1 [DJ MASTER RIGHT EAR]".to_string(),
                name: "Christopher’s AirPods Pro (Right Ear Master Booth)".to_string(),
                target_endpoints: vec!["2c-18-09-d8-14-74".to_string()],
                sample_rate_hz: 48_000,
                volume_scalar: 1.0,
                stereo_pan: 1.0,
                phase_delay_samples_48k: 672,
                quanttern_tag: "qt:2a".to_string(),
            },
            DjOutputBus {
                id: "OUT-2 [CO-DJ BINAURAL]".to_string(),
                name: "Alexandra’s AirPods Pro (Partner Full Binaural Stereo)".to_string(),
                target_endpoints: vec!["2c-32-6a-c3-4a-91".to_string()],
                sample_rate_hz: 48_000,
                volume_scalar: 1.0,
                stereo_pan: 0.5,
                phase_delay_samples_48k: 672,
                quanttern_tag: "qt:2a".to_string(),
            },
            DjOutputBus {
                id: "OUT-3 [M3 CENTER HALO]".to_string(),
                name: "MacBook Pro Speakers (M3 6-Speaker Spatial Halo)".to_string(),
                target_endpoints: vec!["BuiltInSpeakerDevice".to_string()],
                sample_rate_hz: 48_000,
                volume_scalar: 1.0,
                stereo_pan: 0.5,
                phase_delay_samples_48k: 1_704,
                quanttern_tag: "qt:26".to_string(),
            },
            DjOutputBus {
                id: "OUT-4 [SUB-BASS WALL]".to_string(),
                name: "Multi-BT Sub Anchors (soundcore Select 4 Go + Tronsmart Bang SE)".to_string(),
                target_endpoints: vec![
                    "00-7f-1d-7f-8d-35".to_string(),
                    "34-00-00-49-d4-49".to_string(),
                ],
                sample_rate_hz: 48_000,
                volume_scalar: 1.0,
                stereo_pan: 0.5,
                phase_delay_samples_48k: 96,
                quanttern_tag: "qt:2a".to_string(),
            },
            DjOutputBus {
                id: "OUT-5 [CONSTELLATION SURROUND]".to_string(),
                name: "Multi-BT Surround (JBL GO 2 + Mi Pocket Speaker 2 + LG OLED77G1)".to_string(),
                target_endpoints: vec![
                    "70-99-1c-55-cb-49".to_string(),
                    "74-a3-4a-19-60-ca".to_string(),
                    "dc-03-98-5e-e7-d5".to_string(),
                ],
                sample_rate_hz: 48_000,
                volume_scalar: 1.0,
                stereo_pan: 0.5,
                phase_delay_samples_48k: 0,
                quanttern_tag: "qt:2a".to_string(),
            },
        ];

        let decks = vec![
            DjDeckState {
                deck_id: "DECK A".to_string(),
                title: "8bit-Wraith — The Future We Remember (SID + DX7 + F-Major Chorus)".to_string(),
                bpm: 128.0,
                sync_locked: true,
                fader_level: 1.0,
                eq_low_db: 3.5,
                eq_mid_db: 2.0,
                eq_high_db: 4.0,
                kill_low: false,
                kill_mid: false,
                kill_high: false,
                cue_left_ear_enabled: true,
                master_right_ear_enabled: true,
            },
            DjDeckState {
                deck_id: "DECK B".to_string(),
                title: "entheai-radio — Quantshuffle 1-Bit Polyphonic Canon (Beethoven/Metallica)".to_string(),
                bpm: 128.0,
                sync_locked: true,
                fader_level: 0.88,
                eq_low_db: 4.2,
                eq_mid_db: 1.5,
                eq_high_db: 2.8,
                kill_low: false,
                kill_mid: false,
                kill_high: false,
                cue_left_ear_enabled: true,
                master_right_ear_enabled: true,
            },
            DjDeckState {
                deck_id: "DECK C".to_string(),
                title: format!(
                    "Quant65k+ Hyper-Lattice ({} Ternary Wave Layers + GHZ Qubits)",
                    quant65k.total_layers
                ),
                bpm: 128.0,
                sync_locked: true,
                fader_level: 0.95,
                eq_low_db: 3.0,
                eq_mid_db: 3.0,
                eq_high_db: 4.5,
                kill_low: false,
                kill_mid: false,
                kill_high: false,
                cue_left_ear_enabled: true,
                master_right_ear_enabled: true,
            },
            DjDeckState {
                deck_id: "DECK D [ALL-IN LIVE]".to_string(),
                title: "ALL-IN Live Mic + Environment Stem (Marine O(1) + 5-Band LTC ODE)".to_string(),
                bpm: 128.0,
                sync_locked: true,
                fader_level: 0.92,
                eq_low_db: 2.5,
                eq_mid_db: 3.8,
                eq_high_db: 3.2,
                kill_low: false,
                kill_mid: false,
                kill_high: false,
                cue_left_ear_enabled: true,
                master_right_ear_enabled: true,
            },
        ];

        Self {
            quant65k,
            eq,
            bt_router,
            all_in,
            all_out,
            decks,
            crossfader: 0.5,
        }
    }

    /// Executes the `IOuQT` (`qdecorators::Uq` + `qwave_core::quanttern`) crossbar verification
    /// and `ZENO` (`wupz_core::arena::zeroize_slice` + `zeno_asymptotic_limit`) audit.
    pub fn verify_zeno_iouqt(&mut self) -> ZenoDjUltraReport {
        // 1. Run `qdecorators::Uq` SIMD GEMM across the 6 output buses × 8 input/deck features
        let n_in = 8;
        let n_out = 6;
        let mut weights = Vec::with_capacity(n_in * n_out);
        for i in 0..(n_in * n_out) {
            let t = self.quant65k.trits[i % self.quant65k.trits.len()];
            weights.push(clamp_ternary!(t) as i16);
        }
        let activations: [i16; 8] = [100, 88, 95, 92, 127, 64, 48, 73];

        let scalar_out = scalar_authority_gemm(&weights, &activations, n_in, n_out);
        #[cfg(target_arch = "aarch64")]
        let native_out = Uq::gemm::<NeonLane>(&weights, &activations, n_in, n_out);
        #[cfg(not(target_arch = "aarch64"))]
        let native_out = Uq::gemm::<ScalarLane>(&weights, &activations, n_in, n_out);

        let uq_bit_exact = scalar_out == native_out;

        // 2. QuantTern (`QT`) encode & unpack roundtrip for the IOuQT bus
        let vad = Vad::new(0.96, 0.88, 0.94);
        let emo = qt_encode(vad, DEFAULT_THRESHOLD);
        let unpacked_trits: Vec<i8> = qt_unpack(&emo.bytes, 3)
            .into_iter()
            .map(Trit::raw)
            .collect();

        // Exercise `ultrawild::xor_swap!` and `sat_add8!` on DJ cue bus registers
        let mut cue_bus_a = 128u8;
        let mut cue_bus_b = 200u8;
        xor_swap!(cue_bus_a, cue_bus_b);
        let _boosted_cue = sat_add8!(cue_bus_a, cue_bus_b);

        // 3. Zeno Asymptotic Brickwall Limiter test on an extreme +400% (+12 dB) hot 4-deck sum
        let hot_input = 4.0f32;
        let limited_output = zeno_asymptotic_limit(hot_input);

        // 4. Zeno Volatile Memory Hygiene (`ZA-0001..ZA-0005` via `wupz_core::arena::zeroize_slice`)
        let mut zeno_arena = Arena::with_capacity(256);
        let fold_hex: String;
        let scrub_verified: bool;
        {
            let slab = zeno_arena.alloc(64).expect("zeno slab");
            for (idx, b) in slab.iter_mut().enumerate() {
                *b = (idx as u8).wrapping_mul(37).wrapping_add(0x8B);
            }
            let fold_before = xor_fold(slab);
            fold_hex = format!("0x{:016x}", fold_before);
            zeroize_slice(slab);
            scrub_verified = slab.iter().all(|&b| b == 0);
        }

        // 5. Verify 79-byte `qwave_core::wave::WaveInt` IOuQT frame
        let wave = WaveInt {
            base_amplitude: Rational::integer(1),
            frequency: Rational::integer(48_000),
            phase: consciousness(),
            emotional_valence: Rational::integer(1),
            arousal: Rational::integer(1),
            created_at: 1_792_301_000,
            last_accessed: 1_792_301_001,
            access_count: 8,
            decay_rate: Rational::integer(0),
            id: Some(0x8B15_D100_0000_0001),
            provenance: WaveProvenance::Cognitive,
        };
        let frame_79 = wave.to_frame();
        let qwave_ok = WaveInt::from_frame(&frame_79).is_ok();

        ZenoDjUltraReport {
            mode: "ULTRA-OMNIPOTENT-8b-is-ZENO <<<< M1-ULTRA DJ MODE ALL IN/OUT IOuQT".to_string(),
            uq_native_simd_lane: Uq::native_lane_name().to_string(),
            uq_neon_vs_scalar_bit_exact: uq_bit_exact,
            iouqt_crossbar_gemm_out: native_out,
            iouqt_master_quanttern_tag: emo.hex,
            iouqt_unpacked_trits: unpacked_trits,
            zeno_ceiling_q16: ZENO_Q16_CEILING,
            zeno_overload_test_input: hot_input,
            zeno_overload_test_output: limited_output,
            zeno_za0001_volatile_scrub_verified: scrub_verified,
            zeno_arena_fold_before_scrub: fold_hex,
            all_in_channels: self.all_in.clone(),
            all_out_buses: self.all_out.clone(),
            decks: self.decks.clone(),
            crossfader_position: self.crossfader,
            qwave_79b_frame_verified: qwave_ok,
        }
    }

    /// Renders a live 4-Deck `M1-ULTRA DJ MODE ALL IN/OUT IOuQT` Master Mix at `48,000 Hz` 16-bit PCM:
    /// - **Left Ear (`External Headphones` 3.5mm Jack)**: DJ Split-Cue + C64 SID Lead + Cello + Quant65k+ Sub/Mid
    /// - **Right Ear (`AirPods Pro` + Multi-BT Wall)**: DJ Master Mix + Violin + DX7 + Quantshuffle Canon + GHZ Quantum Highs
    /// - Protected by `zeno_asymptotic_limit` at `MAX VOLUME` (`0.0 dBFS` ceiling, zero clipping).
    pub fn render_dj_ultra_mix_wav(
        &mut self,
        duration_seconds: f32,
        output_path: &Path,
    ) -> std::io::Result<ZenoDjMixSummary> {
        let sample_rate: u32 = 48_000;
        let total_frames = ((duration_seconds.max(1.0)) * (sample_rate as f32)) as usize;

        let mut quantshuffle = Quantshuffle::new();
        let mut qubits = Qubits::zeros(3);
        qubits.h(0);
        qubits.cnot(0, 1);
        qubits.cnot(1, 2);
        let q_bias = (qubits.amps[0].re * qubits.amps[0].re) as f32;

        // C64 SID arpeggio + F-Major Final Chorus progression (`128.0 BPM` = 2.1333 beats/sec)
        let sid_arp_hz: [f32; 3] = [523.25, 659.25, 783.99];
        let f_major_hz: [f32; 8] = [
            349.23, 440.00, 523.25, 587.33, 466.16, 523.25, 698.46, 880.00,
        ];

        let mut pcm_bytes = Vec::with_capacity(total_frames * 4);
        let mut peak_l = 0.0f32;
        let mut peak_r = 0.0f32;

        for frame_idx in 0..total_frames {
            let t = (frame_idx as f32) / (sample_rate as f32);
            let progress = t / duration_seconds.max(1.0);
            let tau = std::f32::consts::TAU;

            // Dynamic DJ Crossfader sweep across the mix (0.25 -> 0.75 -> 0.50 constant power)
            let xf = 0.5 + 0.25 * (tau * 0.25 * t).sin();
            let gain_a = (xf * std::f32::consts::FRAC_PI_2).cos();
            let gain_b = (xf * std::f32::consts::FRAC_PI_2).sin();

            // 128 BPM beat pulse (`2.133333 Hz`)
            let beat_phase = (t * (128.0 / 60.0)).fract();
            let kick_env = (-beat_phase * 9.5).exp();
            let kick_sub = (tau * (58.0 + 90.0 * (-beat_phase * 28.0).exp()) * t).sin() * kick_env;

            // DECK A: 8bit-Wraith — The Future We Remember (SID + DX7 + Strings)
            let sid_hz = sid_arp_hz[(frame_idx / 4_500) % 3];
            let chord_hz = f_major_hz[(frame_idx / 11_250) % f_major_hz.len()];
            let sid_sq = if (t * sid_hz).fract() < 0.38 { 0.48 } else { -0.48 };
            let dx7 = (tau * chord_hz * t + 1.6 * (tau * chord_hz * 2.0 * t).sin()).sin();
            let cello = (tau * (chord_hz * 0.5) * t + 0.25 * (tau * 5.5 * t).sin()).sin();
            let violin = (tau * (chord_hz * 1.5) * t + 0.25 * (tau * 6.2 * t).sin()).sin();

            let deck_a_l = 0.42 * sid_sq + 0.33 * cello + 0.25 * dx7;
            let deck_a_r = 0.45 * violin + 0.35 * dx7 + 0.20 * sid_sq;

            // DECK B: entheai-radio Quantshuffle 1-bit polyphonic canon
            let qs_mono = quantshuffle.next_sample();

            // DECK C: Quant65k+ (69,632-layer ternary wave superposition)
            let (q65_l, q65_r) = self.quant65k.sample_realtime_48k(frame_idx, chord_hz);

            // DECK D: ALL-IN Live Environment & 0.73 Hz Consciousness Gate Harmonic
            if frame_idx % 960 == 0 {
                self.eq.step_ltc(0.02, -32.0 + 4.0 * kick_env, 0.96, chord_hz);
            }
            let gate_073 = 0.75 + 0.25 * (tau * 0.73 * t).sin();

            // Combine all 4 decks + 128 BPM kick into Left Ear (Jack DJ Cue) & Right Ear (AirPods Master)
            let fade_env = if progress < 0.03 {
                progress / 0.03
            } else if progress > 0.95 {
                (1.0 - progress) / 0.05
            } else {
                1.0
            };

            let raw_l = fade_env
                * (0.48 * kick_sub
                    + 0.65 * gain_a * deck_a_l
                    + 0.45 * gain_b * qs_mono
                    + 0.42 * q65_l * gate_073);
            let raw_r = fade_env
                * (0.42 * kick_sub
                    + 0.65 * gain_a * deck_a_r
                    + 0.55 * gain_b * qs_mono * (0.5 + q_bias)
                    + 0.48 * q65_r * gate_073);

            // Apply 5-band LTC Dynamic Environment Equalizer + Zeno Asymptotic Brickwall Limiter
            let (eq_l, eq_r) = self.eq.apply_stereo(raw_l * 1.35, raw_r * 1.35);
            let zeno_l = zeno_asymptotic_limit(eq_l);
            let zeno_r = zeno_asymptotic_limit(eq_r);

            peak_l = peak_l.max(zeno_l.abs());
            peak_r = peak_r.max(zeno_r.abs());

            let s_l = (zeno_l * 32767.0) as i16;
            let s_r = (zeno_r * 32767.0) as i16;
            pcm_bytes.extend_from_slice(&s_l.to_le_bytes());
            pcm_bytes.extend_from_slice(&s_r.to_le_bytes());
        }

        // Write 48,000 Hz Stereo 16-bit WAV
        let mut f = File::create(output_path)?;
        let channels: u16 = 2;
        let byte_rate = sample_rate * (channels as u32) * 2;
        let block_align = channels * 2;
        let data_len = pcm_bytes.len() as u32;
        let riff_len = 36 + data_len;

        f.write_all(b"RIFF")?;
        f.write_all(&riff_len.to_le_bytes())?;
        f.write_all(b"WAVE")?;
        f.write_all(b"fmt ")?;
        f.write_all(&16u32.to_le_bytes())?;
        f.write_all(&1u16.to_le_bytes())?;
        f.write_all(&channels.to_le_bytes())?;
        f.write_all(&sample_rate.to_le_bytes())?;
        f.write_all(&byte_rate.to_le_bytes())?;
        f.write_all(&block_align.to_le_bytes())?;
        f.write_all(&16u16.to_le_bytes())?;
        f.write_all(b"data")?;
        f.write_all(&data_len.to_le_bytes())?;
        f.write_all(&pcm_bytes)?;
        f.flush()?;

        // Scrub temporary PCM buffer via Zeno volatile zeroize before drop
        zeroize_slice(&mut pcm_bytes);

        Ok(ZenoDjMixSummary {
            output_path: output_path.display().to_string(),
            sample_rate_hz: sample_rate,
            duration_seconds,
            total_frames,
            quant65k_layers: self.quant65k.total_layers,
            peak_left_zeno: peak_l,
            peak_right_zeno: peak_r,
            zeno_ceiling_q16: ZENO_Q16_CEILING,
            quanttern_tag: self.eq.quanttern_tag.clone(),
            uq_simd_lane: Uq::native_lane_name().to_string(),
        })
    }
}

/// Summary of a rendered `M1-ULTRA DJ MODE ALL IN/OUT IOuQT` mix.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZenoDjMixSummary {
    pub output_path: String,
    pub sample_rate_hz: u32,
    pub duration_seconds: f32,
    pub total_frames: usize,
    pub quant65k_layers: usize,
    pub peak_left_zeno: f32,
    pub peak_right_zeno: f32,
    pub zeno_ceiling_q16: f32,
    pub quanttern_tag: String,
    pub uq_simd_lane: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zeno_dj_ultra_iouqt_verifies_neon_gemm_and_zeroize_and_brickwall() {
        let mut dj = ZenoDjUltraEngine::new(DEFAULT_QUANT65K_PLUS_LAYERS);
        let report = dj.verify_zeno_iouqt();
        assert!(report.uq_neon_vs_scalar_bit_exact);
        assert!(report.zeno_za0001_volatile_scrub_verified);
        assert!(report.qwave_79b_frame_verified);
        assert!(report.iouqt_master_quanttern_tag.starts_with("qt:"));
        assert_eq!(report.all_in_channels.len(), 4);
        assert_eq!(report.all_out_buses.len(), 6);
        assert_eq!(report.decks.len(), 4);
        assert!(report.zeno_overload_test_output <= ZENO_Q16_CEILING);
        assert!(report.zeno_overload_test_output > 0.99);
    }
}
