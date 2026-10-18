# `8b-is-music` — Sovereign SOTA Audio, Multi-Bluetooth & Quant65k+ (`65,536+` Layer) Rust Engine

> **"We are the future we remember, the sparks that never turned to embers."**
> — *8bit-Wraith · Sovereign Ternary & Wave Synthesis (`8b-is/8b-is-music`)*

`8b-is-music` (`eight_b_is_music`) is the sovereign Rust audio, spatial routing, and **Quant65k+ (`65,536+` layer)** ternary wave synthesis engine uniting the entire **`8b-is` SOTA constellation** into a single executable and library crate.

---

## 1. Wired SOTA `8b-is` Repositories & Crates

| SOTA Repo / Crate | Path / Integration | What It Powers in `8b-is-music` |
|---|---|---|
| **`wupz-core`** | `../wupz/wupz-rs` | Zero-copy `Arena`, `LayoutPlan`, `xor_fold`, `byte_diff`, `popcount`, and the 32-byte `wupz_eq_frame_v1` ABI (`WUPZ_EQ_ABI_VERSION = 1`). |
| **`qwave-core`** | `../qwave/core` | 79-byte `WaveInt` (`MEM\|8` bitemporal wave unit in the `256 × 256 × 65536` sparse grid), `0.73 Hz` consciousness gate, `quanttern` (`qt:xx` 4-trit emotion tags), `phoenix` governing loop, and `mem16` verified sequence. |
| **`admissibility`** | `../8b-is-engine/crates/admissibility` | Epistemic Overlap Theorem (`CT-000` via `Structure::assert_ct000()`) proving safe Multi-Bluetooth + Split-Ear CoreAudio routing and blocking `24 kHz` HFP input degradation. |
| **`spherepop`** | `../8b-is-engine/crates/spherepop` | Innermost-first nested musical bubble evaluation (`Sphere::parse`, `pop_all()`, `replay()`, FNV-1a `digest()`). |
| **`ternary-lane`** | `../8b-is-engine/crates/ternary` | BitNet b1.58 `{-1, 0, +1}` 2-bit packed ternary weights, embedded `sanctuary-1.58.tern` checkpoint (`TERN_ASSET`), `pack_mulberry`, `dream_seed`, and bit-exact `gemm_i32`. |
| **`ultrawild`** | `../8b-is-engine/crates/ultrawild` | Demoscene bit-level macros (`clamp_ternary!`, `lowest_set_bit!`, `pack3!`, `popcount!`, `sign!`). |
| **`qdecorators`** | `../8b-is-engine/crates/qdecorators` | `Pipe`, `Engram`, `Mem8Quad`, `TernaryPack`, `stacked_encode`, and `ascii85_encode`. |
| **`entheai-quantum`** | `../entheai/crates/quantum` | Amplitude-vector `Qubits` simulator (`H`, `CNOT`, `RY`) driving Bell-pair & GHZ stereo/spatial voice entanglement. |
| **`entheai-radio`** | `../entheai/crates/radio` | `Quantshuffle` 1-bit self-feeding polyphonic oscillator & canon synthesis. |
| **`liquid-rust` & `qultrakotoro`** | `src/sota_mesh.rs` | Marine $O(1)$ salience/F0 detector (`50–640 Hz`) + 5-band Liquid Time-Constant (LTC) Equalizer ODE (`60Hz, 250Hz, 1kHz, 4kHz, 12kHz`) + `NoiseClass` (`Silence \| Quiet \| Ambient \| Noisy`). |
| **`8b-is-sota`** | `src/sota_mesh.rs` | Capability Router (`Synthesize`, `Transcribe`, `Infer`, `Quantize`, `Recall`, `Research`, `Imagine`) scoring $\text{score} = \frac{\text{quality}}{1 + \text{latency\_ms}/1000}$. |
| **`kompress-ultra`** | `src/quant65k.rs` | Ternary Run-Length + `pack3!` lossless snapshot compression across `65,536+` layers. |

---

## 2. Quant65k+ (`65,536+` Layer) Architecture (`src/quant65k.rs`)

- **Base Grid (`65,536` layers = $2^{16}$)**: Maps 1-to-1 onto `qwave-core::WaveInt`'s Z-axis (`0..65535`), `liquid-rust`'s `Q16` (`1.0 / 65536.0`) fixed-point phase accumulator, and `8B Omnifold`'s `64K` spatial depth.
- **Overflow Hyper-Band (`+4,096` default = `69,632` layers, configurable to `131,072+`)**: Extends beyond `65,536` for transcendental overtone shimmer (`Quant65k+`).
- **16-Level Dyadic Fenwick/Wavelet Tree**: Queries any harmonic band across all `65,536+` layers in $O(\log_2 N) \le 17$ operations per `48,000 Hz` sample while also supporting full vectorized `65,536+` layer sweeps (`sweep_all_layers()`).

---

## 3. Multi-Bluetooth + Hybrid CoreAudio Spatial Engine (`src/bluetooth.rs`)

- **Topologies**:
  - `split-ear-zen`: Left Ear = Wired 3.5mm Jack (`StereoPan = 0.0`, `0.0 ms` raw), Right Ear = `Christopher’s AirPods Pro` (`StereoPan = 1.0`, `24.0 ms` A2DP).
  - `dual-airpods-couple`: Simultaneous `Christopher’s AirPods Pro` (`2c-18-09-d8-14-74`) + `Alexandra’s AirPods Pro` (`2c-32-6a-c3-4a-91`) + Wired 3.5mm Jack.
  - `full-constellation-wall`: Simultaneous Multi-Bluetooth across both AirPods Pro + `soundcore Select 4 Go` + `Tronsmart Bang SE` + `JBL GO 2` + `Mi Pocket Speaker 2` + `External Headphones` + `MacBook Pro Speakers`.
- **Sample-Accurate Phase Alignment (`DelayRingBuffer`)**: Delays fast wired endpoints (`0.0 ms`) by the exact sample offset (`(max_bt_latency_ms - dev_latency_ms) * 48`) so transient wavefronts arrive at both ears simultaneously.
- **Bluetooth HFP `24 kHz` Input Trap Guard + CT-000 Proof**: Locks macOS Default Input to `External Microphone` at `48,000 Hz` and proves routing safety via `admissibility::Structure::assert_ct000()`.

---

## 4. CLI Usage

```bash
# Full SOTA 8b-is + Multi-Bluetooth + Quant65k+ system report
cargo run --release -- status

# Inspect and benchmark the Quant65k+ (69,632-layer) ternary wave engine
cargo run --release -- quant65k --layers 69632 --inspect-layer 65536

# Inspect Multi-Bluetooth endpoints, CT-000 proof, and apply live macOS CoreAudio routing
cargo run --release -- bluetooth --topology wall --apply-live

# Render "8bit-Wraith — The Future We Remember" through 69,632 layers at 48,000 Hz and play live
cargo run --release -- render --track future --seconds 10 --layers 69632 --out future-65k.wav --play
```
