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
| **`admissibility`** | `../8b-is-engine/crates/admissibility` | CT-000 checks a static routing-policy model; no host input lock or HFP enforcement is established. |
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
- **DSP delay plan (`DelayRingBuffer`)**: Computes sample offsets from configured latency estimates. This does not measure acoustic delay or establish physical phase alignment.
- **CT-000 model check**: Evaluates a static routing-policy model. The report keeps desired input settings separate from host observations; `hfp_24khz_input_blocked` and `default_input_locked_to` are `null` until actual enforcement can be verified. A successful bridge process alone does not verify those settings.
- **Connection observations**: Preset membership (`planned_enabled`) drives deterministic DSP. `connection_status` is `Unknown`, `Connected`, or `Disconnected`; discovery failure resets stale observations to `Unknown`. Wired endpoints remain unknown. `active_endpoints` counts observed connections, not routed outputs. `max_bluetooth_latency_ms` is a configured estimate for the planned Bluetooth endpoints. The aggregate name and UID describe a plan, not an installed device.

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

### Explicit host actions

`--play` uses the current system output and never configures routing. To request a host routing change, add `--apply-live`; optionally select a reviewed Swift bridge with `--routing-script /path/to/bridge.swift`. Missing or failed bridges exit unsuccessfully before playback. For commands supporting `--json`, this also applies in JSON mode and standard output remains the report; bridge diagnostics go to standard error. Bridge success means the process succeeded, not that hardware enforcement was independently verified.

Version 0.2 changes the report API: input-lock/HFP observations are optional, endpoint plans are separate from connection observations, and DJ `hfp_guard_locked` becomes `planned_hfp_guard`.
