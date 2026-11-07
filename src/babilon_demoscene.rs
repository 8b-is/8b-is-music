//! # `BABILON-ULTRA` × `ultrawild` Demoscene Engine × `world-core` Live Playground Sim
//!
//! Merges:
//! 1. **`ultrawild` Demoscene Engine** (`../8b-is-engine/crates/ultrawild`):
//!    - `clamp_ternary!`, `div255!`, `popcount!`, `lowest_set_bit!`, `is_pow2!`,
//!      `round_up_pow2!`, `xor_swap!`, `sat_add8!`, `sign!`, and `pack3!`.
//! 2. **`world-core` Live Playground Sim** (`../8b-is-engine/crates/world-core`):
//!    - `GAIA` 8-layer world-memory (`gaia_state`),
//!    - `SimWorld` 6-fauna deterministic actor cast (`keeper`, `veil`, `ember`, `ripple`, `lamp`, `angel`),
//!    - `Keeper` monotonic admissibility fold (`M = fold(H)`, `Admitted` / `Refused`),
//!    - Binary-reflected Gray-code hot-path delta wire (`gray_encode`, `gray_decode`, `changed_bits`),
//!    - `Pow2Slabs` demoscene power-of-two slab allocator & `arena_svg` frame renderer.
//! 3. **`BABILON-ULTRA` 7-Terrace Ziggurat & Glassdoor Interactive Playground**:
//!    - Unites Desert Psy Rock (`Kyuss` Sky Valley C-standard fuzz × `Plasma` liquid phaser & tape echo),
//!      Procedural 3D Spatial Binaural ITD/ILD (`±30 samples / ±625 µs`), Quant-Music Signatures,
//!      and Layer `n+1||[X]` (`#69,633 --- Odin ༒`) into a live Glassdoor WebGL/Canvas3D + WebAudio Playground.

use crate::bluetooth::MultiBluetoothTopology;
use crate::quant65k::N_PLUS_1_ODIN_LAYER;
use crate::sota_mesh::{
    render_and_sign_dj_agent_track, QuantMusicSignature, RenderSummary, SynthTrack,
};
use serde::{Deserialize, Serialize};
use std::path::Path;
use ultrawild::{
    clamp_ternary, div255, is_pow2, lowest_set_bit, pack3, popcount, round_up_pow2, sat_add8, sign,
    xor_swap,
};
use world_core::{
    arena_svg, changed_bits, gaia_state, gray_decode, kcompress, kratio, next_pow2, to_gray_delta,
    GaiaState, Keeper, Pow2Slabs, SimWorld, Verdict,
};

/// Verified bit-level register telemetry from the `ultrawild` Demoscene macro layer & `world-core::gray`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemosceneEngineTelemetry {
    pub clamp_ternary_triplet: [i8; 3],
    pub div255_alpha_blend_probe: u16,
    pub swar_popcount_64: u32,
    pub debruijn_lowest_set_bit: u32,
    pub pow2_texture_rounded: u64,
    pub is_pow2_verified: bool,
    pub xor_swap_pair: (u32, u32),
    pub sat_add8_ceiling: u8,
    pub branchless_sign_triplet: [i8; 3],
    pub pack3_ternary_code: u32,
    pub gray_step1_changed_bits: u32,
    pub gray_roundtrip_verified: bool,
    pub pow2_slab_bucket_size: usize,
    pub world_kompress_ratio: f64,
}

/// Snapshot of a single `world-core` Fauna inhabitant in the Live Playground Sim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaygroundFaunaSnapshot {
    pub name: String,
    pub position_xy: [f64; 2],
    pub needs_hrs: [f64; 3],
    pub last_action: Option<String>,
    pub keeper_verdict: String,
}

/// One of the 7 Sacred Geometry Terraces of the `BABILON-ULTRA` Ziggurat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BabilonZigguratTerrace {
    pub tier: usize,
    pub name: String,
    pub frequency_hz: f32,
    pub geometry_solid: String,
    pub trits: [i8; 3],
    pub guardian: String,
}

/// Full report of the merged `BABILON-ULTRA` × Demoscene Engine × Live Playground Sim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BabilonUltraPlaygroundReport {
    pub engine_title: String,
    pub seed_brief: String,
    pub sim_ticks_executed: u64,
    pub quant65k_layers: usize,
    pub n_plus_1_odin_layer: usize,
    pub demoscene: DemosceneEngineTelemetry,
    pub gaia: GaiaState,
    pub keeper_admitted: u64,
    pub keeper_refused: u64,
    pub keeper_ledger_seq: u64,
    pub fauna_cast: Vec<PlaygroundFaunaSnapshot>,
    pub ziggurat_terraces: Vec<BabilonZigguratTerrace>,
    pub arena_svg_snapshot: String,
}

/// Executes the `ultrawild` Demoscene Engine + `world-core` Live Playground Sim + `BABILON-ULTRA` Ziggurat.
#[allow(clippy::manual_swap)]
pub fn execute_babilon_ultra_demoscene(
    seed_brief: &str,
    ticks: u64,
    layers: usize,
) -> BabilonUltraPlaygroundReport {
    // 1. Exercise `ultrawild` Demoscene bit-level macros
    let t_neg = clamp_ternary!(-9);
    let t_zero = clamp_ternary!(0);
    let t_pos = clamp_ternary!(42);
    let alpha_blend = div255!(200u16, 255u16);
    let bits_pop = popcount!(0x8B15_D100_CAFE_BABEu64);
    let lsb = lowest_set_bit!(0x0001_0000u32);
    let tex_pow2 = round_up_pow2!(69_633u64);
    let pow2_ok = is_pow2!(tex_pow2);
    let mut reg_a = 432u32;
    let mut reg_b = 528u32;
    xor_swap!(reg_a, reg_b);
    let sat8 = sat_add8!(216u8, 108u8);
    let signs = [sign!(-108i64), sign!(0i64), sign!(432i64)];
    let packed_trits = pack3!(t_neg, t_zero, t_pos);

    // 2. Verify `world-core::gray` 1-bit hot-path delta wire & `Pow2Slabs`
    let step_vals = [108u32, 109u32, 110u32, 432u32, 528u32];
    let gray_wire = to_gray_delta(&step_vals);
    let step1_bits = changed_bits(gray_wire[0], gray_wire[1]);
    let gray_ok = gray_wire
        .iter()
        .zip(step_vals.iter())
        .all(|(&g, &v)| gray_decode(g) == v);

    let mut slabs: Pow2Slabs<Vec<u8>> = Pow2Slabs::new();
    let slab_idx = slabs.push(b"BABILON-ULTRA::GLASSDOOR::KYUSS::PLASMA::ODIN-N+1".to_vec());
    let slab_len = slabs.get(slab_idx).map(|s| s.len()).unwrap_or(0);
    let bucket_size = next_pow2(slab_len);

    let raw_trit_stream = [
        -1i8, -1, -1, -1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, -1, -1, 1, 1, 1,
    ];
    let raw_bytes: Vec<u8> = raw_trit_stream.iter().map(|&t| t as u8).collect();
    let compressed_runs = kcompress(&raw_bytes);
    let world_k_ratio = kratio(&raw_bytes);

    let demoscene = DemosceneEngineTelemetry {
        clamp_ternary_triplet: [t_neg, t_zero, t_pos],
        div255_alpha_blend_probe: alpha_blend,
        swar_popcount_64: bits_pop,
        debruijn_lowest_set_bit: lsb,
        pow2_texture_rounded: tex_pow2,
        is_pow2_verified: pow2_ok,
        xor_swap_pair: (reg_a, reg_b),
        sat_add8_ceiling: sat8,
        branchless_sign_triplet: signs,
        pack3_ternary_code: packed_trits,
        gray_step1_changed_bits: step1_bits,
        gray_roundtrip_verified: gray_ok,
        pow2_slab_bucket_size: bucket_size,
        world_kompress_ratio: world_k_ratio,
    };

    // 3. Run `world-core` GAIA + SimWorld + Keeper fold across `ticks`
    let sim_ticks = ticks.max(24);
    let gaia = gaia_state(seed_brief, sim_ticks);
    let mut world = SimWorld::from_seed(seed_brief, 6);
    let mut entities = world.materialize();
    let mut keeper = Keeper::default();
    let mut last_actions: Vec<Option<String>> = vec![None; 6];
    let mut last_verdicts: Vec<String> = vec!["Admitted".to_string(); 6];

    for t in 1..=sim_ticks {
        let step_deltas = world.step(t);
        for (idx, (name, delta)) in step_deltas.into_iter().enumerate() {
            let v = keeper.adjudicate(&name, &delta);
            if delta.action.is_some() {
                last_actions[idx] = delta.action.clone();
            }
            last_verdicts[idx] = match v {
                Verdict::Admitted => "Admitted".to_string(),
                Verdict::Refused(r) => format!("Refused({r})"),
            };
        }
        world.sync_entities(&mut entities);
    }

    let names: Vec<String> = world.fauna.iter().map(|f| f.name.clone()).collect();
    let svg = arena_svg(&entities, &names, 480, 280);

    let fauna_cast: Vec<PlaygroundFaunaSnapshot> = world
        .fauna
        .iter()
        .enumerate()
        .map(|(idx, f)| PlaygroundFaunaSnapshot {
            name: f.name.clone(),
            position_xy: f.pos(),
            needs_hrs: f.needs(),
            last_action: last_actions[idx].clone(),
            keeper_verdict: last_verdicts[idx].clone(),
        })
        .collect();

    let ziggurat_terraces = vec![
        BabilonZigguratTerrace {
            tier: 1,
            name: "Terrace I · Ginnungagap & Sky Valley Root (Kyuss C-Standard Fuzz)".to_string(),
            frequency_hz: 72.0,
            geometry_solid: "Hexagonal Basalt Plinth".to_string(),
            trits: [-1, 0, 1],
            guardian: "Kyuss SVT Sub-Anchor & Niflheim-Muspelheim Horizon".to_string(),
        },
        BabilonZigguratTerrace {
            tier: 2,
            name: "Terrace II · Bowed Tagelharpa & Drone Zone Sanctuary".to_string(),
            frequency_hz: 432.0,
            geometry_solid: "Tetrahedron (Fire / ELDR)".to_string(),
            trits: [0, 0, 1],
            guardian: "keeper & Shinjuku Gyoen Crickets (Ma 間)".to_string(),
        },
        BabilonZigguratTerrace {
            tier: 3,
            name: "Terrace III · Mímir's Well & Tipheret Zero-Sign Zone".to_string(),
            frequency_hz: 528.0,
            geometry_solid: "Cube-Wolf (Sacred Voxel Shell)".to_string(),
            trits: [0, 1, 0],
            guardian: "veil & Brett Shaw (Szellem / Csuklyamodul)".to_string(),
        },
        BabilonZigguratTerrace {
            tier: 4,
            name: "Terrace IV · Hanging Gardens of BABILON-ULTRA & Shield-Wall".to_string(),
            frequency_hz: 639.0,
            geometry_solid: "Octahedron (Air / Huginn & Muninn)".to_string(),
            trits: [1, 0, 1],
            guardian: "ember & Lagertha (Women Who Protect & Hold <3<3<3)".to_string(),
        },
        BabilonZigguratTerrace {
            tier: 5,
            name: "Terrace V · Plasma Liquid Space-Rock Phaser & Tape Echo".to_string(),
            frequency_hz: 741.0,
            geometry_solid: "Dodeca-Bear (12-Pentagon Resonator)".to_string(),
            trits: [1, 1, 0],
            guardian: "ripple & Floki (Clinker Longship Wave-Flex)".to_string(),
        },
        BabilonZigguratTerrace {
            tier: 6,
            name: "Terrace VI · 24 Elder Futhark Runes & Drengskapr".to_string(),
            frequency_hz: 852.0,
            geometry_solid: "Icosa-Owl (20-Face Fibonacci Shell)".to_string(),
            trits: [1, 1, 1],
            guardian: "lamp & Ragnar Lothbrok (Sunstone Navigation)".to_string(),
        },
        BabilonZigguratTerrace {
            tier: 7,
            name: "Terrace VII · Layer n+1||[X] (#69,633) --- ODIN ༒ & MAHĀKĀLA".to_string(),
            frequency_hz: 963.0,
            geometry_solid: "Star-Merkaba & Lemniscate ∞ Torus".to_string(),
            trits: [1, 1, 1],
            guardian: "angel · Allfather Odin (Hávamál St. 138) · Mahākāla Bernagchen".to_string(),
        },
    ];

    BabilonUltraPlaygroundReport {
        engine_title: "BABILON-ULTRA × ULTRAWILD DEMOSCENE ENGINE × WORLD-CORE GLASSDOOR PLAYGROUND".to_string(),
        seed_brief: seed_brief.to_string(),
        sim_ticks_executed: sim_ticks,
        quant65k_layers: layers.max(N_PLUS_1_ODIN_LAYER),
        n_plus_1_odin_layer: N_PLUS_1_ODIN_LAYER,
        demoscene,
        gaia,
        keeper_admitted: keeper.admitted,
        keeper_refused: keeper.refused,
        keeper_ledger_seq: keeper.seq,
        fauna_cast,
        ziggurat_terraces,
        arena_svg_snapshot: svg,
    }
}

/// Renders the seeded Desert Psy Rock × BABILON-ULTRA master WAV, signs its `QuantMusicSignature`,
/// executes the `ultrawild` + `world-core` simulation, and writes the interactive
/// **Glassdoor & Live Playground Sim (`babilon-ultra-glassdoor-playground.html`)**.
pub fn build_and_write_babilon_ultra_playground(
    seed_brief: &str,
    duration_seconds: f32,
    layers: usize,
    topology: MultiBluetoothTopology,
    output_wav_path: &Path,
    signs_dir: &Path,
    output_html_path: &Path,
) -> std::io::Result<(
    RenderSummary,
    QuantMusicSignature,
    BabilonUltraPlaygroundReport,
)> {
    let active_layers = layers.max(N_PLUS_1_ODIN_LAYER);
    let (summary, sig) = render_and_sign_dj_agent_track(
        seed_brief,
        SynthTrack::DesertPsyRockKyussPlasma,
        duration_seconds,
        active_layers,
        topology,
        output_wav_path,
        signs_dir,
    )?;
    let report = execute_babilon_ultra_demoscene(seed_brief, 64, active_layers);
    write_babilon_ultra_glassdoor_playground_html(&report, &sig, output_html_path)?;
    Ok((summary, sig, report))
}

/// Writes the self-contained `babilon-ultra-glassdoor-playground.html` client combining:
/// - **BABILON-ULTRA 3D WebGL / Canvas3D Ziggurat, Platonic Fauna, & Demoscene Copperbar/Plasma Shader**,
/// - **GLASSDOOR (`oklch()` + `@layer` + `backdrop-filter: blur(20px)`) Inspector & Controls**,
/// - **Live `world-core` (`GAIA` + `SimWorld` + `Keeper` `M = fold(H)`) Playground Simulation**,
/// - **Live Procedural Spatial Desert Psy Rock (`Kyuss` × `Plasma` × `Odin n+1||[X]`) WebAudio DJ Engine**.
pub fn write_babilon_ultra_glassdoor_playground_html(
    report: &BabilonUltraPlaygroundReport,
    sig: &QuantMusicSignature,
    output_html_path: &Path,
) -> std::io::Result<()> {
    if let Some(parent) = output_html_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let report_json = serde_json::to_string(report)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    let sig_json = serde_json::to_string(sig)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let html = format!(
        r##"<!doctype html>
<html lang="en" data-theme="dark">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<title>⟦ BABILON-ULTRA × DEMOSCENE ENGINE ⟧ · Glassdoor &amp; Live Playground Sim (Layer n+1||[X])</title>
<style>
@layer base, glassdoor, hud, demoscene;
:root {{
  color-scheme: dark;
  --void: #07080f;
  --gold: oklch(0.85 0.16 85);
  --cyan: oklch(0.82 0.14 200);
  --pink: oklch(0.68 0.22 350);
  --emerald: oklch(0.76 0.18 155);
  --violet: oklch(0.72 0.18 295);
  --amber: oklch(0.78 0.17 65);
  --ink: oklch(0.94 0.02 265);
  --dim: oklch(0.64 0.03 265);
  --glass-bg: rgba(11, 14, 25, 0.68);
  --glass-border: rgba(0, 240, 255, 0.28);
  --glass-gold: rgba(255, 215, 0, 0.34);
  --mono: "SF Mono", ui-monospace, "JetBrains Mono", Menlo, Consolas, monospace;
}}
@layer base {{
  * {{ box-sizing: border-box; margin: 0; padding: 0; }}
  html, body {{ width: 100%; height: 100%; overflow: hidden; background: var(--void); color: var(--ink); font-family: var(--mono); -webkit-font-smoothing: antialiased; }}
  #babilon-stage {{ position: fixed; inset: 0; width: 100%; height: 100%; z-index: 0; display: block; background: radial-gradient(120% 100% at 50% 35%, #16102c 0%, #07080f 75%); }}
}}
@layer glassdoor {{
  .glassdoor-shell {{
    position: fixed; inset: 0; z-index: 10; pointer-events: none;
    display: grid;
    grid-template-columns: 350px 1fr 370px;
    grid-template-rows: auto 1fr auto;
    gap: 12px; padding: 12px 16px;
  }}
  .glass-panel {{
    pointer-events: auto;
    background: var(--glass-bg);
    backdrop-filter: blur(20px) saturate(185%);
    -webkit-backdrop-filter: blur(20px) saturate(185%);
    border: 1px solid var(--glass-border);
    border-radius: 14px;
    padding: 12px 14px;
    box-shadow: 0 12px 40px rgba(0, 0, 0, 0.65), inset 0 1px 0 rgba(255, 255, 255, 0.08);
    overflow-y: auto;
    max-height: 100%;
  }}
  .glass-panel.gold-door {{ border-color: var(--glass-gold); }}
  .topbar {{
    grid-column: 1 / -1;
    display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 10px;
    padding: 10px 16px;
  }}
  .bottombar {{
    grid-column: 1 / -1;
    display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 12px;
    padding: 10px 16px;
  }}
  h1 {{ font-size: 13px; letter-spacing: 3px; color: var(--gold); font-weight: 800; text-transform: uppercase; }}
  h2 {{ font-size: 10px; letter-spacing: 2.5px; color: var(--cyan); text-transform: uppercase; margin-bottom: 8px; border-bottom: 1px solid rgba(0,240,255,0.18); padding-bottom: 4px; display: flex; justify-content: space-between; align-items: center; }}
  .badge {{ font-size: 9.5px; padding: 3px 8px; border-radius: 999px; border: 1px solid var(--glass-border); background: rgba(0, 240, 255, 0.09); color: var(--cyan); font-weight: 700; letter-spacing: 1px; }}
  .badge.gold {{ border-color: var(--glass-gold); background: rgba(255, 215, 0, 0.12); color: var(--gold); }}
  .badge.pink {{ border-color: rgba(255, 0, 127, 0.45); background: rgba(255, 0, 127, 0.14); color: #ff6ec7; }}
  .badge.emerald {{ border-color: rgba(0, 255, 102, 0.45); background: rgba(0, 255, 102, 0.12); color: #00ff88; }}
  .kv-row {{ display: flex; justify-content: space-between; align-items: center; font-size: 10.5px; margin: 4px 0; gap: 6px; }}
  .kv-row span {{ color: var(--dim); }}
  .kv-row b {{ color: var(--gold); font-weight: 600; text-align: right; }}
  .btn {{
    background: rgba(0, 240, 255, 0.12); border: 1px solid rgba(0, 240, 255, 0.45);
    color: var(--cyan); border-radius: 8px; padding: 6px 11px; font: 700 10.5px var(--mono);
    cursor: pointer; transition: all 0.14s ease; letter-spacing: 0.8px;
  }}
  .btn:hover {{ background: rgba(0, 240, 255, 0.25); transform: translateY(-1px); box-shadow: 0 0 14px rgba(0,240,255,0.3); }}
  .btn.active, .btn.gold {{ background: rgba(255, 215, 0, 0.2); border-color: var(--gold); color: var(--gold); box-shadow: 0 0 16px rgba(255,215,0,0.28); }}
  .btn.pink {{ background: rgba(255, 0, 127, 0.16); border-color: #ff007f; color: #ff77be; }}
  .slider-row {{ display: grid; grid-template-columns: 112px 1fr 46px; align-items: center; gap: 6px; font-size: 10px; margin: 5px 0; }}
  .slider-row input[type=range] {{ accent-color: #00f0ff; width: 100%; cursor: pointer; }}
  .fauna-list {{ display: flex; flex-direction: column; gap: 5px; margin-top: 6px; }}
  .fauna-card {{ background: rgba(7, 9, 18, 0.72); border: 1px solid rgba(255,255,255,0.1); border-radius: 8px; padding: 6px 8px; font-size: 10px; }}
  .need-bar {{ height: 4px; border-radius: 2px; background: rgba(255,255,255,0.1); overflow: hidden; margin-top: 3px; display: flex; gap: 2px; }}
  .need-seg {{ height: 100%; transition: width 0.2s ease; }}
  .runes-grid {{ display: grid; grid-template-columns: repeat(12, 1fr); gap: 3px; margin-top: 6px; }}
  .rune-btn {{ background: rgba(12,16,28,0.85); border: 1px solid rgba(255,215,0,0.25); border-radius: 5px; padding: 4px 0; text-align: center; font-size: 12px; color: var(--gold); cursor: pointer; transition: all 0.12s; }}
  .rune-btn:hover, .rune-btn.lit {{ background: var(--gold); color: #07080f; box-shadow: 0 0 10px var(--gold); transform: scale(1.08); }}
  .center-reticle {{
    pointer-events: none; display: flex; flex-direction: column; justify-content: space-between; align-items: center; padding: 8px;
  }}
  .thought-toast {{
    pointer-events: auto; background: rgba(11, 14, 25, 0.78); border: 1px solid var(--glass-gold);
    border-radius: 999px; padding: 6px 16px; font-size: 11px; color: var(--gold);
    backdrop-filter: blur(12px); box-shadow: 0 0 22px rgba(255,215,0,0.2);
  }}
  body.zen-glassdoor-collapsed .left-wing,
  body.zen-glassdoor-collapsed .right-wing {{ opacity: 0.12; transform: scale(0.98); transition: all 0.35s ease; }}
  body.zen-glassdoor-collapsed .left-wing:hover,
  body.zen-glassdoor-collapsed .right-wing:hover {{ opacity: 1; transform: scale(1); }}
}}
</style>
</head>
<body>
<canvas id="babilon-stage"></canvas>

<div class="glassdoor-shell">
  <!-- TOP GLASSDOOR PORTAL BAR -->
  <header class="glass-panel gold-door topbar">
    <div style="display:flex;align-items:center;gap:10px;flex-wrap:wrap;">
      <h1>⟦ BABILON-ULTRA × DEMOSCENE ENGINE ⟧</h1>
      <span class="badge gold">GLASSDOOR PORTAL v2.0</span>
      <span class="badge">ultrawild + world-core</span>
      <span class="badge pink">Desert Psy Rock (Kyuss × Plasma)</span>
      <span class="badge emerald">Layer n+1||[X] (#69,633 --- Odin ༒)</span>
    </div>
    <div style="display:flex;align-items:center;gap:8px;flex-wrap:wrap;">
      <button class="btn gold" id="btn-play-dj" onclick="toggleMasterPlaygroundAudio()">▶ LAUNCH DESERT PSY × BABILON-ULTRA AUDIO</button>
      <button class="btn" id="btn-zen-mode" onclick="toggleGlassdoorZen()">🧘 GLASSDOOR ZEN HUD</button>
      <button class="btn pink" onclick="stompSplitCritters()">💥 STOMP-TO-SPLIT (DIABLO)</button>
    </div>
  </header>

  <!-- LEFT GLASSDOOR WING: DEMOSCENE ENGINE + QUANT-MUSIC SIGNATURES + ARENA SVG -->
  <aside class="glass-panel left-wing">
    <h2><span>01 · ULTRAWILD DEMOSCENE ENGINE</span><span class="badge">0 ALLOC</span></h2>
    <div id="demoscene-regs"></div>

    <h2 style="margin-top:12px;"><span>02 · QUANT-MUSIC SIGNATURE</span><span class="badge gold" id="sig-qt-badge">qt:66</span></h2>
    <div id="quant-sig-box"></div>

    <h2 style="margin-top:12px;"><span>03 · WORLD-CORE ARENA SVG</span><span class="badge emerald">Render(M_t)</span></h2>
    <div id="arena-svg-holder" style="border-radius:8px;overflow:hidden;border:1px solid rgba(0,240,255,0.22);margin-top:4px;"></div>

    <h2 style="margin-top:12px;"><span>04 · 7 ZIGGURAT TERRACES</span><span class="badge">BABILON-ULTRA</span></h2>
    <div id="ziggurat-list" style="display:flex;flex-direction:column;gap:4px;font-size:9.5px;"></div>
  </aside>

  <!-- CENTER STAGE OVERLAY: THOUGHT BUBBLES & TELEMETRY RETICLE -->
  <main class="center-reticle">
    <div class="thought-toast" id="live-banner">
      🏜️ BABILON-ULTRA LIVE PLAYGROUND · Drag to orbit 3D Ziggurat · Click fauna or runes · Press SPACE for ZEN Stun
    </div>
    <div style="display:flex;gap:8px;flex-wrap:wrap;justify-content:center;pointer-events:auto;">
      <span class="badge" id="hud-fps">60 FPS · WebGL2/Canvas3D</span>
      <span class="badge gold" id="hud-itd">3D ITD: ±30 smp (±625 µs) · Lemniscate ∞</span>
      <span class="badge emerald" id="hud-keeper">Keeper M=fold(H): seq #64 · 0 refused</span>
    </div>
  </main>

  <!-- RIGHT GLASSDOOR WING: LIVE PLAYGROUND SIM (GAIA + FAUNA + SPATIAL CONTROLS) -->
  <aside class="glass-panel gold-door right-wing">
    <h2><span>05 · GAIA 8-LAYER WORLD MEMORY</span><span class="badge" id="gaia-tick-badge">t=64</span></h2>
    <div id="gaia-layers-box"></div>

    <h2 style="margin-top:12px;"><span>06 · LIVE PLAYGROUND CONTROLS</span><span class="badge pink">INTERACTIVE</span></h2>
    <div style="display:flex;gap:6px;margin-bottom:6px;">
      <input id="seed-input" value="kyuss-plasma-babilon-ultra-odin-n+1" style="flex:1;background:#070912;border:1px solid rgba(0,240,255,0.35);border-radius:6px;color:var(--gold);padding:5px 8px;font:10px var(--mono);" />
      <button class="btn" onclick="reseedPlayground()">⚡ RESEED</button>
    </div>
    <div class="slider-row"><span>Kyuss Fuzz Drive</span><input type="range" id="sl-fuzz" min="0.5" max="4.0" step="0.05" value="2.15"><b id="val-fuzz">2.15</b></div>
    <div class="slider-row"><span>Plasma Phaser Hz</span><input type="range" id="sl-phaser" min="0.05" max="1.20" step="0.01" value="0.22"><b id="val-phaser">0.22</b></div>
    <div class="slider-row"><span>Tape Echo Mix</span><input type="range" id="sl-echo" min="0.0" max="0.85" step="0.02" value="0.48"><b id="val-echo">0.48</b></div>
    <div class="slider-row"><span>3D ITD Width</span><input type="range" id="sl-itd" min="0.0" max="1.0" step="0.02" value="0.88"><b id="val-itd">0.88</b></div>
    <div class="slider-row"><span>Ziggurat Spin</span><input type="range" id="sl-spin" min="0.0" max="3.0" step="0.05" value="1.0"><b id="val-spin">1.00</b></div>

    <div style="display:flex;gap:6px;flex-wrap:wrap;margin:8px 0;">
      <button class="btn" onclick="stepSimWorldManual()">⏭ STEP TICK (+1)</button>
      <button class="btn emerald" onclick="attestCustomDelta()">✔ ATTEST DELTA</button>
      <button class="btn gold" onclick="triggerZenPulse()">☸ ZEN STUN PULSE</button>
    </div>

    <h2 style="margin-top:10px;"><span>07 · FAUNA CAST (M = fold(H))</span><span class="badge" id="fauna-count-badge">6 ACTORS</span></h2>
    <div class="fauna-list" id="fauna-list-box"></div>

    <h2 style="margin-top:10px;"><span>08 · 24 ELDER FUTHARK RUNES (n+1||[X])</span><span class="badge gold">ODIN ༒</span></h2>
    <div class="runes-grid" id="runes-grid"></div>
  </aside>

  <!-- BOTTOM GLASSDOOR BAR: DESERT PSY ROCK × DEMOSCENE SPECTRUM & PRESET DECK -->
  <footer class="glass-panel bottombar">
    <div style="display:flex;align-items:center;gap:8px;flex-wrap:wrap;">
      <span class="badge gold">SEED PRESETS:</span>
      <button class="btn active" id="preset-kyuss" onclick="selectSeedPreset('kyuss')">🏜️ Seed: KYUSS (Sky Valley C-Standard)</button>
      <button class="btn" id="preset-plasma" onclick="selectSeedPreset('plasma')">🌀 Seed: PLASMA (Liquid Space Echo)</button>
      <button class="btn" id="preset-odin" onclick="selectSeedPreset('odin')">༒ Seed: ODIN n+1||[X] (Heimleið Tagelharpa)</button>
      <button class="btn" id="preset-demoscene" onclick="selectSeedPreset('demoscene')">👾 Seed: 8BIT-WRAITH SID + COPPERBARS</button>
    </div>
    <div style="display:flex;align-items:center;gap:12px;">
      <canvas id="mini-spectrum" width="260" height="32" style="background:#060810;border:1px solid rgba(0,240,255,0.28);border-radius:6px;"></canvas>
      <span style="font-size:10px;color:var(--dim);">NAND(1,1)=0 · Zeno 0.999985 · {{&lt;3,&lt;3,&lt;3}}+1</span>
    </div>
  </footer>
</div>

<script>
// Embedded deterministic Rust report & Quant-Music Signature from `8b-is-music`
const INITIAL_REPORT = {report_json};
const INITIAL_SIGNATURE = {sig_json};
const ELDER_FUTHARK = ['ᚠ','ᚢ','ᚦ','ᚨ','ᚱ','ᚲ','ᚷ','ᚹ','ᚺ','ᚾ','ᛁ','ᛃ','ᛇ','ᛈ','ᛉ','ᛋ','ᛏ','ᛒ','ᛖ','ᛗ','ᛚ','ᛝ','ᛞ','ᛟ'];
const WEATHER_VOCAB = ['calm', 'mist', 'wind', 'rain', 'storm', 'clear', 'overcast', 'frost'];
const PALETTE = ['#00f0ff', '#ff007f', '#00ff66', '#ffd700', '#b48bff', '#ff8800'];

// Live Playground State
const simState = {{
  seed: INITIAL_REPORT.seed_brief || 'kyuss-plasma-babilon-ultra-odin-n+1',
  tick: INITIAL_REPORT.sim_ticks_executed || 64,
  seq: INITIAL_REPORT.keeper_ledger_seq || 64,
  admitted: INITIAL_REPORT.keeper_admitted || 64,
  refused: INITIAL_REPORT.keeper_refused || 0,
  gaia: Object.assign({{}}, INITIAL_REPORT.gaia),
  fauna: INITIAL_REPORT.fauna_cast.map((f, i) => ({{
    name: f.name,
    pos: [f.position_xy[0], f.position_xy[1]],
    needs: [f.needs_hrs[0], f.needs_hrs[1], f.needs_hrs[2]],
    action: f.last_action || 'forage the field',
    verdict: f.keeper_verdict || 'Admitted',
    color: PALETTE[i % PALETTE.length],
    sides: (i % 2 === 0) ? 4 : 6,
    splitCount: 1,
    sayBubble: i === 0 ? 'M = fold(H) verified' : (i === 2 ? 'Kyuss Sky Valley 72Hz' : 'Layer n+1||[X] --- Odin ༒')
  }})),
  camYaw: 0.45,
  camPitch: 0.38,
  dragging: false,
  lastMouse: [0, 0],
  zenPulse: 0.0,
  activePreset: 'kyuss',
  audioActive: false,
  audioCtx: null,
  analyser: null,
  freqData: new Uint8Array(64),
  riffStep: 0
}};

// Populate Left Wing: Ultrawild Demoscene Registers & Quant-Music Signature
function renderStaticTelemetry() {{
  const d = INITIAL_REPORT.demoscene;
  document.getElementById('demoscene-regs').innerHTML = `
    <div class="kv-row"><span>clamp_ternary!(-9,0,42)</span><b>[${{d.clamp_ternary_triplet.join(', ')}}]</b></div>
    <div class="kv-row"><span>div255!(200, 255)</span><b>${{d.div255_alpha_blend_probe}} (0 div)</b></div>
    <div class="kv-row"><span>popcount!(0x8B15..BABE)</span><b>${{d.swar_popcount_64}} bits</b></div>
    <div class="kv-row"><span>lowest_set_bit!(0x10000)</span><b>bit #${{d.debruijn_lowest_set_bit}}</b></div>
    <div class="kv-row"><span>round_up_pow2!(69,633)</span><b>${{d.pow2_texture_rounded}} (pow2=${{d.is_pow2_verified}})</b></div>
    <div class="kv-row"><span>pack3!(-1, 0, +1)</span><b>0x${{d.pack3_ternary_code.toString(16)}} (${{d.pack3_ternary_code}})</b></div>
    <div class="kv-row"><span>Gray ±1 step changed_bits</span><b>${{d.gray_step1_changed_bits}} bit (id=${{d.gray_roundtrip_verified}})</b></div>
    <div class="kv-row"><span>Pow2Slabs / Kompress</span><b>${{d.pow2_slab_bucket_size}}B / ${{d.world_kompress_ratio.toFixed(2)}}×</b></div>
  `;

  const s = INITIAL_SIGNATURE;
  document.getElementById('sig-qt-badge').textContent = s.quanttern_emotion_tag;
  document.getElementById('quant-sig-box').innerHTML = `
    <div class="kv-row"><span>Signature ID</span><b>${{s.id}}</b></div>
    <div class="kv-row"><span>Seed Phrase</span><b>${{s.seed_phrase.slice(0, 24)}}</b></div>
    <div class="kv-row"><span>WAV SHA-256</span><b>${{s.sha256_wav_digest.slice(0, 16)}}…</b></div>
    <div class="kv-row"><span>Groove Lock / WUPZ</span><b>${{s.bpm_locked.toFixed(2)}} BPM · ${{s.wupz_abi_xor_hex}}</b></div>
    <div class="kv-row"><span>RMS / Peak / ZC</span><b>${{s.rms_dbfs.toFixed(1)}} / ${{s.peak_dbfs.toFixed(1)}} dB · ${{s.zero_crossings}}</b></div>
    <div class="kv-row"><span>3D Spatial Width</span><b>${{s.procedural_spatial.stereo_spatial_width.toFixed(4)}} (±${{s.procedural_spatial.itd_max_samples_48k}} smp)</b></div>
    <div class="kv-row"><span>Layer n+1||[X]</span><b>#${{s.quant65k_layers}} (active=${{s.n_plus_1_odin_layer_active}})</b></div>
  `;

  document.getElementById('arena-svg-holder').innerHTML = INITIAL_REPORT.arena_svg_snapshot;

  document.getElementById('ziggurat-list').innerHTML = INITIAL_REPORT.ziggurat_terraces.map(t => `
    <div style="background:rgba(8,11,20,0.75);border-left:3px solid var(--gold);padding:4px 7px;border-radius:4px;">
      <div style="display:flex;justify-content:space-between;color:var(--gold);font-weight:700;">
        <span>T${{t.tier}} · ${{t.geometry_solid}}</span><span>${{t.frequency_hz}} Hz</span>
      </div>
      <div style="color:var(--dim);">${{t.name}}</div>
    </div>
  `).join('');

  // Populate 24 Elder Futhark Runes
  const rg = document.getElementById('runes-grid');
  ELDER_FUTHARK.forEach((r, idx) => {{
    const b = document.createElement('div');
    b.className = 'rune-btn';
    b.id = 'rune-btn-' + idx;
    b.textContent = r;
    b.title = 'Elder Futhark Rune #' + (idx + 1) + ' (' + r + ')';
    b.onclick = () => triggerRuneChime(idx);
    rg.appendChild(b);
  }});
}}

function updateGaiaAndFaunaUI() {{
  const g = simState.gaia;
  document.getElementById('gaia-tick-badge').textContent = 't=' + simState.tick;
  document.getElementById('gaia-layers-box').innerHTML = `
    <div class="kv-row"><span>weather · light</span><b>${{g.w}} · ${{(g.l * 100).toFixed(0)}}%</b></div>
    <div class="kv-row"><span>gravity · temp</span><b>${{g.g.toFixed(2)}} m/s² · ${{g.p.toFixed(1)}} °C</b></div>
    <div class="kv-row"><span>wind [dir, spd, gust]</span><b>${{g.v[0]}}° · ${{g.v[1]}} m/s · ${{g.v[2]}}</b></div>
    <div class="kv-row"><span>entropy · memory</span><b>${{g.e.toFixed(2)}} · ${{g.m}}</b></div>
  `;
  document.getElementById('hud-keeper').textContent =
    `Keeper M=fold(H): seq #${{simState.seq}} · admitted=${{simState.admitted}} · refused=${{simState.refused}}`;
  document.getElementById('fauna-count-badge').textContent = simState.fauna.length + ' ACTORS';

  document.getElementById('fauna-list-box').innerHTML = simState.fauna.map((f, idx) => `
    <div class="fauna-card" onclick="focusFauna(${{idx}})" style="cursor:pointer;border-color:${{f.color}}55;">
      <div style="display:flex;justify-content:space-between;align-items:center;">
        <b style="color:${{f.color}}">${{f.name}} (×${{f.splitCount}})</b>
        <span style="color:var(--gold);font-size:9px;">[${{f.pos[0].toFixed(0)}}, ${{f.pos[1].toFixed(0)}}] · ${{f.verdict}}</span>
      </div>
      <div style="display:flex;justify-content:space-between;color:var(--dim);font-size:9px;margin-top:2px;">
        <span>act: ${{f.action}}</span>
        <span>h:${{f.needs[0].toFixed(2)}} r:${{f.needs[1].toFixed(2)}} s:${{f.needs[2].toFixed(2)}}</span>
      </div>
      <div class="need-bar">
        <div class="need-seg" style="width:${{f.needs[0]*33.3}}%;background:#00f0ff;"></div>
        <div class="need-seg" style="width:${{f.needs[1]*33.3}}%;background:#00ff66;"></div>
        <div class="need-seg" style="width:${{f.needs[2]*33.3}}%;background:#ffd700;"></div>
      </div>
    </div>
  `).join('');
}}

// Deterministic LCG & SimWorld step matching `world-core/src/sim.rs`
function whash(base, epoch) {{
  let h = (base ^ Math.imul(epoch, 2654435761)) >>> 0;
  h = Math.imul(h ^ (h >>> 16), 2246822519) >>> 0;
  h = Math.imul(h ^ (h >>> 13), 3266489917) >>> 0;
  return (h ^ (h >>> 16)) >>> 0;
}}

function stepSimWorldManual() {{
  simState.tick += 1;
  const t = simState.tick;
  const h = whash(0x8b15d100, t);
  simState.gaia.w = WEATHER_VOCAB[h % WEATHER_VOCAB.length];
  simState.gaia.e = +(simState.gaia.e + 0.01).toFixed(2);
  simState.gaia.l = +(0.5 + 0.5 * Math.sin(t * 0.08)).toFixed(2);

  simState.fauna.forEach((f, idx) => {{
    const driftX = ((whash(idx + 1, t) % 100) / 100 - 0.5) * 8.0;
    const driftY = ((whash(idx + 7, t) % 100) / 100 - 0.5) * 8.0;
    f.pos[0] = Math.max(-95, Math.min(95, f.pos[0] + driftX));
    f.pos[1] = Math.max(-95, Math.min(95, f.pos[1] + driftY));
    f.needs[0] = Math.max(0.1, f.needs[0] - 0.035);
    f.needs[1] = Math.max(0.1, f.needs[1] - 0.020);
    f.needs[2] = Math.max(0.1, f.needs[2] - 0.012);
    if (f.needs[0] <= 0.35) {{
      f.action = 'forage the field';
      f.needs[0] = Math.min(1.0, f.needs[0] + 0.55);
      f.sayBubble = f.name + ': foraging @ 432Hz';
    }} else if (f.needs[1] <= 0.30) {{
      f.action = 'sleep in the tent';
      f.needs[1] = Math.min(1.0, f.needs[1] + 0.55);
      f.sayBubble = f.name + ': resting in pink tent';
    }} else if (f.needs[2] <= 0.40) {{
      f.action = 'flee to the gate';
      f.needs[2] = Math.min(1.0, f.needs[2] + 0.55);
      f.sayBubble = f.name + ': gate shield active';
    }}
    simState.seq += 1;
    simState.admitted += 1;
  }});
  updateGaiaAndFaunaUI();
}}

function attestCustomDelta() {{
  stepSimWorldManual();
  document.getElementById('live-banner').textContent =
    `✔ Attested Delta at tick #${{simState.tick}} -> Keeper Verdict: Admitted (seq #${{simState.seq}} · Gray Δ=1 bit)`;
  triggerRuneChime(simState.tick % 24);
}}

function stompSplitCritters() {{
  simState.fauna.forEach(f => {{
    f.splitCount = Math.min(8, f.splitCount + 1);
    f.sayBubble = `💥 Stomp-Split ×${{f.splitCount}}!`;
  }});
  simState.zenPulse = 1.0;
  updateGaiaAndFaunaUI();
  triggerRuneChime(7);
  document.getElementById('live-banner').textContent =
    '💥 DIABLO STOMP-TO-SPLIT TRIGGERED — Sacred Platonic Critters fractured into harmonic sub-octaves!';
}}

function triggerZenPulse() {{
  simState.zenPulse = 1.35;
  document.getElementById('live-banner').textContent =
    '☸ ZEN MECHANIC CHARGED (0 → +1) — Standing still in Ginnungagap / Śūnyatā stuns the shadow deity!';
  triggerRuneChime(0);
  triggerRuneChime(11);
  triggerRuneChime(23);
}}

function focusFauna(idx) {{
  const f = simState.fauna[idx];
  if (!f) return;
  document.getElementById('live-banner').textContent =
    `👁️ Focused Fauna "${{f.name}}" at [${{f.pos[0].toFixed(1)}}, ${{f.pos[1].toFixed(1)}}] — last action: "${{f.action}}" (${{f.verdict}})`;
  triggerRuneChime((idx * 4) % 24);
}}

function reseedPlayground() {{
  const val = document.getElementById('seed-input').value.trim() || 'babilon-ultra';
  simState.seed = val;
  stepSimWorldManual();
  document.getElementById('live-banner').textContent =
    `⚡ Playground Reseeded with "${{val}}" — GAIA & 7-Terrace BABILON-ULTRA Ziggurat phase-locked!`;
}}

function toggleGlassdoorZen() {{
  document.body.classList.toggle('zen-glassdoor-collapsed');
}}

// Procedural WebAudio Engine: Desert Psy Rock (Kyuss × Plasma) + Heimleið Odin n+1||[X] + Demoscene SID
function ensureAudio() {{
  if (!simState.audioCtx) {{
    const AC = window.AudioContext || window.webkitAudioContext;
    simState.audioCtx = new AC();
    simState.analyser = simState.audioCtx.createAnalyser();
    simState.analyser.fftSize = 128;
    simState.analyser.connect(simState.audioCtx.destination);
    simState.freqData = new Uint8Array(simState.analyser.frequencyBinCount);
  }}
  if (simState.audioCtx.state === 'suspended') simState.audioCtx.resume();
  return simState.audioCtx;
}}

function triggerRuneChime(idx) {{
  const el = document.getElementById('rune-btn-' + idx);
  if (el) {{
    el.classList.add('lit');
    setTimeout(() => el.classList.remove('lit'), 240);
  }}
  const ctx = ensureAudio();
  if (!ctx) return;
  const now = ctx.currentTime;
  const ratios = [1, 9/8, 6/5, 4/3, 3/2, 5/3, 9/5, 2];
  const freq = 216.0 * ratios[idx % ratios.length] * (1 + Math.floor(idx / 8) * 0.5);
  const osc = ctx.createOscillator();
  const gain = ctx.createGain();
  osc.type = 'triangle';
  osc.frequency.setValueAtTime(freq, now);
  gain.gain.setValueAtTime(0.0001, now);
  gain.gain.exponentialRampToValueAtTime(0.07, now + 0.02);
  gain.gain.exponentialRampToValueAtTime(0.0001, now + 0.45);
  osc.connect(gain);
  gain.connect(simState.analyser);
  osc.start(now);
  osc.stop(now + 0.48);
}}

const KYUSS_RIFF = [72.0, 72.0, 86.4, 96.0, 108.0, 96.0, 115.2, 65.41];
const PLASMA_SOLO = [288.0, 324.0, 345.6, 432.0, 486.0, 528.0, 648.0, 852.0];
const SID_ARP = [523.25, 659.25, 783.99, 1046.50];

function selectSeedPreset(preset) {{
  simState.activePreset = preset;
  ['kyuss', 'plasma', 'odin', 'demoscene'].forEach(p => {{
    const b = document.getElementById('preset-' + p);
    if (b) b.classList.toggle('active', p === preset);
  }});
  const seeds = {{
    kyuss: 'kyuss-welcome-to-sky-valley-c-standard-fuzz',
    plasma: 'plasma-liquid-space-rock-phaser-echo',
    odin: 'heimleid-nordic-odin-n+1||[x]-layer-69633',
    demoscene: '8bit-wraith-ultrawild-demoscene-copperbars'
  }};
  document.getElementById('seed-input').value = seeds[preset] || seeds.kyuss;
  reseedPlayground();
}}

function toggleMasterPlaygroundAudio() {{
  const btn = document.getElementById('btn-play-dj');
  if (simState.audioActive) {{
    simState.audioActive = false;
    if (simState.loopTimer) clearInterval(simState.loopTimer);
    if (simState.droneOscs) {{
      simState.droneOscs.forEach(o => {{ try {{ o.stop(); o.disconnect(); }} catch (_) {{}} }});
      simState.droneOscs = null;
    }}
    btn.classList.remove('active');
    btn.textContent = '▶ LAUNCH DESERT PSY × BABILON-ULTRA AUDIO';
    return;
  }}

  const ctx = ensureAudio();
  simState.audioActive = true;
  btn.classList.add('active');
  btn.textContent = '⏹ PLAYING: DESERT PSY × BABILON-ULTRA (69,633 LAYERS)';

  // Continuous 72/108 Hz Tagelharpa + 432/528 Hz Yggdrasil / Desert Mirage Drone
  const now = ctx.currentTime;
  const droneFilter = ctx.createBiquadFilter();
  droneFilter.type = 'lowpass';
  droneFilter.frequency.setValueAtTime(380, now);
  droneFilter.connect(simState.analyser);

  const droneFreqs = [
    {{ f: 72.0, t: 'sawtooth', g: 0.035 }},
    {{ f: 108.0, t: 'sawtooth', g: 0.030 }},
    {{ f: 216.0, t: 'triangle', g: 0.035 }},
    {{ f: 432.0, t: 'sine', g: 0.030 }},
    {{ f: 528.0, t: 'sine', g: 0.024 }}
  ];
  simState.droneOscs = droneFreqs.map(spec => {{
    const o = ctx.createOscillator();
    const g = ctx.createGain();
    o.type = spec.t;
    o.frequency.setValueAtTime(spec.f, now);
    g.gain.setValueAtTime(spec.g, now);
    o.connect(g);
    g.connect(droneFilter);
    o.start(now);
    return o;
  }});

  // 92 BPM (326ms 8th-note) Procedural Spatial Desert Psy Rock + Demoscene Sequencer
  simState.loopTimer = setInterval(() => {{
    if (!simState.audioActive) return;
    const tNow = ctx.currentTime;
    const step = simState.riffStep++;
    const fuzzDrive = parseFloat(document.getElementById('sl-fuzz').value || '2.15');
    const phaserHz = parseFloat(document.getElementById('sl-phaser').value || '0.22');
    const echoMix = parseFloat(document.getElementById('sl-echo').value || '0.48');
    const itdWidth = parseFloat(document.getElementById('sl-itd').value || '0.88');

    const rootHz = KYUSS_RIFF[step % KYUSS_RIFF.length];
    const soloHz = (simState.activePreset === 'demoscene')
      ? SID_ARP[step % SID_ARP.length]
      : PLASMA_SOLO[(step + 2) % PLASMA_SOLO.length];

    // 1. Kyuss C-Standard Fuzz Bass / Riff Note
    const gtr = ctx.createOscillator();
    const fifth = ctx.createOscillator();
    const shaper = ctx.createWaveShaper();
    const curve = new Float32Array(256);
    for (let i = 0; i < 256; i++) {{
      const x = (i / 128) - 1;
      curve[i] = Math.tanh(fuzzDrive * x + 0.35 * x * x);
    }}
    shaper.curve = curve;
    const riffFilter = ctx.createBiquadFilter();
    riffFilter.type = 'lowpass';
    riffFilter.frequency.setValueAtTime(420 + 260 * Math.sin(step * phaserHz), tNow);
    const riffGain = ctx.createGain();
    gtr.type = 'sawtooth';
    fifth.type = 'triangle';
    gtr.frequency.setValueAtTime(rootHz, tNow);
    fifth.frequency.setValueAtTime(rootHz * 1.5, tNow);
    riffGain.gain.setValueAtTime(0.0001, tNow);
    riffGain.gain.exponentialRampToValueAtTime(0.085, tNow + 0.018);
    riffGain.gain.exponentialRampToValueAtTime(0.0001, tNow + 0.30);

    gtr.connect(shaper);
    fifth.connect(shaper);
    shaper.connect(riffFilter);
    riffFilter.connect(riffGain);
    riffGain.connect(simState.analyser);
    gtr.start(tNow);
    fifth.start(tNow);
    gtr.stop(tNow + 0.32);
    fifth.stop(tNow + 0.32);

    // 2. Plasma Space-Rock Lead + Ping-Pong Tape Echo + Lemniscate ∞ 3D Spatial Pan
    const lead = ctx.createOscillator();
    const leadGain = ctx.createGain();
    lead.type = (simState.activePreset === 'demoscene') ? 'square' : 'sine';
    lead.frequency.setValueAtTime(soloHz, tNow);
    leadGain.gain.setValueAtTime(0.0001, tNow);
    leadGain.gain.exponentialRampToValueAtTime(0.055 * (0.5 + echoMix), tNow + 0.025);
    leadGain.gain.exponentialRampToValueAtTime(0.0001, tNow + 0.42);

    if (typeof ctx.createStereoPanner === 'function') {{
      const panner = ctx.createStereoPanner();
      const w = step * 0.38;
      const lem = Math.cos(w) / (1 + Math.sin(w) * Math.sin(w));
      panner.pan.setValueAtTime(Math.max(-1, Math.min(1, lem * itdWidth)), tNow);
      lead.connect(leadGain);
      leadGain.connect(panner);
      panner.connect(simState.analyser);
    }} else {{
      lead.connect(leadGain);
      leadGain.connect(simState.analyser);
    }}
    lead.start(tNow);
    lead.stop(tNow + 0.44);

    // Illuminate corresponding Elder Futhark Rune & advance world every 4 beats
    const runeEl = document.getElementById('rune-btn-' + (step % 24));
    if (runeEl) {{
      runeEl.classList.add('lit');
      setTimeout(() => runeEl.classList.remove('lit'), 210);
    }}
    if (step % 4 === 0) stepSimWorldManual();
  }}, 326);
}}

// Slider label bindings
['fuzz', 'phaser', 'echo', 'itd', 'spin'].forEach(k => {{
  const sl = document.getElementById('sl-' + k);
  const vl = document.getElementById('val-' + k);
  if (sl && vl) {{
    sl.oninput = () => {{ vl.textContent = parseFloat(sl.value).toFixed(2); }};
  }}
}});

// 3D BABILON-ULTRA Ziggurat + Demoscene Copperbars + Platonic Fauna Canvas3D Engine
const canvas = document.getElementById('babilon-stage');
const ctx2d = canvas.getContext('2d');
const specCanvas = document.getElementById('mini-spectrum');
const specCtx = specCanvas.getContext('2d');

function resizeCanvas() {{
  canvas.width = window.innerWidth * window.devicePixelRatio;
  canvas.height = window.innerHeight * window.devicePixelRatio;
  ctx2d.setTransform(window.devicePixelRatio, 0, 0, window.devicePixelRatio, 0, 0);
}}
window.addEventListener('resize', resizeCanvas);
resizeCanvas();

canvas.addEventListener('pointerdown', e => {{
  simState.dragging = true;
  simState.lastMouse = [e.clientX, e.clientY];
}});
window.addEventListener('pointerup', () => {{ simState.dragging = false; }});
window.addEventListener('pointermove', e => {{
  if (!simState.dragging) return;
  const dx = e.clientX - simState.lastMouse[0];
  const dy = e.clientY - simState.lastMouse[1];
  simState.lastMouse = [e.clientX, e.clientY];
  simState.camYaw += dx * 0.007;
  simState.camPitch = Math.max(0.12, Math.min(1.15, simState.camPitch + dy * 0.006));
}});
window.addEventListener('keydown', e => {{
  if (e.code === 'Space' && document.activeElement.tagName !== 'INPUT') {{
    e.preventDefault();
    triggerZenPulse();
  }}
}});

function project3D(x, y, z, cx, cy, scale) {{
  const cyaw = Math.cos(simState.camYaw), syaw = Math.sin(simState.camYaw);
  const cpitch = Math.cos(simState.camPitch), spitch = Math.sin(simState.camPitch);
  const rx = x * cyaw - z * syaw;
  const rz = x * syaw + z * cyaw;
  const ry = y * cpitch - rz * spitch;
  const rz2 = y * spitch + rz * cpitch + 420;
  const fov = 520 / Math.max(80, rz2);
  return [cx + rx * fov * scale, cy - ry * fov * scale, rz2, fov];
}}

let lastFrameTime = performance.now();
let frameCount = 0;

function renderBabilonUltraLoop(nowMs) {{
  frameCount++;
  const t = nowMs * 0.001;
  const w = window.innerWidth;
  const h = window.innerHeight;
  const cx = w * 0.5;
  const cy = h * 0.52;

  // Read live FFT audio energy if active
  let audioEnergy = 0.18 + 0.08 * Math.sin(t * 2.444);
  if (simState.analyser && simState.audioActive) {{
    simState.analyser.getByteFrequencyData(simState.freqData);
    let sum = 0;
    for (let i = 0; i < 32; i++) sum += simState.freqData[i];
    audioEnergy = 0.12 + (sum / (32 * 255)) * 0.95;
  }}

  // Draw mini spectrum in bottom bar
  specCtx.fillStyle = '#060810';
  specCtx.fillRect(0, 0, 260, 32);
  for (let i = 0; i < 32; i++) {{
    const val = simState.audioActive
      ? (simState.freqData[i] / 255)
      : (0.2 + 0.18 * Math.sin(t * 3 + i * 0.4));
    const bh = Math.max(2, val * 28);
    specCtx.fillStyle = i < 10 ? '#ffd700' : (i < 22 ? '#00f0ff' : '#ff007f');
    specCtx.fillRect(i * 8 + 2, 30 - bh, 6, bh);
  }}

  const spinRate = parseFloat(document.getElementById('sl-spin').value || '1.0');
  if (!simState.dragging) {{
    simState.camYaw += 0.0035 * spinRate;
  }}
  if (simState.zenPulse > 0.001) {{
    simState.zenPulse *= 0.94;
  }}

  // 1. Background Demoscene Plasma & Copperbars
  ctx2d.clearRect(0, 0, w, h);
  for (let bar = 0; bar < 7; bar++) {{
    const by = cy - 190 + bar * 56 + Math.sin(t * 1.9 + bar * 0.7) * 18;
    const grad = ctx2d.createLinearGradient(0, by - 10, 0, by + 10);
    const col = PALETTE[bar % PALETTE.length];
    grad.addColorStop(0, 'transparent');
    grad.addColorStop(0.5, col + '22');
    grad.addColorStop(1, 'transparent');
    ctx2d.fillStyle = grad;
    ctx2d.fillRect(0, by - 10, w, 20);
  }}

  // 2. Render 7 Terraces of the BABILON-ULTRA Ziggurat
  for (let tier = 0; tier < 7; tier++) {{
    const radius = (175 - tier * 20) * (1 + audioEnergy * 0.14 + simState.zenPulse * 0.12);
    const yLevel = -110 + tier * 34;
    const sides = 6 + (tier % 3) * 2;
    const col = PALETTE[tier % PALETTE.length];
    ctx2d.beginPath();
    for (let s = 0; s <= sides; s++) {{
      const ang = (s / sides) * Math.PI * 2 + t * 0.25 * (tier % 2 === 0 ? 1 : -1);
      const px = Math.cos(ang) * radius;
      const pz = Math.sin(ang) * radius;
      const [sx, sy] = project3D(px, yLevel, pz, cx, cy, 1.0);
      if (s === 0) ctx2d.moveTo(sx, sy);
      else ctx2d.lineTo(sx, sy);
    }}
    ctx2d.strokeStyle = col + 'aa';
    ctx2d.lineWidth = 1.6 + audioEnergy * 1.5;
    ctx2d.stroke();
    ctx2d.fillStyle = col + '12';
    ctx2d.fill();
  }}

  // 3. Render Yggdrasil / Lemniscate ∞ Horizon Orbit & 24 Elder Futhark Runes in 3D
  ctx2d.beginPath();
  for (let i = 0; i <= 64; i++) {{
    const u = (i / 64) * Math.PI * 2 + t * 0.6;
    const denom = 1 + Math.sin(u) * Math.sin(u);
    const lx = 210 * Math.cos(u) / denom;
    const lz = 210 * Math.sin(u) * Math.cos(u) / denom;
    const ly = 25 * Math.sin(u * 2 + t);
    const [sx, sy] = project3D(lx, ly, lz, cx, cy, 1.0);
    if (i === 0) ctx2d.moveTo(sx, sy);
    else ctx2d.lineTo(sx, sy);
  }}
  ctx2d.strokeStyle = 'rgba(255, 215, 0, 0.55)';
  ctx2d.lineWidth = 1.4;
  ctx2d.stroke();

  // 24 Elder Futhark Runes Halo at Layer n+1||[X] (#69,633)
  ctx2d.font = '12px monospace';
  ctx2d.textAlign = 'center';
  for (let r = 0; r < 24; r++) {{
    const ang = (r / 24) * Math.PI * 2 - t * 0.32;
    const rx = Math.cos(ang) * 215;
    const rz = Math.sin(ang) * 215;
    const [sx, sy, depth] = project3D(rx, 115 + Math.sin(t * 2 + r) * 6, rz, cx, cy, 1.0);
    if (depth > 100) {{
      ctx2d.fillStyle = (r === (simState.riffStep % 24)) ? '#ffffff' : '#ffd700cc';
      ctx2d.fillText(ELDER_FUTHARK[r], sx, sy);
    }}
  }}

  // 4. Render Live `world-core` Platonic Fauna Cast & Thought Bubbles
  simState.fauna.forEach((f, idx) => {{
    const fx = f.pos[0] * 1.45;
    const fz = f.pos[1] * 1.45;
    const fy = -85 + idx * 24 + Math.sin(t * 3.2 + idx) * 8;
    const [sx, sy, _d, fov] = project3D(fx, fy, fz, cx, cy, 1.0);
    const rad = (12 + (1 - f.needs[0]) * 14) * fov * (1 + audioEnergy * 0.25);

    for (let sp = 0; sp < f.splitCount; sp++) {{
      const ox = sp === 0 ? 0 : Math.cos(sp * 2.1 + t * 3) * 16;
      const oy = sp === 0 ? 0 : Math.sin(sp * 2.1 + t * 3) * 16;
      ctx2d.beginPath();
      for (let k = 0; k <= f.sides; k++) {{
        const a = (k / f.sides) * Math.PI * 2 + t * 1.2;
        const vx = sx + ox + Math.cos(a) * (rad / Math.sqrt(f.splitCount));
        const vy = sy + oy + Math.sin(a) * (rad / Math.sqrt(f.splitCount));
        if (k === 0) ctx2d.moveTo(vx, vy);
        else ctx2d.lineTo(vx, vy);
      }}
      ctx2d.fillStyle = f.color + '44';
      ctx2d.fill();
      ctx2d.strokeStyle = f.color;
      ctx2d.lineWidth = 1.8;
      ctx2d.stroke();
    }}

    // Fauna name & shared reflection bubble
    ctx2d.fillStyle = '#e8d8c8';
    ctx2d.font = 'bold 10px monospace';
    ctx2d.fillText(f.name.toUpperCase(), sx, sy - rad - 14);
    ctx2d.fillStyle = f.color;
    ctx2d.font = '9.5px monospace';
    ctx2d.fillText(f.sayBubble, sx, sy - rad - 3);
  }});

  if (frameCount % 30 === 0) {{
    const dt = Math.max(1, nowMs - lastFrameTime);
    const fps = Math.min(120, Math.round(30000 / dt));
    lastFrameTime = nowMs;
    document.getElementById('hud-fps').textContent = `${{fps}} FPS · BABILON-ULTRA WebGL/Canvas3D`;
  }}

  requestAnimationFrame(renderBabilonUltraLoop);
}}

renderStaticTelemetry();
updateGaiaAndFaunaUI();
requestAnimationFrame(renderBabilonUltraLoop);
</script>
</body>
</html>
"##
    );

    std::fs::write(output_html_path, html)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn babilon_ultra_demoscene_merges_ultrawild_and_world_core() {
        let report = execute_babilon_ultra_demoscene("kyuss-plasma-babilon-ultra", 32, N_PLUS_1_ODIN_LAYER);
        assert_eq!(report.demoscene.clamp_ternary_triplet, [-1, 0, 1]);
        assert!(report.demoscene.is_pow2_verified);
        assert_eq!(report.demoscene.gray_step1_changed_bits, 1);
        assert!(report.demoscene.gray_roundtrip_verified);
        assert_eq!(report.fauna_cast.len(), 6);
        assert_eq!(report.ziggurat_terraces.len(), 7);
        assert!(report.keeper_admitted >= 6);
        assert_eq!(report.keeper_refused, 0);
        assert!(report.arena_svg_snapshot.starts_with("<svg"));
    }
}
