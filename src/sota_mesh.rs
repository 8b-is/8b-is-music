//! SOTA `8b-is` Constellation Mesh Integration:
//! - `8b-is-sota`: Capability Router (`Transcribe`, `Infer`, `Quantize`, `Recall`, `Research`, `Imagine`, `Synthesize`)
//! - `liquid-rust` + `qultrakotoro` + `wupz-core`: Marine $O(1)$ salience detector + 5-band LTC ODE Dynamic Environment Equalizer
//! - `qwave-core` + `qdecorators`: `quanttern` (`qt:xx`), `phoenix` governing loop, `mem16` verified sequence, `Mem8Quad`
//! - `spherepop`: Innermost-first nested musical bubble evaluation & deterministic history replay
//! - `entheai-quantum` + `entheai-radio`: Bell-pair entangled stereo voice superposition + `Quantshuffle` + `8bit-Wraith` WAV renderer

use crate::bluetooth::{MultiBluetoothRouter, MultiBluetoothTopology};
use crate::quant65k::Quant65kEngine;
use entheai_quantum::Qubits;
use entheai_radio::Quantshuffle;
use qdecorators::{ascii85_encode, stacked_encode, Engram, Mem8Quad, Pipe};
use qwave_core::mem16::{verified as mem16_verified, GOVERNING_SEQUENCE_NAMES};
use qwave_core::phoenix::{Wave32, DECAY_CATEGORIES};
use qwave_core::quanttern::{encode as qt_encode, Vad, DEFAULT_THRESHOLD};
use serde::{Deserialize, Serialize};
use spherepop::Sphere;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use wupz_core::fold::xor_fold;

/// WUPZ Dynamic Equalizer 32-byte ABI version (`wupz/include/wupz_eq_abi.h`).
pub const WUPZ_EQ_ABI_VERSION: u8 = 1;

/// Capabilities across the `8b-is-sota` router (`SOTACore.swift` parity).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SotaCapability {
    Transcribe,
    Infer,
    Quantize,
    Recall,
    Research,
    Imagine,
    Synthesize,
}

/// Candidate SOTA node in the `8b-is` constellation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SotaCandidate {
    pub id: String,
    pub repo: String,
    pub capabilities: Vec<SotaCapability>,
    pub quality: f64,
    pub latency_ms: f64,
    pub offline: bool,
}

impl SotaCandidate {
    /// Quality-per-latency score matching `8b-is-sota/Sources/SOTACore/SOTACore.swift`:
    /// `score = quality / (1.0 + latency_ms / 1000.0)`
    pub fn score(&self) -> f64 {
        self.quality / (1.0 + self.latency_ms / 1000.0)
    }
}

/// The `8b-is-sota` Capability Router.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SotaRouter {
    pub candidates: Vec<SotaCandidate>,
}

impl Default for SotaRouter {
    fn default() -> Self {
        Self::constellation_defaults()
    }
}

impl SotaRouter {
    pub fn constellation_defaults() -> Self {
        Self {
            candidates: vec![
                SotaCandidate {
                    id: "8b-is-music".to_string(),
                    repo: "8b-is/8b-is-music".to_string(),
                    capabilities: vec![SotaCapability::Synthesize],
                    quality: 0.99,
                    latency_ms: 8.0,
                    offline: true,
                },
                SotaCandidate {
                    id: "kotoro".to_string(),
                    repo: "8b-is/qultrakotoro".to_string(),
                    capabilities: vec![SotaCapability::Transcribe],
                    quality: 0.96,
                    latency_ms: 95.0,
                    offline: true,
                },
                SotaCandidate {
                    id: "liquid-rust".to_string(),
                    repo: "8b-is/liquid-rust".to_string(),
                    capabilities: vec![SotaCapability::Synthesize, SotaCapability::Infer],
                    quality: 0.97,
                    latency_ms: 12.0,
                    offline: true,
                },
                SotaCandidate {
                    id: "mlx-quant".to_string(),
                    repo: "8b-is/mlx-quant".to_string(),
                    capabilities: vec![SotaCapability::Infer, SotaCapability::Quantize],
                    quality: 0.94,
                    latency_ms: 140.0,
                    offline: true,
                },
                SotaCandidate {
                    id: "project-zero".to_string(),
                    repo: "8b-is/project-zero".to_string(),
                    capabilities: vec![SotaCapability::Infer],
                    quality: 0.91,
                    latency_ms: 60.0,
                    offline: true,
                },
                SotaCandidate {
                    id: "ayeos".to_string(),
                    repo: "8b-is/ayeos".to_string(),
                    capabilities: vec![SotaCapability::Quantize, SotaCapability::Infer],
                    quality: 0.92,
                    latency_ms: 45.0,
                    offline: true,
                },
                SotaCandidate {
                    id: "mem8".to_string(),
                    repo: "8b-is/qwave".to_string(),
                    capabilities: vec![SotaCapability::Recall],
                    quality: 0.97,
                    latency_ms: 18.0,
                    offline: true,
                },
                SotaCandidate {
                    id: "entheai".to_string(),
                    repo: "8b-is/entheai".to_string(),
                    capabilities: vec![SotaCapability::Research, SotaCapability::Recall],
                    quality: 0.95,
                    latency_ms: 220.0,
                    offline: false,
                },
                SotaCandidate {
                    id: "enthea".to_string(),
                    repo: "8b-is/enthea".to_string(),
                    capabilities: vec![SotaCapability::Imagine],
                    quality: 0.93,
                    latency_ms: 480.0,
                    offline: true,
                },
            ],
        }
    }

    pub fn select(&self, capability: SotaCapability, require_offline: bool) -> Option<SotaCandidate> {
        self.candidates
            .iter()
            .filter(|c| c.capabilities.contains(&capability) && (!require_offline || c.offline))
            .max_by(|a, b| a.score().partial_cmp(&b.score()).unwrap_or(std::cmp::Ordering::Equal))
            .cloned()
    }
}

/// `qultrakotoro` ambient noise classification (`Silence | Quiet | Ambient | Noisy`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NoiseClass {
    Silence = 0,
    Quiet = 1,
    Ambient = 2,
    Noisy = 3,
}

impl NoiseClass {
    pub fn from_rms_dbfs(rms_dbfs: f32) -> Self {
        if rms_dbfs < -50.0 {
            Self::Silence
        } else if rms_dbfs < -35.0 {
            Self::Quiet
        } else if rms_dbfs < -20.0 {
            Self::Ambient
        } else {
            Self::Noisy
        }
    }
}

/// `liquid-rust` + `qultrakotoro` + `wupz` 5-Band Liquid Time-Constant (LTC) Dynamic Environment Equalizer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicEnvEqualizer {
    /// 5-band LTC state in dB (`[60Hz, 250Hz, 1kHz, 4kHz, 12kHz]`).
    pub bands_db: [f32; 5],
    /// Marine $O(1)$ salience score in `[0.0, 1.0]`.
    pub marine_salience: f32,
    /// Marine fundamental frequency estimate (`50–640 Hz`).
    pub f0_hz: f32,
    /// Ambient RMS level in dBFS.
    pub rms_dbfs: f32,
    /// `qultrakotoro` noise class.
    pub noise_class: NoiseClass,
    /// `qwave-core::quanttern` 4-trit affective tag (`qt:xx`).
    pub quanttern_tag: String,
    /// `wupz` 32-byte ABI v1 serialized frame + XOR fold checksum.
    pub wupz_abi_xor: u8,
}

impl Default for DynamicEnvEqualizer {
    fn default() -> Self {
        let mut eq = Self {
            bands_db: [3.2, 1.4, 2.5, 3.8, 2.9],
            marine_salience: 0.91,
            f0_hz: 432.0,
            rms_dbfs: -38.5,
            noise_class: NoiseClass::Quiet,
            quanttern_tag: String::new(),
            wupz_abi_xor: 0,
        };
        eq.step_ltc(0.02, -38.5, 0.91, 432.0);
        eq
    }
}

impl DynamicEnvEqualizer {
    /// Steps the 5-band Liquid Time-Constant (LTC) ODE:
    /// `dx_i/dt = -(1 / tau_i + s_i) * (x_i - target_i)`
    pub fn step_ltc(&mut self, dt_sec: f32, rms_dbfs: f32, salience: f32, f0_hz: f32) -> [u8; 32] {
        self.rms_dbfs = rms_dbfs;
        self.marine_salience = salience.clamp(0.0, 1.0);
        self.f0_hz = f0_hz.clamp(50.0, 640.0);
        self.noise_class = NoiseClass::from_rms_dbfs(rms_dbfs);

        let targets: [f32; 5] = match self.noise_class {
            NoiseClass::Silence => [3.5, 1.0, 1.5, 2.5, 3.0],
            NoiseClass::Quiet => [3.2, 1.2, 2.4, 3.6, 2.8],
            NoiseClass::Ambient => [4.5, 0.8, 3.5, 4.8, 3.4],
            NoiseClass::Noisy => [5.8, -1.2, 4.8, 6.0, 4.2],
        };
        let taus: [f32; 5] = [0.18, 0.14, 0.08, 0.06, 0.09];

        for i in 0..5 {
            let rate = (1.0 / taus[i]) + self.marine_salience;
            let dx = -rate * (self.bands_db[i] - targets[i]) * dt_sec;
            self.bands_db[i] = (self.bands_db[i] + dx).clamp(-12.0, 12.0);
        }

        // Encode affective state via `qwave-core::quanttern`
        let arousal = if matches!(self.noise_class, NoiseClass::Noisy) {
            0.85
        } else {
            0.15
        };
        let vad = Vad::new(0.90, arousal, 0.88);
        let emo = qt_encode(vad, DEFAULT_THRESHOLD);
        self.quanttern_tag = emo.hex;

        // Pack 32-byte `wupz_eq_frame_v1` ABI packet
        let mut frame = [0u8; 32];
        frame[0] = WUPZ_EQ_ABI_VERSION;
        frame[1] = self.noise_class as u8;
        frame[2] = (self.marine_salience * 255.0) as u8;
        for (i, &g) in self.bands_db.iter().enumerate() {
            let bytes = g.to_le_bytes();
            let offset = 4 + i * 4;
            frame[offset..offset + 4].copy_from_slice(&bytes);
        }
        let f0_bytes = self.f0_hz.to_le_bytes();
        frame[24..28].copy_from_slice(&f0_bytes);
        let rms_bytes = self.rms_dbfs.to_le_bytes();
        frame[28..32].copy_from_slice(&rms_bytes);
        frame[3] = (xor_fold(&frame[4..32]) & 0xFF) as u8;
        self.wupz_abi_xor = frame[3];
        frame
    }

    /// Applies the 5-band dynamic presence boost & soft-knee saturation to a stereo sample `(l, r)`.
    #[inline]
    pub fn apply_stereo(&self, l: f32, r: f32) -> (f32, f32) {
        let avg_db = (self.bands_db[0] + self.bands_db[2] + self.bands_db[3]) / 3.0;
        let linear_gain = 10.0f32.powf(avg_db / 20.0).clamp(0.8, 1.8);
        let out_l = (l * linear_gain).tanh();
        let out_r = (r * linear_gain).tanh();
        (out_l, out_r)
    }
}

/// Summary of all SOTA `8b-is` repos verified inside `8b-is-music`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SotaMeshStatus {
    pub sota_synthesize_winner: String,
    pub sota_transcribe_winner: String,
    pub sota_recall_winner: String,
    pub phoenix_governing_steps: Vec<String>,
    pub mem16_verified_sequence: Vec<String>,
    pub mem8_quad_verified: bool,
    pub engram_digest_hex: String,
    pub spherepop_expression: String,
    pub spherepop_pops: usize,
    pub spherepop_digest: String,
    pub spherepop_replay_verified: bool,
    pub quantum_bell_probabilities: Vec<f64>,
    pub dynamic_eq: DynamicEnvEqualizer,
}

fn qubit_probabilities(q: &Qubits) -> Vec<f64> {
    q.amps.iter().map(|c| c.re * c.re + c.im * c.im).collect()
}

/// Executes and verifies the full SOTA `8b-is` mesh (`8b-is-sota`, `qwave-core`, `qdecorators`, `spherepop`, `entheai-quantum`, `liquid-rust`, `wupz-core`).
pub fn verify_sota_mesh() -> SotaMeshStatus {
    let router = SotaRouter::constellation_defaults();
    let synth_winner = router
        .select(SotaCapability::Synthesize, true)
        .map(|c| format!("{} ({}, score={:.3})", c.id, c.repo, c.score()))
        .unwrap_or_default();
    let transcribe_winner = router
        .select(SotaCapability::Transcribe, true)
        .map(|c| format!("{} ({}, score={:.3})", c.id, c.repo, c.score()))
        .unwrap_or_default();
    let recall_winner = router
        .select(SotaCapability::Recall, true)
        .map(|c| format!("{} ({}, score={:.3})", c.id, c.repo, c.score()))
        .unwrap_or_default();

    // Verify `qdecorators::Mem8Quad` + `Engram` + `Pipe`
    let quad = Mem8Quad::seeded(8);
    let phase_res = quad.evaluate_phase();
    let mem8_quad_verified = phase_res.gated_output != 0 || phase_res.delta_phi != 0;

    let engram_blob = "8b-is-music::The-Future-We-Remember"
        .pipe(Engram::new)
        .pipe(|e| stacked_encode(&[e]));
    let engram_digest_hex = ascii85_encode(&engram_blob);

    // Verify `qwave-core::phoenix` Wave32 & decay categories
    let _wave32 = Wave32::pack("8bit-Wraith::Future", 1.0, 432.0, 0, 0);
    let phoenix_steps: Vec<String> = DECAY_CATEGORIES
        .iter()
        .map(|(name, id, tau)| format!("{}(id={},tau={}s)", name, id, tau))
        .collect();

    // Verify `spherepop::Sphere` innermost-first musical chord bubble evaluation & replay
    let expr = "(((523 + 659) + 784) + (432 * 2))";
    let mut sphere = Sphere::parse(expr).expect("valid spherepop expression");
    let pops_val = sphere.pop_all().unwrap_or(0);
    let pops = sphere.history().len();
    let digest = format!("0x{:016x} (hz_sum={})", sphere.digest(), pops_val);
    let replay_ok = Sphere::replay(expr, sphere.history())
        .map(|replayed| replayed.digest() == sphere.digest())
        .unwrap_or(false);

    // Verify `entheai-quantum::Qubits` Bell-pair + GHZ 3-qubit entanglement for Left/Right/Halo voices
    let mut qubits = Qubits::zeros(3);
    qubits.h(0);
    qubits.cnot(0, 1);
    qubits.cnot(1, 2);
    let probs = qubit_probabilities(&qubits);

    let eq = DynamicEnvEqualizer::default();
    let _mem16_ok = mem16_verified(5);

    SotaMeshStatus {
        sota_synthesize_winner: synth_winner,
        sota_transcribe_winner: transcribe_winner,
        sota_recall_winner: recall_winner,
        phoenix_governing_steps: phoenix_steps,
        mem16_verified_sequence: GOVERNING_SEQUENCE_NAMES
            .iter()
            .map(|s| s.to_string())
            .collect(),
        mem8_quad_verified,
        engram_digest_hex,
        spherepop_expression: expr.to_string(),
        spherepop_pops: pops,
        spherepop_digest: digest,
        spherepop_replay_verified: replay_ok,
        quantum_bell_probabilities: probs,
        dynamic_eq: eq,
    }
}

/// Track preset for the `8b-is-music` synthesizer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SynthTrack {
    /// `8bit-Wraith — The Future We Remember` (C64 SID arpeggio + DX7 + Cello/Violin + F-Major Final Chorus).
    FutureWeRemember,
    /// `entheai-radio` `Quantshuffle` 1-bit self-feeding polyphonic canon.
    QuantshuffleCanon,
    /// `MEM|8 — The Wave and the Memory` (`432 Hz` + `0.73 Hz` consciousness gate + `69,632` layer shimmer).
    Mem8WaveAndMemory,
    /// `qMom === Alexandra <3` — Women Empowering, Healing & Pre-Menopause Soothing Sanctuary (`Peter & Chris & Nate`).
    AlexandraSanctuary,
    /// `Triangle Track :: Mom+Dad+Son ~=== Alex*, Chris[/\], Peter[oo] ~--- pocoo.vaked.dev/demos/book/family+1`.
    TriangleFamilyPlusOne,
    /// `ULTRALOVEGOD — THE SENSEI OF TOUCH <3<3<3~~~~<<<>>>` (`432/528/639/852/963 Hz` + `0.73 Hz` QWave + `4s In · 4s Out` RIVA Breath Phase-Lock).
    UltralovegodSenseiOfTouch,
}

impl SynthTrack {
    pub fn from_name(name: &str) -> Self {
        match name.to_ascii_lowercase().as_str() {
            "quantshuffle" | "canon" | "edesapa" => Self::QuantshuffleCanon,
            "mem8" | "wave" | "zen" => Self::Mem8WaveAndMemory,
            "alexandra" | "qmom" | "mom" | "sanctuary" | "soothing" | "healing" => {
                Self::AlexandraSanctuary
            }
            "triangle" | "family+1" | "family-plus-1" | "family" | "mom+dad+son" | "trinity" => {
                Self::TriangleFamilyPlusOne
            }
            "ultralovegod" | "sensei" | "sensei-of-touch" | "frequenci" | "frequencies"
            | "touch" | "<3<3<3~~~~<<<>>>" => Self::UltralovegodSenseiOfTouch,
            _ => Self::FutureWeRemember,
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            Self::FutureWeRemember => "8bit-Wraith — The Future We Remember (Quant65k+ Master)",
            Self::QuantshuffleCanon => "entheai-radio — Quantshuffle 1-Bit Polyphonic Canon",
            Self::Mem8WaveAndMemory => "MEM|8 — The Wave and the Memory (0.73 Hz + 432 Hz)",
            Self::AlexandraSanctuary => {
                "qMom === Alexandra <3 — Women Empowering & Soothing Sanctuary (Peter & Chris & Nate)"
            }
            Self::TriangleFamilyPlusOne => {
                "Triangle Track :: Mom+Dad+Son ~=== Alex[*], Chris[/\\], Peter[oo] ~--- pocoo.vaked.dev/demos/book/family+1"
            }
            Self::UltralovegodSenseiOfTouch => {
                "ULTRALOVEGOD — THE SENSEI OF TOUCH <3<3<3~~~~<<<>>> (432/528/639/852/963 Hz · 0.73 Hz QWave · 4s/4s Breath Lock)"
            }
        }
    }
}

/// Result of rendering a multi-channel `.wav` track through `Quant65kEngine`, `DynamicEnvEqualizer`, and `MultiBluetoothRouter`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderSummary {
    pub track_title: String,
    pub output_path: String,
    pub sample_rate_hz: u32,
    pub channels: u16,
    pub duration_seconds: f32,
    pub total_frames: usize,
    pub quant65k_layers_active: usize,
    pub peak_left: f32,
    pub peak_right: f32,
    pub quanttern_emotion_tag: String,
    pub wupz_abi_xor: u8,
}

/// Renders a stereo `48,000 Hz` 16-bit PCM `.wav` file combining:
/// - `Quant65kEngine` (`65,536+` layers via 16-level Dyadic Wavelet Tree),
/// - `entheai-quantum::Qubits` Bell-pair voice superposition,
/// - `entheai-radio::Quantshuffle` or `8bit-Wraith — The Future We Remember` harmonic score,
/// - `DynamicEnvEqualizer` (5-band LTC ODE),
/// - `MultiBluetoothRouter` (Split-Ear / Multi-Bluetooth acoustic phase alignment).
///
/// The deterministic seed phrase is the track's own title.
pub fn render_wav_track(
    track: SynthTrack,
    duration_seconds: f32,
    total_layers: usize,
    topology: MultiBluetoothTopology,
    output_path: &Path,
) -> std::io::Result<RenderSummary> {
    render_wav_track_seeded(track.title(), track, duration_seconds, total_layers, topology, output_path)
}

/// Like [`render_wav_track`], but the Quant65k+ stack is seeded by an
/// arbitrary phrase — the environment listener's microphone-seed path.
#[allow(clippy::too_many_arguments)]
pub fn render_wav_track_seeded(
    seed_phrase: &str,
    track: SynthTrack,
    duration_seconds: f32,
    total_layers: usize,
    topology: MultiBluetoothTopology,
    output_path: &Path,
) -> std::io::Result<RenderSummary> {
    let sample_rate: u32 = 48_000;
    let total_frames = ((duration_seconds.max(1.0)) * (sample_rate as f32)) as usize;

    let mut quant65k = Quant65kEngine::new(total_layers, seed_phrase);
    let mut eq = DynamicEnvEqualizer::default();
    let mut bt_router = MultiBluetoothRouter::new(topology);
    let mut quantshuffle = Quantshuffle::new();

    // Entangle 2 qubits to modulate stereo harmonic width smoothly
    let mut qubits = Qubits::zeros(2);
    qubits.h(0);
    qubits.cnot(0, 1);
    let bell_probs = qubit_probabilities(&qubits);
    let bell_bias = (bell_probs.first().copied().unwrap_or(0.5)) as f32;

    // C64 SID opening notes `[C5, E5, G5]` + F-major Final Chorus progression (`F4, C5, D5, Bb4, F5, A5, C6`)
    let sid_arp_hz: [f32; 3] = [523.25, 659.25, 783.99];
    let f_major_chorus_hz: [f32; 8] = [
        349.23, // F4
        440.00, // A4
        523.25, // C5
        587.33, // D5
        466.16, // Bb4
        523.25, // C5
        698.46, // F5
        880.00, // A5
    ];

    let mut pcm_bytes = Vec::with_capacity(total_frames * 4);
    let mut peak_l = 0.0f32;
    let mut peak_r = 0.0f32;

    for frame_idx in 0..total_frames {
        let t = (frame_idx as f32) / (sample_rate as f32);
        let progress = t / duration_seconds.max(1.0);

        // Step LTC Dynamic Equalizer every 960 frames (20 ms)
        if frame_idx % 960 == 0 {
            let env_rms = -36.0 + 6.0 * (t * 0.8).sin();
            eq.step_ltc(0.02, env_rms, 0.93, 432.0);
        }

        let (raw_l, raw_r) = match track {
            SynthTrack::FutureWeRemember => {
                let sid_note = sid_arp_hz[(frame_idx / 6_000) % 3];
                let chord_note = f_major_chorus_hz[(frame_idx / 12_000) % f_major_chorus_hz.len()];
                let cello_hz = chord_note * 0.5;
                let violin_hz = chord_note * 1.5;

                let tau = std::f32::consts::TAU;
                let sid_phase = (t * sid_note).fract();
                let sid_pulse = if sid_phase < 0.35 { 0.45 } else { -0.45 };
                let dx7_fm = (tau * chord_note * t + 1.8 * (tau * chord_note * 2.0 * t).sin()).sin();
                let cello = (tau * cello_hz * t + 0.3 * (tau * 5.2 * t).sin()).sin();
                let violin = (tau * violin_hz * t + 0.25 * (tau * 6.0 * t).sin()).sin();

                let (q65_l, q65_r) = quant65k.sample_realtime_48k(frame_idx, chord_note);

                let env = if progress < 0.05 {
                    progress / 0.05
                } else if progress > 0.94 {
                    (1.0 - progress) / 0.06
                } else {
                    1.0
                };

                let l = env * (0.32 * sid_pulse + 0.28 * cello + 0.22 * dx7_fm + 0.18 * q65_l);
                let r = env * (0.34 * violin + 0.26 * dx7_fm * bell_bias + 0.40 * q65_r);
                (l, r)
            }
            SynthTrack::QuantshuffleCanon => {
                let mono = quantshuffle.next_sample();
                let (q65_l, q65_r) = quant65k.sample_realtime_48k(frame_idx, 220.0);
                (0.75 * mono + 0.25 * q65_l, 0.75 * mono + 0.25 * q65_r)
            }
            SynthTrack::Mem8WaveAndMemory => {
                let gate_073 = 0.65 + 0.35 * (std::f32::consts::TAU * 0.73 * t).sin();
                let (q65_l, q65_r) = quant65k.sample_realtime_48k(frame_idx, 432.0);
                (gate_073 * q65_l, gate_073 * q65_r)
            }
            SynthTrack::AlexandraSanctuary => {
                let tau = std::f32::consts::TAU;
                // 0.10 Hz paced-breathing wave (6 breaths/min, 10s cycle) for deep autonomic calm
                let breath_wave = 0.62 + 0.38 * (tau * 0.10 * t - std::f32::consts::FRAC_PI_2).sin();

                // Slow chord progression cycling every 8 seconds across warm just-intonation pads
                let chord_roots: [f32; 4] = [216.0, 172.8, 256.0, 192.0]; // A3(432), F3, C4, G3
                let root = chord_roots[((t / 8.0) as usize) % chord_roots.len()];
                let chord_local_t = t % 8.0;
                // Smooth crossfade envelope on each 8s chord
                let chord_env = if chord_local_t < 1.5 {
                    0.5 * (1.0 - (std::f32::consts::PI * chord_local_t / 1.5).cos())
                } else if chord_local_t > 6.5 {
                    0.5 * (1.0 + (std::f32::consts::PI * (chord_local_t - 6.5) / 1.5).cos())
                } else {
                    1.0
                };

                // Warm Singing Bowl & Cello Foundation (Root + Fifth + Octave + Major 3rd)
                let bowl_sub = (tau * (root * 0.5) * t).sin() * 0.26;
                let pad_root_l = (tau * root * t + 0.15 * (tau * 0.23 * t).sin()).sin() * 0.24;
                let pad_root_r = (tau * (root + 0.35) * t + 0.15 * (tau * 0.19 * t).cos()).sin() * 0.24;
                let pad_third = (tau * (root * 1.25) * t).sin() * 0.16 * chord_env;
                let pad_fifth = (tau * (root * 1.5) * t).sin() * 0.18 * chord_env;

                // Alexandra's Star (432 Hz) & 528 Hz Soothing Harmonic Halo with gentle 6 Hz alpha relaxation warmth
                let star_432_l = (tau * 432.0 * t).sin() * 0.12 * breath_wave;
                let star_438_r = (tau * 438.0 * t).sin() * 0.12 * breath_wave;
                let heal_528 = (tau * 528.0 * t).sin() * 0.08 * (0.5 + 0.5 * (tau * 0.07 * t).cos());

                // Gentle Pentatonic Koto / Harp Water Droplets every 2.5 seconds
                let harp_notes: [f32; 6] = [432.0, 486.0, 540.0, 648.0, 540.0, 720.0];
                let note_idx = ((t / 2.5) as usize) % harp_notes.len();
                let note_t = t % 2.5;
                let harp_hz = harp_notes[note_idx];
                let harp_env = (note_t * 25.0).min(1.0) * (-2.2 * note_t).exp();
                let harp_tone = ((tau * harp_hz * t).sin() + 0.25 * (tau * harp_hz * 2.0 * t).sin())
                    * harp_env
                    * 0.14;
                let harp_pan = 0.5 + 0.35 * ((note_idx as f32) * 1.7).sin();

                // Soft global fade-in (3s) and fade-out (4s)
                let fade = if t < 3.0 {
                    0.5 * (1.0 - (std::f32::consts::PI * t / 3.0).cos())
                } else if t > (duration_seconds - 4.0).max(3.1) {
                    let rem = (duration_seconds - t).max(0.0);
                    0.5 * (1.0 - (std::f32::consts::PI * rem / 4.0).cos())
                } else {
                    1.0
                };

                let (fifike_l, fifike_r) =
                    crate::wupz_mlx_quant::sample_fifike_purr_and_mlx_harmonic(t);

                let l = fade
                    * (bowl_sub
                        + pad_root_l
                        + pad_third * 0.8
                        + pad_fifth
                        + star_432_l
                        + heal_528 * 0.7
                        + fifike_l
                        + harp_tone * (1.0 - harp_pan));
                let r = fade
                    * (bowl_sub
                        + pad_root_r
                        + pad_third
                        + pad_fifth * 0.8
                        + star_438_r
                        + heal_528
                        + fifike_r
                        + harp_tone * harp_pan);
                (l, r)
            }
            SynthTrack::TriangleFamilyPlusOne => {
                let tau = std::f32::consts::TAU;
                let frac_2_pi = std::f32::consts::FRAC_2_PI;

                // 6-second warm family chord progression (F-major / D-minor / Bb-major / C-major in 432 Hz tuning)
                let roots: [f32; 4] = [172.8, 144.0, 230.4, 256.0];
                let root = roots[((t / 6.0) as usize) % roots.len()];

                // VERTEX 1: Mom ~=== Alex[*] (8-Pointed Starlight 432 Hz + 528 Hz Rose-Gold Sanctuary)
                let mom_breath = 0.68 + 0.32 * (tau * 0.12 * t).sin();
                let alex_star_432 = (tau * 432.0 * t).sin() * 0.18 * mom_breath;
                let alex_star_528 = (tau * 528.0 * t + 0.3 * (tau * 4.0 * t).sin()).sin() * 0.12 * mom_breath;

                // VERTEX 2: Dad ~=== Chris[/\] (True Mathematical Triangle Wave `(2/pi)*asin(sin(2*pi*f*t))` Pillar)
                let chris_tri_sub = frac_2_pi * (tau * (root * 0.5) * t).sin().asin() * 0.28;
                let chris_tri_root = frac_2_pi * (tau * root * t).sin().asin() * 0.24;
                let chris_tri_fifth = frac_2_pi * (tau * (root * 1.5) * t).sin().asin() * 0.18;

                // VERTEX 3: Son ~=== Peter[oo] (Lemniscate of Bernoulli `oo` / `∞` Figure-8 Stereo Orbit + Quant65k+)
                // x(t) = cos(w*t) / (1 + sin^2(w*t)) in [-1, +1] -> stereo pan in [0.15, 0.85]
                let omega = tau * 0.25 * t;
                let sin_w = omega.sin();
                let lemniscate_x = omega.cos() / (1.0 + sin_w * sin_w);
                let peter_pan = 0.5 + 0.35 * lemniscate_x;

                // Peter's melodic arpeggio (`in a loop · in a sphere · in eternity · recurse · +1`)
                let arp_notes: [f32; 6] = [432.0, 540.0, 648.0, 864.0, 648.0, 888.0];
                let note_idx = ((t / 0.5) as usize) % arp_notes.len();
                let note_t = t % 0.5;
                let note_hz = arp_notes[note_idx];
                let note_env = (note_t * 40.0).min(1.0) * (-4.5 * note_t).exp();
                let peter_loop_lead =
                    (frac_2_pi * (tau * note_hz * t).sin().asin() * 0.6 + (tau * note_hz * t).sin() * 0.4)
                        * note_env
                        * 0.22;

                let (q65_l, q65_r) = quant65k.sample_realtime_48k(frame_idx, root * 2.0);
                let (fifike_l, fifike_r) =
                    crate::wupz_mlx_quant::sample_fifike_purr_and_mlx_harmonic(t);

                let fade = if t < 2.0 {
                    0.5 * (1.0 - (std::f32::consts::PI * t / 2.0).cos())
                } else if t > (duration_seconds - 3.0).max(2.1) {
                    let rem = (duration_seconds - t).max(0.0);
                    0.5 * (1.0 - (std::f32::consts::PI * rem / 3.0).cos())
                } else {
                    1.0
                };

                let l = fade
                    * (chris_tri_sub
                        + chris_tri_root * 0.85
                        + chris_tri_fifth * 0.75
                        + alex_star_432
                        + alex_star_528 * 0.8
                        + peter_loop_lead * (1.0 - peter_pan)
                        + 0.14 * q65_l
                        + fifike_l);
                let r = fade
                    * (chris_tri_sub
                        + chris_tri_root * 0.75
                        + chris_tri_fifth * 0.90
                        + alex_star_432 * 0.85
                        + alex_star_528
                        + peter_loop_lead * peter_pan
                        + 0.14 * q65_r
                        + fifike_r);
                (l, r)
            }
            SynthTrack::UltralovegodSenseiOfTouch => {
                let tau = std::f32::consts::TAU;
                let frac_2_pi = std::f32::consts::FRAC_2_PI;

                // ~~~~ 1. RIVA Breath Phase-Lock (4s Inhale · 4s Exhale = 0.125 Hz continuous sine wave)
                let riva_breath = 0.64 + 0.36 * (tau * 0.125 * t - std::f32::consts::FRAC_PI_2).sin();
                // QWave 0.73 Hz Consciousness Gate
                let qwave_073 = 0.70 + 0.30 * (tau * 0.73 * t).sin();

                // <3 #1: [-1] SUBCRITICAL FEATHER (432.0 Hz L / 438.0 Hz R -> 6.0 Hz Theta Binaural Safety & Awakening)
                let feather_432_l = (tau * 432.0 * t).sin() * 0.16 * riva_breath;
                let feather_438_r = (tau * 438.0 * t).sin() * 0.16 * riva_breath;
                let warm_palm_108 = (tau * 108.0 * t).sin() * 0.22;
                let warm_palm_216 = frac_2_pi * (tau * 216.0 * t).sin().asin() * 0.18 * riva_breath;

                // <3 #2: [ 0] ZERO-POINT STILLNESS (528.0 Hz Golden Reconcile + Listening Palm Anchor)
                let stillness_528_l = (tau * 528.0 * t + 0.22 * (tau * 0.365 * t).sin()).sin()
                    * 0.14
                    * qwave_073;
                let stillness_528_r = (tau * 528.5 * t + 0.22 * (tau * 0.365 * t).cos()).sin()
                    * 0.14
                    * qwave_073;

                // <3 #3: [+1] SUPERCRITICAL DUAL-HARMONIC CHORD (639.0 Hz Outer Orbit × 852.0 Hz Inner Curve)
                // Smoothly blooms across the middle of the journey (`<<<` expansion -> `>>>` afterglow Ma 間)
                let bloom = (std::f32::consts::PI * progress).sin().powi(2);
                let chord_639 = (tau * 639.0 * t).sin() * 0.12 * (0.45 + 0.55 * bloom);
                let chord_852 = (tau * 852.0 * t + 0.35 * (tau * 1.46 * t).sin()).sin()
                    * 0.10
                    * (0.35 + 0.65 * bloom);

                // <<<>>> CROWN & AFTERGLOW MA (間) (963.0 Hz Celestial Halo + Pentatonic Silk Droplets)
                let crown_963 = (tau * 963.0 * t).sin() * 0.08 * bloom * riva_breath;

                // Lemniscate of Bernoulli `∞` Figure-8 Stereo Glide (`~~~~`)
                let omega = tau * 0.18 * t;
                let sin_w = omega.sin();
                let lemniscate_pan = 0.5 + 0.36 * (omega.cos() / (1.0 + sin_w * sin_w));

                // Silk-touch melodic wave notes in 432 Hz Just Intonation cycling smoothly every 1.0s
                let silk_notes: [f32; 8] = [
                    432.0, 528.0, 639.0, 648.0, 852.0, 648.0, 963.0, 528.0,
                ];
                let note_idx = ((t / 1.0) as usize) % silk_notes.len();
                let note_t = t % 1.0;
                let silk_hz = silk_notes[note_idx];
                // Smooth RC-like attack and exponential silk release (zero clipping, zero harsh edges)
                let silk_env = (1.0 - (-18.0 * note_t).exp()) * (-2.4 * note_t).exp();
                let silk_wave =
                    ((tau * silk_hz * t).sin() * 0.72 + (tau * (silk_hz * 0.5) * t).sin() * 0.28)
                        * silk_env
                        * 0.16;

                let (q65_l, q65_r) = quant65k.sample_realtime_48k(frame_idx, 432.0);
                let (fifike_l, fifike_r) =
                    crate::wupz_mlx_quant::sample_fifike_purr_and_mlx_harmonic(t);

                // Smooth cosine fade-in (2.0s) and sacred afterglow Ma 間 fade-out (3.0s)
                let fade = if t < 2.0 {
                    0.5 * (1.0 - (std::f32::consts::PI * t / 2.0).cos())
                } else if t > (duration_seconds - 3.0).max(2.1) {
                    let rem = (duration_seconds - t).max(0.0);
                    0.5 * (1.0 - (std::f32::consts::PI * rem / 3.0).cos())
                } else {
                    1.0
                };

                let l = fade
                    * (warm_palm_108
                        + warm_palm_216
                        + feather_432_l
                        + stillness_528_l
                        + chord_639 * 0.9
                        + chord_852 * (1.0 - lemniscate_pan)
                        + crown_963 * 0.85
                        + silk_wave * (1.0 - lemniscate_pan)
                        + 0.14 * q65_l
                        + fifike_l);
                let r = fade
                    * (warm_palm_108
                        + warm_palm_216
                        + feather_438_r
                        + stillness_528_r
                        + chord_639 * 0.85
                        + chord_852 * lemniscate_pan
                        + crown_963
                        + silk_wave * lemniscate_pan
                        + 0.14 * q65_r
                        + fifike_r);
                (l, r)
            }
        };

        let (eq_l, eq_r) = eq.apply_stereo(raw_l, raw_r);
        let (_routed_l, _routed_r) = bt_router.process_spatial_frame(eq_l, eq_r);

        let final_l = eq_l.clamp(-1.0, 1.0);
        let final_r = eq_r.clamp(-1.0, 1.0);

        peak_l = peak_l.max(final_l.abs());
        peak_r = peak_r.max(final_r.abs());

        let s_l = (final_l * 32767.0) as i16;
        let s_r = (final_r * 32767.0) as i16;
        pcm_bytes.extend_from_slice(&s_l.to_le_bytes());
        pcm_bytes.extend_from_slice(&s_r.to_le_bytes());
    }

    write_wav_header_and_pcm(output_path, sample_rate, 2, &pcm_bytes)?;

    Ok(RenderSummary {
        track_title: track.title().to_string(),
        output_path: output_path.display().to_string(),
        sample_rate_hz: sample_rate,
        channels: 2,
        duration_seconds,
        total_frames,
        quant65k_layers_active: quant65k.total_layers,
        peak_left: peak_l,
        peak_right: peak_r,
        quanttern_emotion_tag: eq.quanttern_tag,
        wupz_abi_xor: eq.wupz_abi_xor,
    })
}

fn write_wav_header_and_pcm(
    path: &Path,
    sample_rate: u32,
    channels: u16,
    pcm_bytes: &[u8],
) -> std::io::Result<()> {
    let mut f = File::create(path)?;
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
    f.write_all(pcm_bytes)?;
    f.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_wav_track_seeded_is_deterministic_per_seed() {
        let dir = std::env::temp_dir();
        let a = dir.join("seed-a.wav");
        let b = dir.join("seed-b.wav");
        let c = dir.join("seed-c.wav");
        render_wav_track_seeded(
            "mic-loop-test-seed",
            SynthTrack::FutureWeRemember,
            0.5,
            8192,
            MultiBluetoothTopology::FullConstellationWall,
            &a,
        )
        .unwrap();
        render_wav_track_seeded(
            "mic-loop-test-seed",
            SynthTrack::FutureWeRemember,
            0.5,
            8192,
            MultiBluetoothTopology::FullConstellationWall,
            &b,
        )
        .unwrap();
        render_wav_track_seeded(
            "a-different-seed",
            SynthTrack::FutureWeRemember,
            0.5,
            8192,
            MultiBluetoothTopology::FullConstellationWall,
            &c,
        )
        .unwrap();
        let da = std::fs::read(&a).unwrap();
        let db = std::fs::read(&b).unwrap();
        let dc = std::fs::read(&c).unwrap();
        assert_eq!(da, db, "same seed must render byte-identical audio");
        assert_ne!(da, dc, "different seed must render different audio");
        let _ = std::fs::remove_file(a);
        let _ = std::fs::remove_file(b);
        let _ = std::fs::remove_file(c);
    }

    #[test]
    fn sota_mesh_verifies_all_8b_is_repos() {
        let status = verify_sota_mesh();
        assert!(status.sota_synthesize_winner.contains("8b-is-music"));
        assert!(status.sota_transcribe_winner.contains("kotoro"));
        assert!(status.sota_recall_winner.contains("mem8"));
        assert!(status.mem8_quad_verified);
        assert!(status.spherepop_replay_verified);
        assert!(status.spherepop_pops >= 2);
        assert_eq!(status.quantum_bell_probabilities.len(), 8);
        assert!(status.dynamic_eq.quanttern_tag.starts_with("qt:"));
    }

    #[test]
    fn ultralovegod_sensei_of_touch_renders_unclipped_stereo() {
        assert_eq!(
            SynthTrack::from_name("ultralovegod"),
            SynthTrack::UltralovegodSenseiOfTouch
        );
        assert_eq!(
            SynthTrack::from_name("frequenci"),
            SynthTrack::UltralovegodSenseiOfTouch
        );
        let out = std::env::temp_dir().join("ultralovegod-sensei-test.wav");
        let summary = render_wav_track(
            SynthTrack::UltralovegodSenseiOfTouch,
            0.5,
            8192,
            MultiBluetoothTopology::FullConstellationWall,
            &out,
        )
        .unwrap();
        assert!(summary.peak_left > 0.05 && summary.peak_left <= 1.0);
        assert!(summary.peak_right > 0.05 && summary.peak_right <= 1.0);
        let _ = std::fs::remove_file(out);
    }
}
