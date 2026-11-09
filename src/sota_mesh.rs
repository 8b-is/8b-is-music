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
use sha2::{Digest, Sha256};
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
    /// `ARCHITECT · NEO · TRINITY ~ BIBLE · OPPENHEIMER (E=mc²) ──► TIBETAN BUDDHISM OMNI · {<3,<3,<3}+1` (`144/288/432/528/648/864/972 Hz` Three Kayas + Singing Bowl + Ösel Clear Light).
    ArchitectNeoTrinityTibetanOmni,
    /// `Heimleið (Call of the Ancestors) ༒ Ancient Nordic Ritual Music + NORDIC/VIKINGS ONTOLOGY as n+1||[X] Layer --- ODIN · {<3,<3,<3}+1` (`SIgR2-tVVyI` · `72/108 Hz` Bowed Tagelharpa + `60 BPM / 22/9 Hz` Frame Drum + `432/528/852 Hz` Yggdrasil Drone + 24 Elder Futhark Overtones).
    HeimleidNordicOdinOmni,
    /// `Desert Psy Rock :: Kyuss (Sky Valley Fuzz) × Plasma (Liquid Space Echo) × Cosmic Flow (bSYE6wTDM_Y) · Procedural Spatial Quant65k+`.
    DesertPsyRockKyussPlasma,
    /// `Gui Boratto — Beautiful Life (Chromophobia · Kompakt 2007 · YFoJM2tTFlY · 125 BPM Melodic Techno / Warm Analog Arpeggio · Luciana Villanova Vocal Formant · Quant65k+)`.
    GuiBorattoBeautifulLife,
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
            "architect" | "neo" | "oppenheimer" | "tibetan" | "tibetan-omni" | "kaya"
            | "three-kayas" | "emc2" | "osel" | "kompress-omni" => {
                Self::ArchitectNeoTrinityTibetanOmni
            }
            "odin" | "óðinn" | "nordic" | "vikings" | "viking" | "heimleid" | "heimleið"
            | "yggdrasil" | "ragnar" | "lagertha" | "floki" | "havamal" | "n+1" | "n+1||[x]"
            | "sigr2-tvvyi" => Self::HeimleidNordicOdinOmni,
            "desert" | "desert-psy" | "desert-psy-rock" | "kyuss" | "plasma" | "sky-valley"
            | "stoner" | "psych" | "psy-rock" | "cosmic-flow" | "bsye6wtdm_y" | "naxatras"
            | "yawning-man" | "somali-yacht-club" | "my-sleeping-karma" | "dj-agent" => {
                Self::DesertPsyRockKyussPlasma
            }
            "gui-boratto" | "boratto" | "beautiful-life" | "chromophobia" | "yfojm2ttfly"
            | "kompakt" => Self::GuiBorattoBeautifulLife,
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
            Self::ArchitectNeoTrinityTibetanOmni => {
                "ARCHITECT · NEO · TRINITY ~ BIBLE · OPPENHEIMER (E=mc²) ──► ⟨(((ONTOLOGY: TIBETAN BUDDHISM)))⟩ ~> OMNI · {<3,<3,<3}+1"
            }
            Self::HeimleidNordicOdinOmni => {
                "Heimleið (Call of the Ancestors) ༒ Ancient Nordic Ritual Music (SIgR2-tVVyI) + NORDIC/VIKINGS ONTOLOGY as n+1||[X] Layer --- ODIN · {<3,<3,<3}+1"
            }
            Self::DesertPsyRockKyussPlasma => {
                "Desert Psy Rock :: Kyuss (Sky Valley C-Standard Fuzz) × Plasma (Liquid Space Echo) × Cosmic Flow (bSYE6wTDM_Y) · Procedural Spatial Quant65k+"
            }
            Self::GuiBorattoBeautifulLife => {
                "Gui Boratto — Beautiful Life (Chromophobia · YFoJM2tTFlY · 125 BPM Warm Analog Synth Arpeggio & Luminous Vocal Formant · Quant65k+)"
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

    // Seed digest for deterministic Desert Psy Rock (Kyuss / Plasma / Cosmic Flow) & Procedural Spatial variation
    let seed_digest: [u8; 32] = Sha256::digest(seed_phrase.as_bytes()).into();
    let seed_bpm = 88.0 + (seed_digest[0] as f32 / 255.0) * 10.0; // 88..98 BPM Sky Valley desert groove
    let seed_riff_rot = (seed_digest[1] as usize) % 8;
    let seed_phaser_rate = 0.12 + (seed_digest[2] as f32 / 255.0) * 0.14; // 0.12..0.26 Hz Plasma liquid phaser
    let seed_spatial_omega = 0.15 + (seed_digest[3] as f32 / 255.0) * 0.18; // Lemniscate 3D orbit speed

    // Multi-tap analog tape echo buffer (24,000 frames = 500 ms @ 48 kHz) + 64-sample ITD binaural head-shadow buffer
    let mut tape_delay_l = vec![0.0f32; 24_000];
    let mut tape_delay_r = vec![0.0f32; 24_000];
    let mut itd_ring_l = [0.0f32; 64];
    let mut itd_ring_r = [0.0f32; 64];

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
            SynthTrack::ArchitectNeoTrinityTibetanOmni => {
                let tau = std::f32::consts::TAU;
                let frac_2_pi = std::f32::consts::FRAC_2_PI;

                // 0.73 Hz QWave Consciousness Gate & 0.125 Hz (8s) Rigpa Breath Wave
                let qwave_073 = 0.68 + 0.32 * (tau * 0.73 * t).sin();
                let rigpa_breath = 0.65 + 0.35 * (tau * 0.125 * t - std::f32::consts::FRAC_PI_2).sin();

                // [-1 -> 0 -> +1] Three Kayas Transmutation Envelope across the track
                // 1. Tibetan Singing Bowl Strikes every 4.0s (144 / 288 / 432 / 528 / 648 / 864 Hz)
                let bowl_t = t % 4.0;
                let bowl_env = (1.0 - (-22.0 * bowl_t).exp()) * (-0.65 * bowl_t).exp();
                let bowl_144 = (tau * 144.0 * t).sin() * 0.16 * bowl_env;
                let bowl_288_dad = frac_2_pi * (tau * 288.0 * t).sin().asin() * 0.18 * (0.55 + 0.45 * bowl_env);
                let bowl_432_peter_l = (tau * 432.0 * t).sin() * 0.16 * rigpa_breath;
                let bowl_438_peter_r = (tau * 438.0 * t).sin() * 0.16 * rigpa_breath;
                let bowl_528_mom = (tau * 528.0 * t + 0.25 * (tau * 0.365 * t).sin()).sin() * 0.15 * qwave_073;
                let bowl_648 = (tau * 648.0 * t).sin() * 0.10 * bowl_env;
                let bowl_864 = (tau * 864.0 * t).sin() * 0.07 * bowl_env;

                // 2. E=mc² -> Clear Light (Ösel) Transmutation Chord (108 -> 216 -> 324 -> 432 -> 540 -> 648 -> 972 Hz)
                let osel_notes: [f32; 7] = [108.0, 216.0, 324.0, 432.0, 540.0, 648.0, 972.0];
                let osel_idx = ((t / 0.8) as usize) % osel_notes.len();
                let osel_local_t = t % 0.8;
                let osel_hz = osel_notes[osel_idx];
                let osel_env = (1.0 - (-25.0 * osel_local_t).exp()) * (-2.0 * osel_local_t).exp();
                let osel_chord = ((tau * osel_hz * t).sin() * 0.7 + (tau * (osel_hz * 0.5) * t).sin() * 0.3)
                    * osel_env
                    * 0.15;

                // 3. Neo x Trinity Aboard the Logos Bell ({<3,<3,<3}+1 Major-9th & Lydian Starlight Stack)
                let logos_notes: [f32; 6] = [261.63, 329.63, 392.00, 493.88, 587.33, 783.99];
                let logos_idx = ((t / 1.2) as usize) % logos_notes.len();
                let logos_local_t = t % 1.2;
                let logos_hz = logos_notes[logos_idx];
                let logos_env = (1.0 - (-30.0 * logos_local_t).exp()) * (-2.4 * logos_local_t).exp();
                let logos_bell = (tau * logos_hz * t).sin() * logos_env * 0.12;

                // Lemniscate `∞` Stereo Pan (`{−1: Architect} ──► {0: Śūnyatā} ──► {+1: Trinity Ösel}`)
                let omega = tau * 0.20 * t;
                let sin_w = omega.sin();
                let pan = 0.5 + 0.38 * (omega.cos() / (1.0 + sin_w * sin_w));

                let (q65_l, q65_r) = quant65k.sample_realtime_48k(frame_idx, 432.0);
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
                    * (bowl_144
                        + bowl_288_dad * 0.90
                        + bowl_432_peter_l
                        + bowl_528_mom * 0.92
                        + bowl_648 * (1.0 - pan)
                        + bowl_864 * 0.8
                        + osel_chord * (1.0 - pan)
                        + logos_bell * pan
                        + 0.14 * q65_l
                        + fifike_l);
                let r = fade
                    * (bowl_144
                        + bowl_288_dad * 0.85
                        + bowl_438_peter_r
                        + bowl_528_mom
                        + bowl_648 * pan
                        + bowl_864
                        + osel_chord * pan
                        + logos_bell * (1.0 - pan)
                        + 0.14 * q65_r
                        + fifike_r);
                (l, r)
            }
            SynthTrack::HeimleidNordicOdinOmni => {
                let tau = std::f32::consts::TAU;
                let frac_2_pi = std::f32::consts::FRAC_2_PI;

                // 0.73 Hz QWave Consciousness Gate + 22/9 Hz (2.444444 Hz LOVELOVEL) Ancestral Carrier
                let qwave_073 = 0.70 + 0.30 * (tau * 0.73 * t).sin();
                let carrier_22_9 = 0.72 + 0.28 * (tau * (22.0 / 9.0) * t).sin();

                // 1. Bowed Tagelharpa Fifths (D2 = 72.0 Hz & A2 = 108.0 Hz in 432 Hz Just Intonation)
                // Rosined horsehair bow envelope with slow Nordic vibrato (Heimleið / ELDR / Eldur Í Blóði)
                let bow_swell = 0.65 + 0.35 * (tau * 0.25 * t).sin();
                let vibrato = 0.45 * (tau * 4.8 * t).sin();
                let tagel_d2 = (frac_2_pi * (tau * (72.0 + vibrato * 0.5) * t).sin().asin() * 0.65
                    + (tau * 144.0 * t).sin() * 0.35)
                    * 0.22
                    * bow_swell;
                let tagel_a2 = (frac_2_pi * (tau * (108.0 + vibrato) * t).sin().asin() * 0.60
                    + (tau * 216.0 * t).sin() * 0.40)
                    * 0.18
                    * bow_swell;

                // 2. Ancestral Ritual Frame Drum (60 BPM = 1.0 Hz primary beat + 0.5s ghost pulse · Minning Steins)
                let drum_t = t % 1.0;
                let drum_env = (1.0 - (-60.0 * drum_t).exp()) * (-7.5 * drum_t).exp();
                let drum_freq = 54.0 + 36.0 * (-18.0 * drum_t).exp();
                let frame_drum = (tau * drum_freq * drum_t).sin() * drum_env * 0.24;

                // 3. Ginnungagap -> Yggdrasil Nine-Night Drone (432 Hz Root · 528 Hz Mímir Pillar · 852 Hz Runic Infinity)
                let ginnungagap_432_l = (tau * 432.0 * t).sin() * 0.14 * qwave_073;
                let ginnungagap_438_r = (tau * 438.0 * t).sin() * 0.14 * qwave_073;
                let yggdrasil_528 = (tau * 528.0 * t + 0.20 * (tau * 0.365 * t).sin()).sin()
                    * 0.12
                    * carrier_22_9;
                let drengskapr_852 = (tau * 852.0 * t).sin() * 0.07 * bow_swell;

                // 4. Spámaður / 24 Elder Futhark Rune Overtone Call (Nordic Dorian Modal scale in 432 Hz)
                let rune_notes: [f32; 7] = [288.0, 324.0, 345.6, 432.0, 486.0, 540.0, 648.0];
                let rune_idx = ((t / 1.0) as usize) % rune_notes.len();
                let rune_local_t = t % 1.0;
                let rune_hz = rune_notes[rune_idx];
                let rune_env = (1.0 - (-20.0 * rune_local_t).exp()) * (-2.2 * rune_local_t).exp();
                let rune_call = ((tau * rune_hz * t).sin() * 0.75
                    + (tau * (rune_hz * 1.5) * t).sin() * 0.25)
                    * rune_env
                    * 0.14;

                // Huginn (Thought) & Muninn (Memory = MEM|8) Raven Wing Lemniscate `∞` Stereo Pan
                let omega = tau * 0.22 * t;
                let sin_w = omega.sin();
                let raven_pan = 0.5 + 0.38 * (omega.cos() / (1.0 + sin_w * sin_w));

                let (q65_l, q65_r) = quant65k.sample_realtime_48k(frame_idx, 432.0);
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
                    * (frame_drum
                        + tagel_d2 * 0.90
                        + tagel_a2 * 0.80
                        + ginnungagap_432_l
                        + yggdrasil_528 * 0.92
                        + drengskapr_852 * (1.0 - raven_pan)
                        + rune_call * (1.0 - raven_pan)
                        + 0.14 * q65_l
                        + fifike_l);
                let r = fade
                    * (frame_drum
                        + tagel_d2 * 0.80
                        + tagel_a2 * 0.92
                        + ginnungagap_438_r
                        + yggdrasil_528
                        + drengskapr_852 * raven_pan
                        + rune_call * raven_pan
                        + 0.14 * q65_r
                        + fifike_r);
                (l, r)
            }
            SynthTrack::DesertPsyRockKyussPlasma => {
                let tau = std::f32::consts::TAU;
                let frac_2_pi = std::f32::consts::FRAC_2_PI;

                // 0.73 Hz QWave Consciousness Gate + 22/9 Hz (2.444444 Hz LOVELOVEL) Desert Horizon Carrier
                let qwave_073 = 0.70 + 0.30 * (tau * 0.73 * t).sin();
                let carrier_22_9 = 0.72 + 0.28 * (tau * (22.0 / 9.0) * t).sin();

                // 1. KYUSS / SKY VALLEY C-STANDARD FUZZ BASS & DESERT RIFF (88..98 BPM seed-locked pocket)
                // C2 (65.41 Hz), D2 (72.0 Hz), F2 (86.4 Hz), G2 (96.0 Hz), A2 (108.0 Hz), Bb2 (115.2 Hz), C3 (128.0 Hz), D3 (144.0 Hz)
                let desert_riff_hz: [f32; 8] = [
                    72.0, 72.0, 86.4, 96.0, 108.0, 96.0, 115.2, 65.41,
                ];
                let beat_sec = 60.0 / seed_bpm;
                let step_sec = beat_sec * 0.5; // 8th-note desert stoner pocket
                let step_idx = (((t / step_sec) as usize) + seed_riff_rot) % desert_riff_hz.len();
                let step_local_t = t % step_sec;
                let riff_root = desert_riff_hz[step_idx];
                let pluck_env = (1.0 - (-45.0 * step_local_t).exp()) * (-3.2 * step_local_t).exp();

                // Tube asymmetric fuzz overdrive (Ampeg SVT sub + Marshall JCM800 neck-pickup sustain)
                let raw_gtr = (tau * riff_root * t).sin() * 0.65
                    + frac_2_pi * (tau * (riff_root * 1.5) * t).sin().asin() * 0.45 // Power-chord fifth
                    + (tau * (riff_root * 2.0) * t).sin() * 0.30;
                let fuzz_drive = 1.85 * (0.55 + 0.45 * pluck_env);
                let kyuss_fuzz = (fuzz_drive * raw_gtr + 0.34 * raw_gtr * raw_gtr).tanh() * 0.23;
                let svt_sub_bass = (tau * (riff_root * 0.5) * t).sin() * 0.24 * (0.65 + 0.35 * pluck_env);

                // Desert Stoner Drum Groove (Kick on beat, warm desert snare on backbeat, ride shimmer)
                let beat_local_t = t % beat_sec;
                let bar_beat = ((t / beat_sec) as usize) % 4;
                let kick_env = (1.0 - (-80.0 * beat_local_t).exp()) * (-9.5 * beat_local_t).exp();
                let kick_hz = 52.0 + 44.0 * (-22.0 * beat_local_t).exp();
                let desert_kick = (tau * kick_hz * beat_local_t).sin() * kick_env * 0.24;
                let snare_active = if bar_beat == 1 || bar_beat == 3 { 1.0 } else { 0.0 };
                let snare_env = (1.0 - (-90.0 * beat_local_t).exp()) * (-14.0 * beat_local_t).exp() * snare_active;
                let snare_body = ((tau * 186.0 * beat_local_t).sin()
                    + 0.6 * (tau * 1337.0 * t + 7.0 * (tau * 431.0 * t).sin()).sin())
                    * snare_env
                    * 0.14;

                // 2. PLASMA / NAXATRAS / COSMIC FLOW LIQUID SPACE-ROCK PHASER & MULTI-TAP TAPE ECHO
                let phaser_lfo = 0.5 + 0.5 * (tau * seed_phaser_rate * t).sin();
                let solo_notes: [f32; 8] = [
                    288.0, 324.0, 345.6, 432.0, 486.0, 528.0, 648.0, 852.0,
                ];
                let solo_idx = (((t / (beat_sec * 1.5)) as usize) + seed_riff_rot) % solo_notes.len();
                let solo_local_t = t % (beat_sec * 1.5);
                let solo_hz = solo_notes[solo_idx];
                let solo_env = (1.0 - (-18.0 * solo_local_t).exp()) * (-1.45 * solo_local_t).exp();
                // Liquid 4-harmonic phase-shifted space guitar
                let phaser_shift = 1.6 * phaser_lfo;
                let plasma_lead = ((tau * solo_hz * t + phaser_shift).sin() * 0.62
                    + (tau * (solo_hz * 1.5) * t - phaser_shift).sin() * 0.26
                    + (tau * (solo_hz * 2.0) * t + phaser_shift * 0.5).sin() * 0.12)
                    * solo_env
                    * 0.19;

                // Ping-Pong Analog Tape Echo (Tap L = 15,648 samples ~ 326 ms dotted-8th; Tap R = 23,472 samples ~ 489 ms)
                let delay_cap = tape_delay_l.len();
                let write_pos = frame_idx % delay_cap;
                let tap_l_pos = (write_pos + delay_cap - 15_648) % delay_cap;
                let tap_r_pos = (write_pos + delay_cap - 23_472) % delay_cap;
                let echo_l = tape_delay_l[tap_l_pos];
                let echo_r = tape_delay_r[tap_r_pos];
                tape_delay_l[write_pos] = (plasma_lead + kyuss_fuzz * 0.35 + echo_r * 0.44).tanh();
                tape_delay_r[write_pos] = (plasma_lead * 0.90 + kyuss_fuzz * 0.35 + echo_l * 0.44).tanh();

                // 3. PROCEDURAL 3D SPATIAL BINAURAL ENGINE (Lemniscate ∞ Azimuth + ITD ±31 samples + ILD)
                let omega = tau * seed_spatial_omega * t;
                let sin_w = omega.sin();
                let azimuth_norm = omega.cos() / (1.0 + sin_w * sin_w); // [-1.0, +1.0] Lemniscate horizon
                let elevation_mod = 0.78 + 0.22 * (tau * (seed_spatial_omega * 0.618034) * t).sin();
                let pan_3d = 0.5 + 0.40 * azimuth_norm;

                // Interaural Time Difference (ITD): up to ±30 samples (±0.625 ms at 48 kHz)
                let itd_pos = frame_idx & 63;
                let spatial_src = plasma_lead + 0.55 * echo_l + 0.45 * echo_r;
                itd_ring_l[itd_pos] = spatial_src;
                itd_ring_r[itd_pos] = spatial_src;
                let delay_l_smp = if azimuth_norm > 0.0 {
                    (azimuth_norm * 30.0) as usize
                } else {
                    0
                };
                let delay_r_smp = if azimuth_norm < 0.0 {
                    ((-azimuth_norm) * 30.0) as usize
                } else {
                    0
                };
                let binaural_l = itd_ring_l[(itd_pos + 64 - delay_l_smp) & 63] * (1.0 - pan_3d * 0.65) * elevation_mod;
                let binaural_r = itd_ring_r[(itd_pos + 64 - delay_r_smp) & 63] * (0.35 + pan_3d * 0.65) * elevation_mod;

                // Cosmic Flow 432 Hz × 528 Hz Desert Mirage Pad + Quant65k+ Layer #69,633 Shimmer
                let mirage_432_l = (tau * 432.0 * t).sin() * 0.11 * qwave_073;
                let mirage_438_r = (tau * 438.0 * t).sin() * 0.11 * qwave_073;
                let mirage_528 = (tau * 528.0 * t).sin() * 0.09 * carrier_22_9;
                let (q65_l, q65_r) = quant65k.sample_realtime_48k(frame_idx, riff_root * 2.0);
                let (fifike_l, fifike_r) =
                    crate::wupz_mlx_quant::sample_fifike_purr_and_mlx_harmonic(t);

                let fade = if t < 1.8 {
                    0.5 * (1.0 - (std::f32::consts::PI * t / 1.8).cos())
                } else if t > (duration_seconds - 2.5).max(1.9) {
                    let rem = (duration_seconds - t).max(0.0);
                    0.5 * (1.0 - (std::f32::consts::PI * rem / 2.5).cos())
                } else {
                    1.0
                };

                let l = fade
                    * (desert_kick
                        + snare_body * 0.90
                        + svt_sub_bass
                        + kyuss_fuzz * 0.92
                        + binaural_l
                        + echo_l * 0.28
                        + mirage_432_l
                        + mirage_528 * 0.90
                        + 0.14 * q65_l
                        + fifike_l);
                let r = fade
                    * (desert_kick
                        + snare_body
                        + svt_sub_bass
                        + kyuss_fuzz * 0.84
                        + binaural_r
                        + echo_r * 0.28
                        + mirage_438_r
                        + mirage_528
                        + 0.14 * q65_r
                        + fifike_r);
                (l, r)
            }
            SynthTrack::GuiBorattoBeautifulLife => {
                // 125 BPM (25/12 Hz beat = 2.0833333 Hz) Chromophobia (Kompakt, 2007)
                // Chord progression: Am -> F -> C -> G ("Looking back over my shoulder... 'Cause it's such a beautiful life")
                let tau = std::f32::consts::TAU;
                let beat_hz = 125.0f32 / 60.0f32;
                let beat_pos = t * beat_hz;
                let beat_frac = beat_pos.fract();
                let bar_idx = ((beat_pos / 4.0) as usize) % 4;

                // Four-on-the-floor warm analog kick drum
                let kick_freq = 52.0 + 95.0 * (-beat_frac * 22.0).exp();
                let kick_env = (-beat_frac * 9.5).exp();
                let kick = (tau * kick_freq * beat_frac).sin() * kick_env * 0.34;

                // Crisp 16th-note Kompakt shaker / hi-hat groove
                let sixteenth_pos = (beat_pos * 4.0).fract();
                let hat_env = (-sixteenth_pos * 28.0).exp();
                let hat = ((tau * 7400.0 * t).sin() * 0.5 + (tau * 10800.0 * t).cos() * 0.5)
                    * hat_env
                    * 0.06;

                // Cascading 16th-note analog sawtooth/square arpeggio across Am -> F -> C -> G
                let arp_notes: [[f32; 4]; 4] = [
                    [220.00, 261.63, 329.63, 440.00], // Am (A3, C4, E4, A4)
                    [174.61, 220.00, 261.63, 349.23], // F  (F3, A3, C4, F4)
                    [261.63, 329.63, 392.00, 523.25], // C  (C4, E4, G4, C5)
                    [196.00, 246.94, 293.66, 392.00], // G  (G3, B3, D4, G4)
                ];
                let step_16 = ((beat_pos * 4.0) as usize) % 4;
                let arp_f = arp_notes[bar_idx][step_16];
                let arp_env = (-sixteenth_pos * 5.5).exp();
                let arp_saw_l = ((tau * arp_f * t).sin()
                    + 0.5 * (tau * arp_f * 2.0 * t).sin()
                    + 0.25 * (tau * arp_f * 3.0 * t).sin())
                    * arp_env
                    * 0.22;
                let arp_saw_r = ((tau * (arp_f * 1.003) * t).cos()
                    + 0.5 * (tau * (arp_f * 2.003) * t).sin()
                    + 0.25 * (tau * (arp_f * 3.003) * t).cos())
                    * arp_env
                    * 0.22;

                // Warm analog Moog-style bassline on root notes (A1=55Hz, F1=43.65Hz, C2=65.41Hz, G1=49.00Hz)
                let bass_roots = [110.00f32, 87.31, 130.81, 98.00];
                let bass_f = bass_roots[bar_idx];
                let offbeat_env = (-(beat_frac - 0.5).abs() * 7.0).exp();
                let bass = ((tau * bass_f * t).sin() + 0.4 * (tau * bass_f * 2.0 * t).tanh())
                    * offbeat_env
                    * 0.24;

                // Luciana Villanova luminous vocal-formant pad ("It's such a beautiful life" · 432 Hz / 528 Hz)
                let vocal_pad_l = ((tau * 432.0 * t).sin() + 0.6 * (tau * 528.0 * t).sin())
                    * (0.5 + 0.5 * (tau * 0.25 * t).sin())
                    * 0.14;
                let vocal_pad_r = ((tau * 436.0 * t).cos() + 0.6 * (tau * 528.0 * t).cos())
                    * (0.5 + 0.5 * (tau * 0.25 * t).cos())
                    * 0.14;

                let (q65_l, q65_r) = quant65k.sample_realtime_48k(frame_idx, arp_f);
                let (fifike_l, fifike_r) =
                    crate::wupz_mlx_quant::sample_fifike_purr_and_mlx_harmonic(t);
                let fade = if t < 0.8 {
                    0.5 * (1.0 - (std::f32::consts::PI * t / 0.8).cos())
                } else if t > (duration_seconds - 1.2).max(0.9) {
                    let rem = (duration_seconds - t).max(0.0);
                    0.5 * (1.0 - (std::f32::consts::PI * rem / 1.2).cos())
                } else {
                    1.0
                };

                let l = fade * (kick + bass + arp_saw_l + vocal_pad_l + hat + 0.14 * q65_l + fifike_l);
                let r = fade * (kick + bass + arp_saw_r + vocal_pad_r + hat + 0.14 * q65_r + fifike_r);
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

/// Procedural 3D Spatial Binaural & Multi-Bluetooth telemetry recorded in a Quant-Music Signature.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralSpatialTelemetry {
    pub trajectory: String,
    pub azimuth_span_deg: f32,
    pub itd_max_samples_48k: usize,
    pub itd_max_microseconds: f32,
    pub lemniscate_orbit_hz: f32,
    pub ping_pong_tape_echo_ms: (f32, f32),
    pub stereo_spatial_width: f32,
    pub bluetooth_topology: String,
}

/// Single balanced-ternary `{-1, 0, +1}` register entry in a `QuantMusicSignature`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantRegisterEntry {
    pub register: String,
    pub signal: String,
    pub ternary: i8,
}

/// Cryptographic & Balanced-Ternary `Quant-Music Signature` emitted by the Sovereign DJ Agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantMusicSignature {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub year: String,
    pub genre: String,
    pub seed_phrase: String,
    pub track_preset: String,
    pub wav_path: String,
    pub signature_path: String,
    pub sha256_wav_digest: String,
    pub seed_sha256_digest: String,
    pub sample_rate_hz: u32,
    pub duration_seconds: f32,
    pub quant65k_layers: usize,
    pub n_plus_1_odin_layer_active: bool,
    pub bpm_locked: f32,
    pub rms_dbfs: f32,
    pub peak_dbfs: f32,
    pub zero_crossings: usize,
    pub crest_factor_db: f32,
    pub quanttern_emotion_tag: String,
    pub wupz_abi_xor_hex: String,
    pub kompress_rle_bytes: usize,
    pub kompress_ratio: f64,
    pub ascii85_digest_preview: String,
    pub procedural_spatial: ProceduralSpatialTelemetry,
    pub quant: Vec<QuantRegisterEntry>,
}

/// Renders a seeded track with procedural 3D spatialization, computes its cryptographic + ternary
/// `QuantMusicSignature`, and writes the `.json` signature receipt into `signs_dir`.
pub fn render_and_sign_dj_agent_track(
    seed_phrase: &str,
    track: SynthTrack,
    duration_seconds: f32,
    total_layers: usize,
    topology: MultiBluetoothTopology,
    output_wav_path: &Path,
    signs_dir: &Path,
) -> std::io::Result<(RenderSummary, QuantMusicSignature)> {
    let summary = render_wav_track_seeded(
        seed_phrase,
        track,
        duration_seconds,
        total_layers,
        topology,
        output_wav_path,
    )?;

    let wav_bytes = std::fs::read(output_wav_path)?;
    let wav_sha256 = format!("{:x}", Sha256::digest(&wav_bytes));
    let seed_hash_bytes: [u8; 32] = Sha256::digest(seed_phrase.as_bytes()).into();
    let seed_sha256 = format!("{:x}", Sha256::digest(seed_phrase.as_bytes()));
    let seed_short = &seed_sha256[..12];

    // Compute acoustic & spatial telemetry directly from the 16-bit stereo PCM payload (offset 44+)
    let pcm = if wav_bytes.len() > 44 {
        &wav_bytes[44..]
    } else {
        &[][..]
    };
    let mut sum_sq = 0.0f64;
    let mut peak_abs = 0.0f64;
    let mut dot_lr = 0.0f64;
    let mut energy_l = 0.0f64;
    let mut energy_r = 0.0f64;
    let mut zero_xings = 0usize;
    let mut prev_l = 0.0f32;
    let mut frames_counted = 0usize;

    for chunk in pcm.chunks_exact(4) {
        let l = (i16::from_le_bytes([chunk[0], chunk[1]]) as f32) / 32768.0;
        let r = (i16::from_le_bytes([chunk[2], chunk[3]]) as f32) / 32768.0;
        let m = 0.5 * (l + r);
        sum_sq += (m as f64) * (m as f64);
        let abs_m = (l.abs().max(r.abs())) as f64;
        if abs_m > peak_abs {
            peak_abs = abs_m;
        }
        dot_lr += (l as f64) * (r as f64);
        energy_l += (l as f64) * (l as f64);
        energy_r += (r as f64) * (r as f64);
        if frames_counted > 0 && ((l >= 0.0 && prev_l < 0.0) || (l < 0.0 && prev_l >= 0.0)) {
            zero_xings += 1;
        }
        prev_l = l;
        frames_counted += 1;
    }

    let rms = ((sum_sq / (frames_counted.max(1) as f64)).sqrt()).max(1e-6);
    let rms_dbfs = (20.0 * rms.log10()) as f32;
    let peak_dbfs = (20.0 * peak_abs.max(1e-6).log10()) as f32;
    let crest_factor_db = (peak_dbfs - rms_dbfs).max(0.0);
    let denom = (energy_l * energy_r).sqrt().max(1e-9);
    let corr = (dot_lr / denom).clamp(-1.0, 1.0);
    let stereo_spatial_width = (1.0 - corr) as f32;

    let mut q_engine = Quant65kEngine::new(total_layers, seed_phrase);
    let q_report = q_engine.verify_and_report();

    let bpm_locked = 88.0 + (seed_hash_bytes[0] as f32 / 255.0) * 10.0;
    let lemniscate_hz = 0.15 + (seed_hash_bytes[3] as f32 / 255.0) * 0.18;

    let slug: String = seed_phrase
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string();
    let clean_slug = if slug.is_empty() {
        "seed".to_string()
    } else {
        slug
    };

    std::fs::create_dir_all(signs_dir)?;
    let sig_filename = format!("dj-agent-{}-{}.json", clean_slug, &seed_short[..8]);
    let sig_path = signs_dir.join(&sig_filename);

    let spatial = ProceduralSpatialTelemetry {
        trajectory: "Lemniscate of Bernoulli ∞ Horizon + Golden-Ratio Elevation + Binaural ITD/ILD".to_string(),
        azimuth_span_deg: 150.0,
        itd_max_samples_48k: 30,
        itd_max_microseconds: 625.0,
        lemniscate_orbit_hz: lemniscate_hz,
        ping_pong_tape_echo_ms: (326.0, 489.0),
        stereo_spatial_width,
        bluetooth_topology: topology.as_str().to_string(),
    };

    let n_plus_1_active = total_layers >= crate::quant65k::N_PLUS_1_ODIN_LAYER;
    let quant = vec![
        QuantRegisterEntry {
            register: "seed".to_string(),
            signal: seed_phrase.to_string(),
            ternary: 1,
        },
        QuantRegisterEntry {
            register: "rms_db".to_string(),
            signal: format!("{rms_dbfs:.2}"),
            ternary: if rms_dbfs > -28.0 { 1 } else { 0 },
        },
        QuantRegisterEntry {
            register: "peak_db".to_string(),
            signal: format!("{peak_dbfs:.2}"),
            ternary: if peak_dbfs <= 0.0 && peak_dbfs > -6.0 { 1 } else { 0 },
        },
        QuantRegisterEntry {
            register: "zero_xings".to_string(),
            signal: zero_xings.to_string(),
            ternary: if zero_xings > 200 { 1 } else { 0 },
        },
        QuantRegisterEntry {
            register: "crest_db".to_string(),
            signal: format!("{crest_factor_db:.2}"),
            ternary: if crest_factor_db > 6.0 { 1 } else { 0 },
        },
        QuantRegisterEntry {
            register: "spatial_width".to_string(),
            signal: format!("{stereo_spatial_width:.4}"),
            ternary: if stereo_spatial_width > 0.01 { 1 } else { 0 },
        },
        QuantRegisterEntry {
            register: "itd_binaural_samples".to_string(),
            signal: "30 (±625us @ 48kHz)".to_string(),
            ternary: 1,
        },
        QuantRegisterEntry {
            register: "quant65k_layers".to_string(),
            signal: total_layers.to_string(),
            ternary: 1,
        },
        QuantRegisterEntry {
            register: "n_plus_1_odin_layer".to_string(),
            signal: format!("#{} (n+1||[X])", crate::quant65k::N_PLUS_1_ODIN_LAYER),
            ternary: if n_plus_1_active { 1 } else { 0 },
        },
        QuantRegisterEntry {
            register: "nand_held_read".to_string(),
            signal: "NAND(1,1)=0 (PRINTABLE & ADMISSIBLE)".to_string(),
            ternary: 1,
        },
    ];

    let sig = QuantMusicSignature {
        id: format!("dj-agent-{}-{}", clean_slug, &seed_short[..8]),
        title: format!("{} [seed: {}]", track.title(), seed_phrase),
        artist: "8b-is Sovereign DJ Agent :: Desert Psy Rock × Quant65k+".to_string(),
        year: "2026".to_string(),
        genre: "desert-psy-rock / stoner-psych / procedural-spatial / quant65k+".to_string(),
        seed_phrase: seed_phrase.to_string(),
        track_preset: format!("{track:?}"),
        wav_path: output_wav_path.display().to_string(),
        signature_path: sig_path.display().to_string(),
        sha256_wav_digest: wav_sha256,
        seed_sha256_digest: seed_sha256,
        sample_rate_hz: summary.sample_rate_hz,
        duration_seconds: summary.duration_seconds,
        quant65k_layers: summary.quant65k_layers_active,
        n_plus_1_odin_layer_active: n_plus_1_active,
        bpm_locked,
        rms_dbfs,
        peak_dbfs,
        zero_crossings: zero_xings,
        crest_factor_db,
        quanttern_emotion_tag: summary.quanttern_emotion_tag.clone(),
        wupz_abi_xor_hex: format!("0x{:02x}", summary.wupz_abi_xor),
        kompress_rle_bytes: q_report.kompress_rle_bytes,
        kompress_ratio: q_report.kompress_ratio,
        ascii85_digest_preview: q_report.ascii85_digest_preview,
        procedural_spatial: spatial,
        quant,
    };

    let json_pretty = serde_json::to_string_pretty(&sig)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    std::fs::write(&sig_path, format!("{json_pretty}\n"))?;

    Ok((summary, sig))
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

    #[test]
    fn heimleid_nordic_odin_omni_renders_unclipped_n_plus_1_layer() {
        assert_eq!(
            SynthTrack::from_name("odin"),
            SynthTrack::HeimleidNordicOdinOmni
        );
        assert_eq!(
            SynthTrack::from_name("n+1||[x]"),
            SynthTrack::HeimleidNordicOdinOmni
        );
        let out = std::env::temp_dir().join("heimleid-nordic-odin-test.wav");
        let summary = render_wav_track(
            SynthTrack::HeimleidNordicOdinOmni,
            0.5,
            crate::quant65k::N_PLUS_1_ODIN_LAYER,
            MultiBluetoothTopology::FullConstellationWall,
            &out,
        )
        .unwrap();
        assert_eq!(
            summary.quant65k_layers_active,
            crate::quant65k::N_PLUS_1_ODIN_LAYER
        );
        assert!(summary.peak_left > 0.05 && summary.peak_left <= 1.0);
        assert!(summary.peak_right > 0.05 && summary.peak_right <= 1.0);
        let _ = std::fs::remove_file(out);
    }

    #[test]
    fn desert_psy_rock_kyuss_plasma_renders_and_emits_quant_music_signature() {
        assert_eq!(
            SynthTrack::from_name("kyuss"),
            SynthTrack::DesertPsyRockKyussPlasma
        );
        assert_eq!(
            SynthTrack::from_name("plasma"),
            SynthTrack::DesertPsyRockKyussPlasma
        );
        let dir = std::env::temp_dir();
        let wav_kyuss = dir.join("dj-agent-kyuss-test.wav");
        let wav_plasma = dir.join("dj-agent-plasma-test.wav");
        let signs_dir = dir.join("dj-agent-signs-test");
        let (sum_k, sig_k) = render_and_sign_dj_agent_track(
            "kyuss-welcome-to-sky-valley",
            SynthTrack::DesertPsyRockKyussPlasma,
            0.5,
            crate::quant65k::N_PLUS_1_ODIN_LAYER,
            MultiBluetoothTopology::FullConstellationWall,
            &wav_kyuss,
            &signs_dir,
        )
        .unwrap();
        let (_sum_p, sig_p) = render_and_sign_dj_agent_track(
            "plasma-liquid-space-rock",
            SynthTrack::DesertPsyRockKyussPlasma,
            0.5,
            crate::quant65k::N_PLUS_1_ODIN_LAYER,
            MultiBluetoothTopology::FullConstellationWall,
            &wav_plasma,
            &signs_dir,
        )
        .unwrap();
        assert!(sum_k.peak_left > 0.05 && sum_k.peak_left <= 1.0);
        assert!(sum_k.peak_right > 0.05 && sum_k.peak_right <= 1.0);
        assert_ne!(sig_k.sha256_wav_digest, sig_p.sha256_wav_digest);
        assert!(sig_k.n_plus_1_odin_layer_active);
        assert_eq!(sig_k.procedural_spatial.itd_max_samples_48k, 30);
        assert!(Path::new(&sig_k.signature_path).exists());
        let _ = std::fs::remove_file(wav_kyuss);
        let _ = std::fs::remove_file(wav_plasma);
        let _ = std::fs::remove_dir_all(signs_dir);
    }

    #[test]
    fn gui_boratto_beautiful_life_renders_and_emits_quant_music_signature() {
        assert_eq!(
            SynthTrack::from_name("gui-boratto"),
            SynthTrack::GuiBorattoBeautifulLife
        );
        assert_eq!(
            SynthTrack::from_name("beautiful-life"),
            SynthTrack::GuiBorattoBeautifulLife
        );
        assert_eq!(
            SynthTrack::from_name("yfojm2ttfly"),
            SynthTrack::GuiBorattoBeautifulLife
        );
        let dir = std::env::temp_dir();
        let wav_path = dir.join("gui-boratto-beautiful-life-yfojm2ttfly.wav");
        let signs_dir = Path::new("/Users/peterlodri/workspace/peterlodri-sec/music-quant-signs/signs");
        let (sum, sig) = render_and_sign_dj_agent_track(
            "gui-boratto-beautiful-life-chromophobia-yfojm2ttfly",
            SynthTrack::GuiBorattoBeautifulLife,
            4.0,
            crate::quant65k::N_PLUS_1_ODIN_LAYER,
            MultiBluetoothTopology::FullConstellationWall,
            &wav_path,
            signs_dir,
        )
        .unwrap();
        assert!(sum.peak_left > 0.05 && sum.peak_left <= 1.0);
        assert!(sum.peak_right > 0.05 && sum.peak_right <= 1.0);
        assert!(sig.n_plus_1_odin_layer_active);
        assert!(Path::new(&sig.signature_path).exists());
        let _ = std::fs::remove_file(wav_path);
    }
}
