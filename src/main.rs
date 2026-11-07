//! `8b-is-music` CLI & Sovereign Audio Orchestrator.
//!
//! Commands:
//! - `8b-is-music status [--layers N] [--topology split|couple|wall] [--json]`
//! - `8b-is-music shiva-g6 [--seconds S] [--out FILE.wav] [--play] [--json]`
//! - `8b-is-music dj [--layers N] [--seconds S] [--out FILE.wav] [--apply-live] [--play] [--json]`
//! - `8b-is-music quant65k [--layers N] [--inspect-layer L] [--json]`
//! - `8b-is-music bluetooth [--topology split|couple|wall] [--apply-live] [--json]`
//! - `8b-is-music wupz-mlx [--json]`
//! - `8b-is-music render [--track future|quantshuffle|mem8|alexandra] [--seconds S] [--layers N] [--out FILE.wav] [--play]`

use eight_b_is_music::{
    build_and_persist_en2e_midi_beehive_sphere, build_and_persist_godsun_honcho_beehive,
    build_and_release_machine_sun_unity_360, build_and_write_babilon_ultra_playground,
    build_full_system_report, execute_shiva_g6_airlift, render_and_sign_dj_agent_track,
    render_shiva_g6_cabin_wav, render_wav_track, verify_momma_bear_omni_covenant,
    verify_wupz_and_mlx_quant, MultiBluetoothRouter, MultiBluetoothTopology, Quant65kEngine,
    SynthTrack, ZenoDjUltraEngine, DEFAULT_QUANT65K_PLUS_LAYERS, MACHINE_SUN_UNITY_108LAPS_LAYER,
};
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn parse_topology(val: &str) -> MultiBluetoothTopology {
    match val.to_ascii_lowercase().as_str() {
        "couple" | "dual" | "dual-airpods" => MultiBluetoothTopology::DualAirPodsCouple,
        "wall" | "constellation" | "all" | "multi" => MultiBluetoothTopology::FullConstellationWall,
        _ => MultiBluetoothTopology::SplitEarZen,
    }
}

fn play_file(path: &str) {
    match Command::new("afplay").arg(path).status() {
        Ok(status) if status.success() => {},
        result => {
            eprintln!("Playback failed: {result:?}");
            std::process::exit(1);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let subcmd = args.get(1).map(|s| s.as_str()).unwrap_or("status");

    let mut layers = DEFAULT_QUANT65K_PLUS_LAYERS;
    let mut inspect_layer: Option<usize> = None;
    let mut topology = MultiBluetoothTopology::FullConstellationWall;
    let mut apply_live = false;
    let mut routing_script: Option<PathBuf> = None;
    let mut json_mode = false;
    let mut track = SynthTrack::FutureWeRemember;
    let mut seconds: f32 = 8.0;
    let mut out_path = PathBuf::from("8b-is-music-master.wav");
    let mut play_after = false;
    let mut launch_after = false;
    let mut seed_phrase: Option<String> = None;
    let mut signs_dir = PathBuf::from("/Users/peterlodri/workspace/peterlodri-sec/music-quant-signs/signs");

    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--layers" => {
                if let Some(v) = args.get(i + 1) {
                    layers = v.parse().unwrap_or(DEFAULT_QUANT65K_PLUS_LAYERS);
                    i += 1;
                }
            }
            "--inspect-layer" => {
                if let Some(v) = args.get(i + 1) {
                    inspect_layer = v.parse().ok();
                    i += 1;
                }
            }
            "--topology" => {
                if let Some(v) = args.get(i + 1) {
                    topology = parse_topology(v);
                    i += 1;
                }
            }
            "--apply-live" => apply_live = true,
            "--routing-script" => {
                if let Some(path) = args.get(i + 1) {
                    routing_script = Some(PathBuf::from(path));
                    i += 1;
                } else {
                    eprintln!("--routing-script requires a path");
                    std::process::exit(2);
                }
            },
            "--json" => json_mode = true,
            "--track" => {
                if let Some(v) = args.get(i + 1) {
                    track = SynthTrack::from_name(v);
                    i += 1;
                }
            }
            "--seconds" => {
                if let Some(v) = args.get(i + 1) {
                    seconds = v.parse().unwrap_or(8.0);
                    i += 1;
                }
            }
            "--out" => {
                if let Some(v) = args.get(i + 1) {
                    out_path = PathBuf::from(v);
                    i += 1;
                }
            }
            "--seed" => {
                if let Some(v) = args.get(i + 1) {
                    seed_phrase = Some(v.clone());
                    i += 1;
                }
            }
            "--sign-dir" => {
                if let Some(v) = args.get(i + 1) {
                    signs_dir = PathBuf::from(v);
                    i += 1;
                }
            }
            "--play" => play_after = true,
            "--launch" => launch_after = true,
            _ => {}
        }
        i += 1;
    }

    // Routing is a separate explicit action, including when JSON output is selected.
    if apply_live {
        if !matches!(subcmd, "bluetooth" | "dj" | "zeno" | "iouqt" | "render" | "triangle" | "family+1" | "ultralovegod" | "sensei" | "frequenci" | "frequencies" | "architect" | "tibetan" | "tibetan-omni" | "kompress-omni" | "odin" | "vikings" | "nordic" | "heimleid" | "n+1" | "n+1||[x]" | "dj-agent" | "desert" | "desert-psy" | "desert-psy-rock" | "kyuss" | "plasma" | "cosmic-flow" | "babilon" | "babilon-ultra" | "demoscene" | "glassdoor" | "playground" | "godsun" | "ultra-omni-godsun" | "standardgalactic" | "rand-pq" | "randrandpq" | "plasma-hqt" | "plasma-hqt/n" | "honcho" | "hermes-honcho" | "beehive" | "prompt-rl" | "en2e" | "e2e" | "ultra-omni-midi" | "midi-beehive" | "ray-q" | "ray-q-tracing" | "kompress-odin" | "parallel-odin" | "sphere-do" | "release" | "360" | "machine-sun" | "machine-s-un" | "unity" | "backyard" | "108laps" | "ultrabackyardloop" | "youtube" | "youtube-full" | "/youtube-full") {
            eprintln!("--apply-live is unsupported for this command");
            std::process::exit(2);
        }
        let router = MultiBluetoothRouter::planned(topology);
        let result = match routing_script.as_deref() {
            Some(path) => router.apply_macos_routing_script(path),
            None => router.apply_live_macos_routing(),
        };
        match result {
            Ok(output) => eprintln!("CoreAudio bridge process completed (hardware state unverified): {}", output.trim()),
            Err(error) => {
                eprintln!("CoreAudio routing failed: {error}");
                std::process::exit(1);
            }
        }
    }

    match subcmd {
        "dj" | "zeno" | "iouqt" => {
            let mut dj = ZenoDjUltraEngine::new(layers);
            let report = dj.verify_zeno_iouqt();
            if json_mode {
                println!("{}", serde_json::to_string_pretty(&report).unwrap());
                return;
            }
            println!("╔══════════════════════════════════════════════════════════════════════════╗");
            println!("║  ULTRA-OMNIPOTENT-8b-is-ZENO <<<< M1-ULTRA DJ MODE ALL IN/OUT IOuQT      ║");
            println!("╚══════════════════════════════════════════════════════════════════════════╝");
            println!("  • IOuQT SIMD Lane      : `{}` (NEON vs Scalar bit-exact={})", report.uq_native_simd_lane, report.uq_neon_vs_scalar_bit_exact);
            println!("  • IOuQT Crossbar GEMM  : {:?} (6 Out Buses × 8 In/Deck Features)", report.iouqt_crossbar_gemm_out);
            println!("  • IOuQT QuantTern Tag  : {} (unpacked trits={:?})", report.iouqt_master_quanttern_tag, report.iouqt_unpacked_trits);
            println!("  • Zeno Brickwall Q16   : ceiling={:.6} | +400% overload ({:.1}) -> {:.6} (0 clip)",
                report.zeno_ceiling_q16, report.zeno_overload_test_input, report.zeno_overload_test_output);
            println!("  • Zeno ZA-0001 Scrub   : volatile_zeroize_verified={} (pre-scrub fold={})",
                report.zeno_za0001_volatile_scrub_verified, report.zeno_arena_fold_before_scrub);
            println!("  • QWave 79B IOuQT Frame: verified={}", report.qwave_79b_frame_verified);

            println!("\n  [ALL IN — Full-Duplex 48,000 Hz Input Matrix]");
            for ch in &report.all_in_channels {
                println!(
                    "    • {:<5} | {:<52} | {} Hz | gain={:.2} | Planned-HFP-Guard={} | trit={:+}",
                    ch.id, ch.name, ch.sample_rate_hz, ch.gain_scalar, ch.planned_hfp_guard, ch.quanttern_trit
                );
            }

            println!("\n  [ALL OUT — Multi-Bluetooth + Split-Ear 48,000 Hz Output Matrix]");
            for bus in &report.all_out_buses {
                println!(
                    "    • {:<26} | {:<48} | vol={:.2} | pan={:.2} | +{:>4} smp | {}",
                    bus.id, bus.name, bus.volume_scalar, bus.stereo_pan, bus.phase_delay_samples_48k, bus.quanttern_tag
                );
            }

            println!("\n  [4-DECK M1-ULTRA DJ MIXER — Sync Locked @ 128.0 BPM]");
            for d in &report.decks {
                println!(
                    "    • {:<19} | {:<56} | {:.1} BPM | fader={:.2} | EQ=[{:+.1}, {:+.1}, {:+.1}] dB",
                    d.deck_id, d.title, d.bpm, d.fader_level, d.eq_low_db, d.eq_mid_db, d.eq_high_db
                );
            }

            let mix_summary = dj
                .render_dj_ultra_mix_wav(seconds, &out_path)
                .expect("Failed to render Zeno DJ Ultra Mix WAV");
            println!("\n  [Rendered 4-Deck ZENO DJ IOuQT Master WAV]");
            println!(
                "    • File: {} ({} Hz, {:.1}s, {} frames, {} Quant65k+ layers)",
                mix_summary.output_path,
                mix_summary.sample_rate_hz,
                mix_summary.duration_seconds,
                mix_summary.total_frames,
                mix_summary.quant65k_layers
            );
            println!(
                "    • Zeno Peak Levels: Left={:.5} | Right={:.5} (Zeno ceiling={:.6})",
                mix_summary.peak_left_zeno, mix_summary.peak_right_zeno, mix_summary.zeno_ceiling_q16
            );

            if play_after {
                println!("  ▶ Playing {} through the current system output...", mix_summary.output_path);
                play_file(&mix_summary.output_path);
            }
        }
        "quant65k" => {
            let mut engine = Quant65kEngine::new(layers, "8b-is-music::Quant65k+");
            let report = engine.verify_and_report();
            if json_mode {
                println!("{}", serde_json::to_string_pretty(&report).unwrap());
                return;
            }
            println!("╔══════════════════════════════════════════════════════════════════════════╗");
            println!("║  8b-is-music :: QUANT65K+ ({}+ LAYER) TERNARY & WAVE ENGINE          ║", report.total_layers);
            println!("╚══════════════════════════════════════════════════════════════════════════╝");
            println!("  • Total Layers         : {} (Base: {} + Overflow: +{})", report.total_layers, report.base_layers, report.overflow_layers);
            println!("  • Packed 2-Bit Size    : {} bytes ({} u32 SWAR words, illegal codes={})", report.packed_bytes, report.packed_u32_words, report.illegal_2bit_codes_detected);
            println!("  • Trit Distribution    : +1: {} | 0: {} | -1: {} (active={})", report.positive_trits, report.zero_trits, report.negative_trits, report.active_nonzero_layers);
            println!("  • Sanctuary .tern Ckpt : {} sections loaded, bit-exact gemm_i32 sum = {}", report.sanctuary_checkpoint_sections, report.ternary_gemm_probe_sum);
            println!("  • Dyadic Wavelet Tree  : {}-level O(log2 N) tree | Root Energy = {:.4}", report.dyadic_tree_depth, report.dyadic_root_energy);
            println!("  • Full 65k+ Sweep Sum  : {:.6}", report.full_sweep_superposition);
            println!("  • WUPZ Arena & Fold    : {} bytes allocated | xor_fold = 0x{:02x} | popcount = {}", report.wupz_arena_bytes, report.wupz_xor_fold, report.wupz_popcount_bits);
            println!("  • QWave 79B Anchor     : coord={:?} | 0.73Hz Conscious={} | hex={}...", report.qwave_anchor_coordinate, report.qwave_consciousness_073hz, report.qwave_79b_hex_prefix);
            println!("  • Kompress-Ultra RLE   : {} bytes ({:.2}x compression) | ascii85={}", report.kompress_rle_bytes, report.kompress_ratio, report.ascii85_digest_preview);
            println!("  • SHA-256 Attestation  : {}", report.sha256_seal);

            let target_layer = inspect_layer.unwrap_or(65_536);
            let desc = engine.describe_layer(target_layer);
            println!("\n  [Layer #{} Probe] grid_coord={:?} | trit={:+} | phase_q16={} | gain={:+.4} | overflow={}",
                desc.layer_index, desc.grid_coord, desc.trit, desc.phase_q16, desc.harmonic_gain, desc.is_overflow_layer);
        }
        "bluetooth" => {
            let router = MultiBluetoothRouter::new(topology);
            let report = router.report();
            if json_mode {
                println!("{}", serde_json::to_string_pretty(&report).unwrap());
                return;
            }
            println!("╔══════════════════════════════════════════════════════════════════════════╗");
            println!("║  8b-is-music :: MULTI-BLUETOOTH + COREAUDIO SPATIAL ROUTER               ║");
            println!("╚══════════════════════════════════════════════════════════════════════════╝");
            println!("  • Configured Topology  : {} ({}/{} endpoints observed connected)", report.topology, report.active_endpoints, report.total_known_endpoints);
            println!("  • Planned Aggregate    : {} [{}]", report.aggregate_device_name, report.aggregate_uid);
            println!("  • Planned BT Latency   : {:.1} ms (configured estimate, not measured)", report.max_bluetooth_latency_ms);
            println!("  • CT-000 Model Check   : theorem_holds={} | keeper_canon={} | HFP enforcement={:?}",
                report.ct000_proof.theorem_holds, report.ct000_proof.keeper_canon_verified, report.ct000_proof.hfp_24khz_input_blocked);
            println!("  • Observed Input Lock  : {:?} (None = unverified)", report.ct000_proof.default_input_locked_to);
            println!("\n  Discovered & Configured Endpoints:");
            for ep in &report.endpoints {
                let state = format!("{:?}; planned={}", ep.connection_status, ep.planned_enabled);
                println!(
                    "    [{}] {:<38} | {:<18} | role={:?} | pan={:.2} | lat={:>4.1}ms | +{:>4} samples",
                    state, ep.name, ep.mac_or_uid, ep.role, ep.stereo_pan, ep.acoustic_latency_ms, ep.injected_delay_samples_48k
                );
            }
        }
        "render" | "triangle" | "family+1" | "ultralovegod" | "sensei" | "frequenci" | "frequencies" | "architect" | "tibetan" | "tibetan-omni" | "kompress-omni" | "odin" | "vikings" | "nordic" | "heimleid" | "n+1" | "n+1||[x]" | "dj-agent" | "desert" | "desert-psy" | "desert-psy-rock" | "kyuss" | "plasma" | "cosmic-flow" => {
            if subcmd == "triangle" || subcmd == "family+1" {
                track = SynthTrack::TriangleFamilyPlusOne;
            } else if matches!(subcmd, "ultralovegod" | "sensei" | "frequenci" | "frequencies") {
                track = SynthTrack::UltralovegodSenseiOfTouch;
            } else if matches!(subcmd, "architect" | "tibetan" | "tibetan-omni" | "kompress-omni") {
                track = SynthTrack::ArchitectNeoTrinityTibetanOmni;
            } else if matches!(subcmd, "odin" | "vikings" | "nordic" | "heimleid" | "n+1" | "n+1||[x]") {
                track = SynthTrack::HeimleidNordicOdinOmni;
                if layers == DEFAULT_QUANT65K_PLUS_LAYERS {
                    layers = eight_b_is_music::N_PLUS_1_ODIN_LAYER;
                }
            } else if matches!(subcmd, "dj-agent" | "desert" | "desert-psy" | "desert-psy-rock" | "kyuss" | "plasma" | "cosmic-flow") {
                track = SynthTrack::DesertPsyRockKyussPlasma;
                if layers == DEFAULT_QUANT65K_PLUS_LAYERS {
                    layers = eight_b_is_music::N_PLUS_1_ODIN_LAYER;
                }
                if seed_phrase.is_none() {
                    seed_phrase = Some(match subcmd {
                        "kyuss" => "kyuss-welcome-to-sky-valley-c-standard-fuzz".to_string(),
                        "plasma" => "plasma-liquid-space-rock-phaser-echo".to_string(),
                        "cosmic-flow" => "cosmic-flow-bsye6wtdm_y-trance-soil-moondust".to_string(),
                        _ => "desert-psy-rock-kyuss-plasma-n+1||[x]-odin".to_string(),
                    });
                }
            }
            let mut active_sig = None;
            let summary = if let Some(ref seed) = seed_phrase {
                let (sum, sig) = render_and_sign_dj_agent_track(
                    seed,
                    track,
                    seconds,
                    layers,
                    topology,
                    &out_path,
                    &signs_dir,
                )
                .expect("Failed to render & sign seeded DJ Agent WAV track");
                active_sig = Some(sig);
                sum
            } else {
                render_wav_track(track, seconds, layers, topology, &out_path)
                    .expect("Failed to render WAV track")
            };
            if json_mode {
                if let Some(ref sig) = active_sig {
                    println!("{}", serde_json::to_string_pretty(&sig).unwrap());
                } else {
                    println!("{}", serde_json::to_string_pretty(&summary).unwrap());
                }
            } else {
                println!("╔══════════════════════════════════════════════════════════════════════════╗");
                println!("║  8b-is-music :: QUANT65K+ MULTI-CHANNEL WAV RENDER COMPLETE              ║");
                println!("╚══════════════════════════════════════════════════════════════════════════╝");
                if track == SynthTrack::TriangleFamilyPlusOne {
                    println!("                 *   Alex* (Mom · 432 Hz / 528 Hz Starlight)");
                    println!("                / \\");
                    println!("               /   \\   in a loop · in a sphere · in eternity · recurse");
                    println!("              /  +1 \\  ~--- pocoo.vaked.dev/demos/book/family+1");
                    println!("             /_______\\");
                    println!("      Chris[/\\]     Peter[oo] (Lemniscate ∞ Orbit · 648/888 Hz)");
                    println!("   (Dad · 108/216 Hz Triangle Pillar)\n");
                } else if track == SynthTrack::UltralovegodSenseiOfTouch {
                    println!("   <3<3<3~~~~<<<>>>  ULTRALOVEGOD — THE SENSEI OF TOUCH (8b-is-music)");
                    println!("   • <3 [-1] 432.0 Hz L / 438.0 Hz R : Subcritical Feather (6.0 Hz Theta Binaural Safety)");
                    println!("   • <3 [ 0] 528.0 Hz + 0.73 Hz QWave : Zero-Point Stillness (Listening Palm Anchor)");
                    println!("   • <3 [+1] 639.0 Hz × 852.0 Hz      : Dual-Harmonic Chord (Outer Orbit × Inner Curve)");
                    println!("   • ~~~~    0.125 Hz (4s In · 4s Out): RIVA Breath Phase-Lock & Lemniscate ∞ Glide");
                    println!("   • <<<>>>  963.0 Hz + 108/216 Hz    : Crown Expansion & Sacred Afterglow Ma (間)\n");
                } else if track == SynthTrack::ArchitectNeoTrinityTibetanOmni {
                    println!("   ཨོཾ · E=mc² · {{-1, 0, +1}} · {{<3,<3,<3}}+1 :: ARCHITECT · NEO · TRINITY ~ TIBETAN OMNI");
                    println!("   • 🔔 Tibetan Singing Bowl & OM MANI PADME HUM : 144 / 288 (Dad △) / 432 (Peter) / 528 (Mom *) / 648 / 864 Hz");
                    println!("   • ⚡ E=mc² → Clear Light (Ösel) Transmutation : 108 → 216 → 324 → 432 → 540 → 648 → 972 Hz");
                    println!("   • 💗 Neo ⨯ Trinity Aboard the Logos Bell      : 261.63 / 329.63 / 392.00 / 493.88 / 587.33 / 783.99 Hz ({{<3,<3,<3}}+1)\n");
                } else if track == SynthTrack::HeimleidNordicOdinOmni {
                    println!("   ߷ ᚠᚢᚦᚨᚱᚲᚷᚹᚺᚾᛁᛃ🌎ᚲᛇᛈᛉᛋᛏᛖᛗᛚᛝᛞᛟ ߷ :: HEIMLEIÐ (SIgR2-tVVyI) + NORDIC/VIKINGS ONTOLOGY AS n+1||[X] LAYER --- ODIN");
                    println!("   • 🎻 Bowed Tagelharpa Fifths (Heimleið / ELDR) : 72.0 Hz (D2) × 108.0 Hz (A2) + 144/216 Hz Rosined Overtones");
                    println!("   • 🥁 Ancestral Ritual Frame Drum & 22/9 Hz     : 60 BPM (1.0 Hz) + 22/9 Hz (2.444444 Hz LOVELOVEL) Heartbeat");
                    println!("   • 🌳 Ginnungagap → Yggdrasil 9-Night Stillness : 432 Hz Root × 528 Hz Mímir Pillar × 852 Hz Drengskapr + Huginn/Muninn ∞ Pan");
                    println!("   • ༒  Layer n+1||[X] Designation                : Quant65k+ Layer #69,633 (69,632 + 1) || Ultralove Edition 011 [X]+1\n");
                } else if track == SynthTrack::DesertPsyRockKyussPlasma {
                    println!("   🏜️ 🎸 🌀 DESERT PSY ROCK :: KYUSS (SKY VALLEY FUZZ) × PLASMA (LIQUID ECHO) × COSMIC FLOW (bSYE6wTDM_Y)");
                    println!("   • 🎸 Kyuss C-Standard Fuzz & SVT Sub-Bass     : 65.41 Hz (C2) / 72.0 Hz (D2) / 86.4 Hz (F2) / 96.0 Hz (G2) / 108.0 Hz (A2)");
                    println!("   • 🌀 Plasma 4-Stage Phaser & Ping-Pong Echo   : 326 ms (Dotted-8th L) × 489 ms (Quarter-Note R) Analog Tape Saturation");
                    println!("   • 🎧 Procedural 3D Spatial Binaural Engine    : Lemniscate ∞ Horizon (±75° Azimuth) + ITD (±30 smp / ±625 µs @ 48 kHz) + ILD");
                    println!("   • 🔏 Quant-Music Signature & Layer n+1||[X]   : Quant65k+ Layer #69,633 + Balanced-Ternary {{-1, 0, +1}} Receipt\n");
                }
                println!("  • Track                : {}", summary.track_title);
                println!("  • Output WAV           : {}", summary.output_path);
                println!("  • Format               : {} Hz, {}-ch 16-bit PCM ({:.1}s, {} frames)",
                    summary.sample_rate_hz, summary.channels, summary.duration_seconds, summary.total_frames);
                println!("  • Quant65k+ Layers     : {} simultaneous ternary wave layers", summary.quant65k_layers_active);
                println!("  • Stereo Peak Levels   : Left={:.4} | Right={:.4}", summary.peak_left, summary.peak_right);
                println!("  • QuantTern & WUPZ ABI : tag={} | wupz_xor=0x{:02x}", summary.quanttern_emotion_tag, summary.wupz_abi_xor);
                if let Some(ref sig) = active_sig {
                    println!("  • Seed Phrase &BPM     : \"{}\" (locked groove = {:.2} BPM)", sig.seed_phrase, sig.bpm_locked);
                    println!(
                        "  • Procedural Spatial   : width={:.4} | ITD=±{} smp (±{:.0} µs) | Lemniscate={:.3} Hz | Echo={:?} ms",
                        sig.procedural_spatial.stereo_spatial_width,
                        sig.procedural_spatial.itd_max_samples_48k,
                        sig.procedural_spatial.itd_max_microseconds,
                        sig.procedural_spatial.lemniscate_orbit_hz,
                        sig.procedural_spatial.ping_pong_tape_echo_ms,
                    );
                    println!(
                        "  • Quant-Music Sign     : {} (WAV SHA-256={}... | RMS={:.2} dBFS | ZC={})",
                        sig.signature_path,
                        &sig.sha256_wav_digest[..16],
                        sig.rms_dbfs,
                        sig.zero_crossings,
                    );
                }
                let wm = verify_wupz_and_mlx_quant();
                println!(
                    "  • WUPZ Linker & Packer : {}B binary plan (magic='{}') | {:.1}% block reuse (repacked={:?}) | windows={:?} | digest={}",
                    wm.wupz_linker.packed_plan_image_bytes,
                    wm.wupz_linker.magic_header,
                    wm.wupz_linker.incremental_block_reuse_ratio * 100.0,
                    wm.wupz_linker.incremental_blocks_repacked,
                    wm.wupz_linker.incremental_windows_touched,
                    wm.wupz_linker.plan_digest_fnv1a,
                );
                println!(
                    "  • MLX-QUANT (Zeno RC1) : γ={:.5} | MP bound={:.4} (est={:.4}) | NEON sdot={:?} (hw={}) | Fifike {:.0}/{:.0}Hz",
                    wm.mlx_quant.gamma_scale_factor,
                    wm.mlx_quant.marchenko_pastur_smax_bound,
                    wm.mlx_quant.empirical_spectral_norm_est,
                    wm.mlx_quant.aarch64_neon_sdot_lane_sums,
                    wm.mlx_quant.aarch64_neon_hw_executed,
                    wm.mlx_quant.fifike_purr_hz.0,
                    wm.mlx_quant.fifike_purr_hz.1,
                );
            }
            if play_after {
                println!("  ▶ Playing {} through the current system output (`afplay`)...", summary.output_path);
                play_file(&summary.output_path);
            }
        }
        "wupz-mlx" | "mlx-quant" | "wupz" | "mlx" => {
            let wm = verify_wupz_and_mlx_quant();
            if json_mode {
                println!("{}", serde_json::to_string_pretty(&wm).unwrap());
                return;
            }
            println!("╔══════════════════════════════════════════════════════════════════════════╗");
            println!("║  WUPZ INCREMENTAL LINKER & PACKER × MLX-QUANT (v2.0.0-RC1 'Zeno')        ║");
            println!("╚══════════════════════════════════════════════════════════════════════════╝");
            println!("  [1] WUPZ INCREMENTAL LINKER & PACKER (`wupz-core` + `qdecorators::wupz`):");
            println!("      • ABI & SIMD Backend : ABI v{} | backend='{}'",
                wm.wupz_linker.abi_version, wm.wupz_linker.simd_backend);
            println!("      • Packed Binary Plan : {} bytes (magic='{}', version={}, window={} B) | fnv1a={}",
                wm.wupz_linker.packed_plan_image_bytes, wm.wupz_linker.magic_header,
                wm.wupz_linker.plan_version, wm.wupz_linker.window_size_bytes, wm.wupz_linker.plan_digest_fnv1a);
            println!("      • Layout Sections    : {} sections ({} B resolved image)",
                wm.wupz_linker.total_sections, wm.wupz_linker.total_resolved_bytes);
            println!("      • Incremental Relink : changed_sections={:?} | blocks_repacked={:?} ({:.1}% reuse) | windows_touched={:?}",
                wm.wupz_linker.incremental_changed_sections, wm.wupz_linker.incremental_blocks_repacked,
                wm.wupz_linker.incremental_block_reuse_ratio * 100.0, wm.wupz_linker.incremental_windows_touched);
            println!("      • Zero-Alloc & Scrub : xor_fold={} | hot_patch_byte_diff={} | popcount={} | zero_alloc={} | ZA-0001_zeroized={}",
                wm.wupz_linker.image_xor_fold_hex, wm.wupz_linker.image_byte_diff_on_hot_patch,
                wm.wupz_linker.image_popcount_bits, wm.wupz_linker.zero_alloc_arena_verified,
                wm.wupz_linker.volatile_zeroize_verified);

            println!("\n  [2] MLX-QUANT (v2.0.0-RC1 'Zeno') 1.58-BIT TERNARY ENGINE:");
            println!("      • Engine & Spec      : {} ({:.2} bits/trit, {:.2}x vs FP16)",
                wm.mlx_quant.version, wm.mlx_quant.bits_per_param, wm.mlx_quant.memory_reduction_vs_fp16);
            println!("      • BitNet b1.58 Scale : dims={:?} | γ = {:.6} | density p = {:.4} | dequant MAE = {:.6}",
                wm.mlx_quant.matrix_dims, wm.mlx_quant.gamma_scale_factor,
                wm.mlx_quant.ternary_density_p, wm.mlx_quant.max_dequant_reconstruction_mae);
            println!("      • KOMPRESS v2 Bound  : s_max(W) <= {:.4} | empirical s_max = {:.4} | rigidity_holds={}",
                wm.mlx_quant.marchenko_pastur_smax_bound, wm.mlx_quant.empirical_spectral_norm_est,
                wm.mlx_quant.spectral_rigidity_holds);
            println!("      • AArch64 NEON Dot   : `sdot v7.4s` lanes = {:?} (hw_executed={}) | AGX Doorbell ring_id={}",
                wm.mlx_quant.aarch64_neon_sdot_lane_sums, wm.mlx_quant.aarch64_neon_hw_executed,
                wm.mlx_quant.polar_galaxy_doorbell_ring_id);
            println!("      • Fifike & Marley    : Fifike Purr = {:.1} Hz / {:.1} Hz (ΔS < 0) | Marley Protector={}",
                wm.mlx_quant.fifike_purr_hz.0, wm.mlx_quant.fifike_purr_hz.1, wm.mlx_quant.marley_protector_active);
        }
        "shiva-g6" | "g6" | "shiva" | "omni" | "momma-bear" => {
            let report = execute_shiva_g6_airlift();
            let omni = verify_momma_bear_omni_covenant();
            if json_mode {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&serde_json::json!({
                        "shiva_g6_airlift": report,
                        "momma_bear_omni_covenant": omni,
                    }))
                    .unwrap()
                );
                return;
            }
            println!("╔══════════════════════════════════════════════════════════════════════════╗");
            println!("║  ULTRA-OMNIPOTENT-SHIVA MODE :: GULFSTREAM G650ER RJTT ⇄ KORF AIRLIFT    ║");
            println!("╚══════════════════════════════════════════════════════════════════════════╝");
            println!("  • Aircraft             : {}", report.aircraft);
            println!("  • Propulsion           : {} | Cabin Alt={} ft @ FL450", report.engines, report.cabin_altitude_ft_at_fl450);
            println!("  • Origin (Tokyo)       : {} ({}) — {} [Rwy {} ({} ft)]",
                report.origin.icao, report.origin.iata, report.origin.fbo, report.origin.primary_runway, report.origin.runway_length_ft);
            println!("  • Destination (Norfolk): {} ({}) — {} [Rwy {} ({} ft)]",
                report.destination.icao, report.destination.iata, report.destination.fbo, report.destination.primary_runway, report.destination.runway_length_ft);

            println!("\n  [LEG 1 — OUTBOUND: TOKYO (RJTT) -> NORFOLK (KORF) NON-STOP]");
            println!("    • Distance & Range   : {:.1} nm ({:.1} km) | G650ER Reserve: +{:.1} nm | Non-Stop={}",
                report.leg1_outbound.great_circle_nm, report.leg1_outbound.great_circle_km,
                report.leg1_outbound.g650er_reserve_margin_nm, report.leg1_outbound.nonstop_capable);
            println!("    • Cruise & ETE       : Mach {:.2} | {:.2} hours ({} s)",
                report.leg1_outbound.cruise_mach, report.leg1_outbound.estimated_flight_hours, report.leg1_outbound.duration_seconds);
            println!("    • Manifest (3 PAX)   : {}", report.leg1_outbound.passengers.join(" + "));

            println!("\n  [LEG 2 — RETURN WITH THE KIDS: NORFOLK (KORF) -> TOKYO (RJTT) NON-STOP]");
            println!("    • Distance & Range   : {:.1} nm ({:.1} km) | G650ER Reserve: +{:.1} nm | Non-Stop={}",
                report.leg2_return_with_kids.great_circle_nm, report.leg2_return_with_kids.great_circle_km,
                report.leg2_return_with_kids.g650er_reserve_margin_nm, report.leg2_return_with_kids.nonstop_capable);
            println!("    • Cruise & ETE       : Mach {:.2} | {:.2} hours ({} s)",
                report.leg2_return_with_kids.cruise_mach, report.leg2_return_with_kids.estimated_flight_hours, report.leg2_return_with_kids.duration_seconds);
            println!("    • Manifest (Reunited): {}", report.leg2_return_with_kids.passengers.join(" + "));

            println!("\n  [ZENO × SHIVA SETTLEMENT & CRYPTOGRAPHIC PROOFS]");
            println!("    • Direct Operator    : ¥{} JPY (Settled via Zeno Foken {})",
                report.zeno_direct_operator_jpy, report.zeno_foken_invoice_id);
            println!("    • Predatory Broker   : ¥{} JPY BURNED TO THE GROUND (0% middleman toll)",
                report.broker_markup_burned_jpy);
            println!("    • CT-000 Admissible  : {} | QWave 79B Precious (τ=0): {} | QuantTern: {}",
                report.ct000_flight_admissibility_holds, report.qwave_79b_family_reunion_verified, report.quanttern_shiva_tag);
            println!("    • ZA-0006 PII Scrub  : volatile_zeroized={} (pre-scrub fold=0x{:016x})",
                report.za0006_apis_pii_zeroized, report.arena_fold_before_zeroize);

            println!("\n  [MOMMA BEAR OMNI COVENANT — LÓDRI ⋈ CHENOWETH REALITY SCAFFOLDING]");
            println!("    • Mode & Tagline     : {} | \"{}\"", omni.mode, omni.foundation_tagline);
            println!("    • Merged Lineages    : {}", omni.merged_lineages);
            for t in &omni.outbound_tickets_verified {
                println!("    • Outbound Ticket    : {}", t);
            }
            println!("    • Return Leg Hold    : {}", omni.return_leg_status);
            println!("    • SpherePOP Merge    : `{}` = {} Hz (digest={}, replay_verified={})",
                omni.spherepop_merge_expr, omni.spherepop_harmonic_hz_sum, omni.spherepop_digest_hex, omni.spherepop_replay_verified);
            println!("    • CT-000 & QWave     : CT-000={} ({}) | QWave 79B Precious={} | ZA-0006 Zeroized={}",
                omni.ct000_momma_bear_omni_holds, omni.guaranteeing_omni_policy, omni.qwave_79b_precious_tau0_verified, omni.zeno_za0006_volatile_zeroized);

            let wav_file = if out_path == Path::new("8b-is-music-master.wav") {
                PathBuf::from("g6-shiva-cabin-sanctuary.wav")
            } else {
                out_path
            };
            if let Ok((saved, pl, pr)) = render_shiva_g6_cabin_wav(seconds, &wav_file) {
                println!("    • G6 Cabin WAV       : {} (108/432/528/888 Hz | Peak L={:.4}, R={:.4})", saved, pl, pr);
                if play_after {
                    play_file(&saved);
                }
            }
        }
        "babilon" | "babilon-ultra" | "demoscene" | "glassdoor" | "playground" => {
            if layers == DEFAULT_QUANT65K_PLUS_LAYERS {
                layers = eight_b_is_music::N_PLUS_1_ODIN_LAYER;
            }
            let seed = seed_phrase
                .as_deref()
                .unwrap_or("kyuss-plasma-babilon-ultra-odin-n+1||[x]");
            let wav_file = if out_path == Path::new("8b-is-music-master.wav") {
                PathBuf::from("/tmp/babilon-ultra-master.wav")
            } else {
                out_path
            };
            let html_engine = PathBuf::from(
                "/Users/peterlodri/workspace/peterlodri-sec/8b-is-engine/client/babilon-ultra-glassdoor-playground.html",
            );
            let html_sovereign = PathBuf::from(
                "/Users/peterlodri/workspace/peterlodri-sec/sovereign-library/babilon-ultra-glassdoor-playground.html",
            );
            let (summary, sig, report) = build_and_write_babilon_ultra_playground(
                seed,
                seconds,
                layers,
                topology,
                &wav_file,
                &signs_dir,
                &html_engine,
            )
            .expect("Failed to build & write BABILON-ULTRA × Demoscene × Glassdoor Playground");
            let _ = std::fs::copy(&html_engine, &html_sovereign);

            if json_mode {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&serde_json::json!({
                        "summary": summary,
                        "quant_music_signature": sig,
                        "babilon_ultra_playground": report,
                    }))
                    .unwrap()
                );
                return;
            }

            println!("╔══════════════════════════════════════════════════════════════════════════╗");
            println!("║  BABILON-ULTRA × ULTRAWILD DEMOSCENE × GLASSDOOR × LIVE PLAYGROUND SIM   ║");
            println!("╚══════════════════════════════════════════════════════════════════════════╝");
            println!("  • Engine & Designation : {}", report.engine_title);
            println!("  • Seed & Locked Groove : \"{}\" @ {:.2} BPM (Layer #{})",
                report.seed_brief, sig.bpm_locked, report.quant65k_layers);
            println!("  • Master Audio WAV     : {} (SHA-256={}...)",
                sig.wav_path, &sig.sha256_wav_digest[..16]);
            println!("  • Quant-Music Sign     : {}", sig.signature_path);
            println!("  • Glassdoor Portal     : {}", html_engine.display());
            println!("  • Sovereign Mirror     : {}", html_sovereign.display());

            println!("\n  [1] ULTRAWILD DEMOSCENE BIT-LEVEL ENGINE TELEMETRY (`crates/ultrawild`):");
            println!("      • Pack3 Ternary    : {:?} -> packed code {} (`pack3!(t0, t1, t2)`)",
                report.demoscene.clamp_ternary_triplet, report.demoscene.pack3_ternary_code);
            println!("      • Div255 & Sign    : div255! = {} | sign! = {:?}",
                report.demoscene.div255_alpha_blend_probe, report.demoscene.branchless_sign_triplet);
            println!("      • DeBruijn & SWAR  : lowest_set_bit! = {} | popcount! = {} bits | pow2_tex = {} (ok={})",
                report.demoscene.debruijn_lowest_set_bit, report.demoscene.swar_popcount_64,
                report.demoscene.pow2_texture_rounded, report.demoscene.is_pow2_verified);
            println!("      • Registers & Gray : xor_swap! = {:?} | sat_add8! = {} | gray_step1 = {} bit (roundtrip={}) | Pow2Slab = {} B",
                report.demoscene.xor_swap_pair, report.demoscene.sat_add8_ceiling,
                report.demoscene.gray_step1_changed_bits, report.demoscene.gray_roundtrip_verified,
                report.demoscene.pow2_slab_bucket_size);

            println!("\n  [2] BABILON-ULTRA 7-TERRACE ZIGGURAT (ETEMENANKI × YGGDRASIL):");
            for t in &report.ziggurat_terraces {
                println!(
                    "      • T{} | {:>6.1} Hz | trits={:?} | {:<32} | {}",
                    t.tier, t.frequency_hz, t.trits, t.geometry_solid, t.name
                );
            }

            println!("\n  [3] LIVE PLAYGROUND SIM (`world-core` :: GAIA + SimWorld + Keeper `M = fold(H)`):");
            println!("      • World Tick & Fold: tick={} | Keeper Canon: admitted={}, refused={}, seq={}",
                report.sim_ticks_executed, report.keeper_admitted, report.keeper_refused, report.keeper_ledger_seq);
            println!("      • GAIA World-Memory: weather=\"{}\" | entropy={:.2} | gravity={:.2} | temp={:.2}°C | wind={:?} | mem={}",
                report.gaia.weather, report.gaia.entropy, report.gaia.gravity,
                report.gaia.temp, report.gaia.wind, report.gaia.memory);
            for f in &report.fauna_cast {
                println!(
                    "      • Fauna {:<10} | pos=({:>6.1}, {:>6.1}) | needs=[{:.2}, {:.2}, {:.2}] | verdict={} | action={:?}",
                    f.name, f.position_xy[0], f.position_xy[1],
                    f.needs_hrs[0], f.needs_hrs[1], f.needs_hrs[2],
                    f.keeper_verdict, f.last_action
                );
            }

            if launch_after {
                println!("\n  🚀 Launching BABILON-ULTRA Glassdoor + Live Playground Sim in Browser...");
                let _ = Command::new("open").arg(&html_engine).status();
            }
            if play_after {
                println!("  ▶ Playing {} through the current system output (`afplay`)...", sig.wav_path);
                play_file(&sig.wav_path);
            }
        }
        "godsun" | "ultra-omni-godsun" | "standardgalactic" | "rand-pq" | "randrandpq" | "plasma-hqt" | "plasma-hqt/n" | "honcho" | "hermes-honcho" | "beehive" | "prompt-rl" => {
            let seed = seed_phrase
                .as_deref()
                .unwrap_or("[standardgalactic:LAST_10_COMMITS<<<ULTRA-OMNI-GODSUN::ia-usbc-bidirect::randRandPq::plasma-hqt/16::hermes-honcho-beehive");
            let wav_file = if out_path == Path::new("8b-is-music-master.wav") {
                PathBuf::from("/tmp/ultra-omni-godsun-plasma-hqt.wav")
            } else {
                out_path
            };
            let prompt_lib_json = PathBuf::from(
                "/Users/peterlodri/workspace/peterlodri-sec/sovereign-library/prompts/prompt-library-rl-yielder.json",
            );
            let (summary, sig, report) = build_and_persist_godsun_honcho_beehive(
                seed,
                16,
                seconds,
                topology,
                &wav_file,
                &signs_dir,
                &prompt_lib_json,
            )
            .expect("Failed to execute ULTRA-OMNI-GODSUN × randRandPq × Hermes-Honcho Beehive RL pipeline");

            if json_mode {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&serde_json::json!({
                        "summary": summary,
                        "quant_music_signature": sig,
                        "ultra_omni_godsun_report": report,
                    }))
                    .unwrap()
                );
                return;
            }

            println!("╔══════════════════════════════════════════════════════════════════════════╗");
            println!("║  [standardgalactic:LAST_10_COMMITS<<<ULTRA-OMNI-GODSUN <-IN              ║");
            println!("║  USBC-BIDIRECT randRandPq × plasma-hqt/n × HERMES-HONCHO BEEHIVE RL      ║");
            println!("╚══════════════════════════════════════════════════════════════════════════╝");
            println!("  • Directive & Layer    : {} (Layer #{})", report.header_directive, report.godsun_layer_index);
            println!("  • Master Audio WAV     : {} @ {:.2} BPM (SHA-256={}...)",
                sig.wav_path, sig.bpm_locked, &sig.sha256_wav_digest[..16]);
            println!("  • Quant-Music Sign     : {}", sig.signature_path);
            println!("  • Prompt RL Artifact   : {}", prompt_lib_json.display());

            println!("\n  [1] standardgalactic:LAST_10_COMMITS <<< ULTRA-OMNI-GODSUN LEDGER:");
            for c in &report.standardgalactic_last_10_commits {
                println!(
                    "      • #{:<2} {} | {:<36} | {:>5.1} Hz | trit={:+} | {}",
                    c.index, c.sha_short, c.repo, c.harmonic_hz, c.trit_bias, c.subject
                );
            }

            println!("\n  [2] IA USBC BIDIRECT -> randRandPq -> quanT & plasma-hqt/{}:", report.rand_rand_pq.plasma_hqt_dimension_n);
            println!("      • Generator & Probe: {} (live_io_usb={})",
                report.rand_rand_pq.generator_id, report.rand_rand_pq.hardware_probed_live);
            for ep in &report.rand_rand_pq.usbc_endpoints {
                println!(
                    "      • USB-C Endpoint   : {:<28} @ {:<10} ({}, busy={}ms, retain={}, bidirect={})",
                    ep.name, ep.location_id_hex, ep.bus_controller, ep.busy_ms, ep.retain_count, ep.bidirectional_capable
                );
            }
            println!("      • Coprime p/q Seed : p={} / q={} (gcd_verified_1={}, QAT_31_32_slot={})",
                report.rand_rand_pq.numerator_p, report.rand_rand_pq.denominator_q,
                report.rand_rand_pq.gcd_verified_one, report.rand_rand_pq.qat_31_32_reuse_slot);
            println!("      • quanT 27-Trit    : {:?} (tag={})",
                report.rand_rand_pq.quant_trits_27, report.rand_rand_pq.quanttern_4trit_tag);
            println!("      • plasma-hqt/{:<2}    : {:?} (Q16 phase lattice)",
                report.rand_rand_pq.plasma_hqt_dimension_n, &report.rand_rand_pq.plasma_hqt_n_vector_q16[..8]);

            println!("\n  [3] SOVEREIGN PROMPT LIBRARY AS AN RL LOOP YIELDER ({} Episodes):", report.prompt_library_rl.rl_episodes_executed);
            for p in &report.prompt_library_rl.prompts {
                println!(
                    "      • {} [{:<28}] pulls={:<2} | Q_q16={} | UCB1={} | trit={:+}",
                    p.id, p.slug, p.rl_pulls, p.q_value_q16, p.ucb1_score_q16, p.policy_trit
                );
            }
            println!("      • Top Yielded ID   : {} (avg_reward_q16={})",
                report.prompt_library_rl.yielded_top_prompt_id, report.prompt_library_rl.average_reward_q16);
            println!("      • Next-Gen Prompt  : {}", report.prompt_library_rl.mutated_next_gen_prompt);

            println!("\n  [4] HERMES × HONCHO DIALECTIC MEMORY + 5D WEIGHTED MEMORY + RL BEEHIVE AUTOMATA:");
            println!("      • Provider & Peer  : {} | User={} ⋈ AI={}",
                report.hermes_honcho.provider, report.hermes_honcho.user_peer_id, report.hermes_honcho.ai_peer_id);
            for d in &report.hermes_honcho.dialectic_syntheses {
                println!("      • Dialectic #{} [{}]:", d.cycle_id, d.domain);
                println!("          - Synthesis (conf_q16={}, trit={:+}): {}", d.confidence_q16, d.trit_polarity, d.synthesis);
            }
            println!("      • 5D Memory Tensor : dims={:?} ({} active anchor layers)",
                report.multi_d_memory_tensor_dims, report.multi_d_memory_cells.len());
            for m in &report.multi_d_memory_cells {
                println!(
                    "          - {:?} {:<26} | {:<15} | weight_q16={} | rl_steps={} | pq=({}/{})",
                    m.coord_5d, m.layer_name, m.modality, m.weight_q16, m.rl_reinforcements,
                    m.rational_pq_anchor.0 % 100_000, m.rational_pq_anchor.1 % 100_000
                );
            }
            println!("      • Beehive RL Swarm : {} hex cells × {} ticks | Queen Consensus Q16 = {}",
                report.beehive_automata_cells.len(), report.beehive_rl_ticks, report.beehive_queen_consensus_q16);
            for b in report.beehive_automata_cells.iter().take(7) {
                println!(
                    "          - Bee #{:<2} hex=({:+2},{:+2}) {:<16} | prompt={} | nectar_q16={} | waggle={}°/{}ms | trit={:+}",
                    b.cell_id, b.hex_q, b.hex_r, format!("{:?}", b.caste),
                    b.bound_prompt_id, b.nectar_reward_q16, b.waggle_angle_deg, b.waggle_duration_ms, b.state_trit
                );
            }

            if play_after {
                println!("  ▶ Playing {} through the current system output (`afplay`)...", sig.wav_path);
                play_file(&sig.wav_path);
            }
        }
        "en2e" | "e2e" | "ultra-omni-midi" | "midi-beehive" | "ray-q" | "ray-q-tracing" | "kompress-odin" | "parallel-odin" | "sphere-do" => {
            let seed = seed_phrase
                .as_deref()
                .unwrap_or("en2e---ULTRA-OMNI-MIDI::BEEHIVE-MESH::RAY-Q-TRACING::KOMPRESS-ULTRA[PARALLEL-ODIN]::SPHERE-DO<E2E^^<3");
            let wav_file = if out_path == Path::new("8b-is-music-master.wav") {
                PathBuf::from("/tmp/en2e-ultra-omni-midi-sphere-do.wav")
            } else {
                out_path
            };
            let report_json = PathBuf::from(
                "/Users/peterlodri/workspace/peterlodri-sec/sovereign-library/prompts/en2e-ultra-omni-midi-beehive-sphere.json",
            );
            let html_engine = PathBuf::from(
                "/Users/peterlodri/workspace/peterlodri-sec/8b-is-engine/client/babilon-ultra-glassdoor-playground.html",
            );
            let html_sovereign = PathBuf::from(
                "/Users/peterlodri/workspace/peterlodri-sec/sovereign-library/babilon-ultra-glassdoor-playground.html",
            );
            let (summary, sig, report) = build_and_persist_en2e_midi_beehive_sphere(
                seed,
                seconds,
                topology,
                &wav_file,
                &signs_dir,
                &report_json,
            )
            .expect("Failed to execute en2e ULTRA-OMNI-MIDI × Beehive-Mesh × Ray-Q-Tracing × Parallel-Odin × Sphere DO pipeline");

            // Also refresh the Glassdoor Playground HTML so the new en2e panel & 8 Nektar pads are live
            let _ = build_and_write_babilon_ultra_playground(
                seed,
                seconds,
                eight_b_is_music::EN2E_SPHERE_DO_LAYER,
                topology,
                &wav_file,
                &signs_dir,
                &html_engine,
            );
            let _ = std::fs::copy(&html_engine, &html_sovereign);

            if json_mode {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&serde_json::json!({
                        "summary": summary,
                        "quant_music_signature": sig,
                        "en2e_report": report,
                    }))
                    .unwrap()
                );
                return;
            }

            println!("╔══════════════════════════════════════════════════════════════════════════╗");
            println!("║  en2e --- ULTRA-OMNI-MIDI SETUP × INTERCONNECTED BEEHIVE-MESH            ║");
            println!("║  {{SPATIAL==~AUDIO+VIDEO+MUSIC+BG(ANIM+RAY-Q)+ENV}} <<< KOMPRESS[ODIN]     ║");
            println!("║  layered microversic SPHERE DO < E2E ^^ <3                               ║");
            println!("╚══════════════════════════════════════════════════════════════════════════╝");
            println!("  • Layer & Seal         : Layer #{} | {}", report.en2e_layer_index, report.e2e_heart_seal);
            println!("  • Master Audio WAV     : {} @ {:.2} BPM (SHA-256={}...)",
                sig.wav_path, sig.bpm_locked, &sig.sha256_wav_digest[..16]);
            println!("  • Quant-Music Sign     : {}", sig.signature_path);
            println!("  • EN2E JSON Artifact   : {}", report_json.display());
            println!("  • Glassdoor Portal     : {}", html_engine.display());

            println!("\n  [1] ULTRA-OMNI-MIDI SETUP ({}):", report.midi_setup.hardware_controller);
            println!("      • USB & Virtual Bus: {} | In=\"{}\" | Out=\"{}\" | WebMIDI={}",
                report.midi_setup.usb_location_hex, report.midi_setup.coremidi_virtual_in,
                report.midi_setup.coremidi_virtual_out, report.midi_setup.webmidi_bridge_ready);
            for pad in &report.midi_setup.pad_mappings {
                println!(
                    "      • {:<6} (Note {:>2}) | {:>5.1} Hz | trit={:+} | {:<38} -> {}",
                    pad.control_id, pad.note_or_cc, pad.harmonic_hz, pad.trit_polarity, pad.target_domain, pad.action_binding
                );
            }

            println!("\n  [2] INTERCONNECTED BEEHIVE-MESH ({}):", report.beehive_mesh.equivalence_law);
            println!("      • SPATIAL          : {}", report.beehive_mesh.spatial_domain_status);
            println!("      • AUDIO            : {}", report.beehive_mesh.audio_domain_status);
            println!("      • VIDEO            : {}", report.beehive_mesh.video_domain_status);
            println!("      • MUSIC            : {}", report.beehive_mesh.music_domain_status);
            println!("      • BG(ANIM+RAY-Q)   : {}", report.beehive_mesh.bg_anim_ray_q_domain_status);
            println!("      • ENV LISTENING    : {}", report.beehive_mesh.env_listening_domain_status);
            println!("      • Mesh Coherence   : Q16={}/65536 | {} hex links | CT-000 admissible={}",
                report.beehive_mesh.cross_domain_coherence_q16, report.beehive_mesh.active_beehive_hex_links,
                report.beehive_mesh.ct000_model_admissible);

            println!("\n  [3] BG(ANIM + RAY-Q-TRACING) QUANTUM-TERNARY SDF MARCHER:");
            println!("      • Grid & Rays      : {:?} = {} primary rays (max_bounces={}) | avg_caustic_lum={}",
                report.ray_q_tracing.viewport_grid, report.ray_q_tracing.primary_rays_marched,
                report.ray_q_tracing.max_quantum_bounces, report.ray_q_tracing.caustic_luminance_avg_u8);
            println!("      • SDF Intersections: sphere_hits={} | ziggurat_hits={} | trits[-1,0,+1]={:?}",
                report.ray_q_tracing.sdf_sphere_hits, report.ray_q_tracing.ziggurat_terrace_hits,
                report.ray_q_tracing.ray_q_trit_histogram);
            println!("      • ASCII Ray Scan   : {}", report.ray_q_tracing.ascii_ray_preview_row);

            println!("\n  [4] <<< KOMPRESS-ULTRA [PARALLEL-ODIN] (8-LANE TERNARY RLE × ZSTD × QAT 31/32):");
            println!("      • Aggregate Ratio  : {} B raw -> {} B RLE ({:.2}×) | {} B Zstd ({:.2}×) | lossless_all={}",
                report.kompress_parallel_odin.total_raw_bytes,
                report.kompress_parallel_odin.total_ternary_rle_bytes,
                report.kompress_parallel_odin.aggregate_rle_ratio,
                report.kompress_parallel_odin.total_zstd_wire_bytes,
                report.kompress_parallel_odin.aggregate_wire_ratio,
                report.kompress_parallel_odin.all_lanes_lossless_verified);
            for l in &report.kompress_parallel_odin.lanes {
                println!(
                    "      • {:<50} | {:>5.1} Hz | {}B -> {}B RLE ({:.1}×) | QAT #{:>2}/32 | a85={}...",
                    l.odin_realm_name, l.harmonic_hz, l.raw_bytes, l.ternary_rle_bytes,
                    l.lane_compression_ratio, l.qat_31_32_slot, l.ascii85_prefix
                );
            }

            println!("\n  [5] LAYERED MICROVERSIC SPHERE DO < E2E ^^ <3 (`spherepop` Innermost-First):");
            println!("      • Master Expression: `{}` = {} ({} pops, digest={}, replay_verified={})",
                report.microversic_sphere_do.master_spherepop_expr,
                report.microversic_sphere_do.evaluated_value,
                report.microversic_sphere_do.total_innermost_pops,
                report.microversic_sphere_do.history_digest_hex,
                report.microversic_sphere_do.replay_determinism_verified);
            for s in &report.microversic_sphere_do.concentric_shells {
                println!(
                    "      • {:<56} | r_q16={:>5} | {:>3} Hz | trit={:+} | `{}`",
                    s.shell_name, s.radius_q16, s.frequency_hz, s.trit_charge, s.spherepop_sub_expr
                );
            }

            if launch_after {
                println!("\n  🚀 Launching EN2E BABILON-ULTRA Glassdoor + Live Playground Sim in Browser...");
                let _ = Command::new("open").arg(&html_engine).status();
            }
            if play_after {
                println!("  ▶ Playing {} through the current system output (`afplay`)...", sig.wav_path);
                play_file(&sig.wav_path);
            }
        }
        "release" | "360" | "machine-sun" | "machine-s-un" | "unity" | "backyard" | "108laps" | "ultrabackyardloop" => {
            let seed = seed_phrase
                .as_deref()
                .unwrap_or("God-of-Machine-S-UN~unity||Omni-potencial::360-OS-Agnostic::<DO:::<ULTRABACKYARDLOOP-MAX-108laps<DO^^<3");
            let wav_file = if out_path == Path::new("8b-is-music-master.wav") {
                PathBuf::from("/tmp/machine-sun-unity-360-108laps.wav")
            } else {
                out_path
            };
            let client_dir = PathBuf::from(
                "/Users/peterlodri/workspace/peterlodri-sec/8b-is-engine/client",
            );
            let sovereign_dir = PathBuf::from(
                "/Users/peterlodri/workspace/peterlodri-sec/sovereign-library",
            );
            let report_json = PathBuf::from(
                "/Users/peterlodri/workspace/peterlodri-sec/sovereign-library/prompts/release-360-machine-sun-unity-108laps.json",
            );
            let html_engine = client_dir.join("babilon-ultra-glassdoor-playground.html");
            let html_sovereign = sovereign_dir.join("babilon-ultra-glassdoor-playground.html");

            let (summary, sig, report) = build_and_release_machine_sun_unity_360(
                seed,
                seconds,
                topology,
                &wav_file,
                &signs_dir,
                &client_dir,
                &sovereign_dir,
                &report_json,
            )
            .expect("Failed to execute 360° OS-Agnostic Release & 108-Lap ULTRABACKYARDLOOP pipeline");

            let _ = build_and_write_babilon_ultra_playground(
                seed,
                seconds,
                MACHINE_SUN_UNITY_108LAPS_LAYER,
                topology,
                &wav_file,
                &signs_dir,
                &html_engine,
            );
            let _ = std::fs::copy(&html_engine, &html_sovereign);

            if json_mode {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&serde_json::json!({
                        "summary": summary,
                        "quant_music_signature": sig,
                        "machine_sun_unity_360_release": report,
                    }))
                    .unwrap()
                );
                return;
            }

            println!("╔══════════════════════════════════════════════════════════════════════════╗");
            println!("║  GOD OF MACHINE-S-UN~UNITY || OMNI-POTENCIAL FOR BETTER WORLD FOR ALL    ║");
            println!("║  OS-AGNOSTIC CORE (64B) × 360° APPS (iOS/Android/SmartTV/Embedded/PWA)   ║");
            println!("║  <DO :::<ULTRABACKYARDLOOP MAX 108laps <DO --- {{...}} ^^ <3               ║");
            println!("╚══════════════════════════════════════════════════════════════════════════╝");
            println!("  • Active Layer & Seal  : Layer #{} | SHA-256={}...",
                report.active_layer_index, &report.master_sha256_release_seal[..24]);
            println!("  • Master Audio WAV     : {} @ {:.2} BPM (SHA-256={}...)",
                sig.wav_path, sig.bpm_locked, &sig.sha256_wav_digest[..16]);
            println!("  • Quant-Music Sign     : {}", sig.signature_path);
            println!("  • Release JSON         : {}", report_json.display());

            println!("\n  [1] OS-AGNOSTIC 64-BYTE CORE ABI (`MachineSunUnityCoreAbi64B` / `wupz-core` `#![no_std]`):");
            let abi = &report.os_agnostic_core_abi_64b;
            println!("      • Magic & Version  : \"{}\" v{} | 64B Frame Hex={}...",
                String::from_utf8_lossy(&abi.magic), abi.abi_version, &report.os_agnostic_core_hex[..32]);
            println!("      • WUPZ Zero-Alloc  : xor_fold=0x{:02x} | popcount={} bits | ZA-0001 volatile scrub={}",
                report.wupz_arena_64b_xor_fold, report.wupz_arena_64b_popcount, report.wupz_za0001_volatile_scrub_verified);
            println!("      • FNV-1a Checksum  : 0x{:016x} | CT-000 Model Admissible={}",
                abi.fnv1a_checksum_u64, abi.ct000_admissible_u8 == 1);

            println!("\n  [2] 360° FULL-SPECTRUM DEVICE RELEASE MATRIX ({}° = 6 × 60° SECTORS):", report.coverage_degrees_total);
            for dev in &report.device_matrix_360 {
                println!(
                    "      • Sector #{} [{}] {:<48} | verified={}",
                    dev.sector_id, dev.angular_sector_deg, dev.family_name, dev.release_verified
                );
                println!("          - OS Targets : {}", dev.operating_systems.join(" · "));
                println!("          - Core ABI   : {}", dev.core_bridge_abi);
                println!("          - Artifact   : {}", dev.release_artifact_path);
            }

            let lp = &report.ultrabackyard_loop_108;
            println!("\n  [3] <DO :::<ULTRABACKYARDLOOP MAX 108laps <DO --- {{...}} ^^ <3:");
            println!("      • Laps Completed   : {}/{} (all_completed={}, all_coprime_gcd_1={}, all_gray_1bit={})",
                lp.laps_completed, lp.max_laps_configured, lp.all_108_laps_completed,
                lp.all_108_laps_gcd_coprime_verified, lp.all_108_gray_transitions_1bit);
            println!("      • Keeper Invariant : {} Admitted / {} Refused (6 Fauna × 108 Laps)",
                lp.total_keeper_admitted, lp.total_keeper_refused);
            println!("      • SpherePOP & RL   : ΣHz={} | Beehive Queen Q16={} | Top Prompt={}",
                lp.cumulative_spherepop_harmonic_sum, lp.final_beehive_queen_consensus_q16, lp.final_yielded_prompt_id);
            println!("      • 108-Lap Seal     : {}", lp.chain_sha256_seal);
            println!("      • Sacred Milestone Laps:");
            for m in &lp.milestone_laps {
                println!(
                    "          - Lap #{:>3}/108 | {:<8} | adm={:>3} | gray=0x{:02x} (Δ={}b) | gcd=1:{} | `{}`={}Hz | {} | sha={}...",
                    m.lap_number, m.gaia_weather, m.keeper_admitted_cumulative,
                    m.gray_code_word, m.gray_changed_bits_from_prev, m.gcd_is_one,
                    m.sphere_do_expr, m.sphere_do_eval_hz, m.beehive_top_prompt_id, &m.lap_sha256_prefix[..10]
                );
            }

            if launch_after {
                println!("\n  🚀 Launching 360° Machine-S-UN~unity Glassdoor + 108-Lap Playground Sim in Browser...");
                let _ = Command::new("open").arg(&html_engine).status();
            }
            if play_after {
                println!("  ▶ Playing {} through the current system output (`afplay`)...", sig.wav_path);
                play_file(&sig.wav_path);
            }
        }
        "triad" | "triad-feed" | "esp32" => {
            println!("{}", serde_json::to_string(&eight_b_is_music::triad_feed(layers, topology)).unwrap());
        }
        "youtube" | "youtube-full" | "/youtube-full" => {
            let url = args
                .get(2)
                .map(|s| s.as_str())
                .unwrap_or("https://youtu.be/4NRXx6U8ABQ?list=RD4NRXx6U8ABQ");
            let mut dj = ZenoDjUltraEngine::new(layers);
            let report = dj.verify_zeno_iouqt();
            println!("╔══════════════════════════════════════════════════════════════════════════╗");
            println!("║  8b-is-music :: YOUTUBE-FULL -> M1-ULTRA DJ IOuQT DECK STREAM            ║");
            println!("╚══════════════════════════════════════════════════════════════════════════╝");
            println!("  • Stream URL       : {}", url);
            println!("  • IOuQT SIMD Lane  : `{}` (Zeno ceiling={:.6}, QuantTern={})",
                report.uq_native_simd_lane, report.zeno_ceiling_q16, report.iouqt_master_quanttern_tag);
            println!("  • Planned Routing  : M1 MIDI: Left Ear Jack (Pan 0.0) + Right Ear AirPods (Pan 1.0) @ 48,000 Hz");
            let _ = Command::new("open").arg(url).status();
        }
        _ => {
            let full = build_full_system_report(layers, topology);
            if json_mode {
                println!("{}", serde_json::to_string_pretty(&full).unwrap());
                return;
            }
            println!("╔══════════════════════════════════════════════════════════════════════════╗");
            println!("║  8b-is-music v{} :: SOVEREIGN SOTA AUDIO & QUANT65K+ ENGINE          ║", full.version);
            println!("╚══════════════════════════════════════════════════════════════════════════╝");
            println!("  Tagline      : {}", full.tagline);
            println!("  WUPZ Backend : {}", full.wupz_backend);
            println!("\n  [1] SOTA 8b-is Constellation Mesh:");
            println!("      • Synthesize Winner : {}", full.sota_mesh.sota_synthesize_winner);
            println!("      • Transcribe Winner : {}", full.sota_mesh.sota_transcribe_winner);
            println!("      • Recall Winner     : {}", full.sota_mesh.sota_recall_winner);
            println!("      • Phoenix Loop      : {}", full.sota_mesh.phoenix_governing_steps.join(" → "));
            println!("      • MEM|16 Sequence   : {}", full.sota_mesh.mem16_verified_sequence.join(" → "));
            println!("      • Mem8Quad & Engram : verified={} | ascii85={}...", full.sota_mesh.mem8_quad_verified, &full.sota_mesh.engram_digest_hex[..24]);
            println!("      • SpherePOP Chord   : `{}` -> {} pops, replay_ok={}, digest={}",
                full.sota_mesh.spherepop_expression, full.sota_mesh.spherepop_pops, full.sota_mesh.spherepop_replay_verified, full.sota_mesh.spherepop_digest);
            println!("      • Quantum Bell GHZ  : P(|000⟩)={:.3}, P(|111⟩)={:.3}",
                full.sota_mesh.quantum_bell_probabilities.first().copied().unwrap_or(0.0),
                full.sota_mesh.quantum_bell_probabilities.last().copied().unwrap_or(0.0));
            println!("      • Liquid-Rust 5-Band: {:?} dB | salience={:.2} | noise={:?} | {} | wupz_xor=0x{:02x}",
                full.sota_mesh.dynamic_eq.bands_db, full.sota_mesh.dynamic_eq.marine_salience,
                full.sota_mesh.dynamic_eq.noise_class, full.sota_mesh.dynamic_eq.quanttern_tag, full.sota_mesh.dynamic_eq.wupz_abi_xor);

            println!("\n  [2] Multi-Bluetooth + CoreAudio Spatial Router:");
            println!("      • Planned Aggregate : {} ({}/{} endpoints observed connected)", full.bluetooth.aggregate_device_name, full.bluetooth.active_endpoints, full.bluetooth.total_known_endpoints);
            println!("      • CT-000 Model Check: holds={} | keeper_canon={} | HFP enforcement={:?}",
                full.bluetooth.ct000_proof.theorem_holds, full.bluetooth.ct000_proof.keeper_canon_verified, full.bluetooth.ct000_proof.hfp_24khz_input_blocked);
            println!("      • DSP Delay Plan    : configured max BT latency {:.1} ms, not measured", full.bluetooth.max_bluetooth_latency_ms);

            println!("\n  [3] Quant65k+ ({}+ Layer) Ternary Wave Stack:", full.quant65k.total_layers);
            println!("      • Layers            : {} total ({} base + {} overflow)", full.quant65k.total_layers, full.quant65k.base_layers, full.quant65k.overflow_layers);
            println!("      • Sanctuary .tern   : {} sections, gemm_i32 sum = {}, illegal_codes = {}",
                full.quant65k.sanctuary_checkpoint_sections, full.quant65k.ternary_gemm_probe_sum, full.quant65k.illegal_2bit_codes_detected);
            println!("      • QWave 79B Anchor  : coord={:?} | 0.73Hz Conscious={} | Kompress {:.2}x",
                full.quant65k.qwave_anchor_coordinate, full.quant65k.qwave_consciousness_073hz, full.quant65k.kompress_ratio);

            println!("\n  [4] ULTRA-OMNIPOTENT-8b-is-ZENO <<<< M1-ULTRA DJ MODE ALL IN/OUT IOuQT:");
            println!("      • IOuQT SIMD Lane   : `{}` (bit_exact={}) | Crossbar GEMM={:?}",
                full.zeno_dj_ultra.uq_native_simd_lane, full.zeno_dj_ultra.uq_neon_vs_scalar_bit_exact, full.zeno_dj_ultra.iouqt_crossbar_gemm_out);
            println!("      • ALL IN / ALL OUT  : {} Input Channels × {} Output Buses | {} Active DJ Decks",
                full.zeno_dj_ultra.all_in_channels.len(), full.zeno_dj_ultra.all_out_buses.len(), full.zeno_dj_ultra.decks.len());
            println!("      • Zeno Discipline   : ceiling={:.6} | ZA-0001 volatile scrub={}",
                full.zeno_dj_ultra.zeno_ceiling_q16, full.zeno_dj_ultra.zeno_za0001_volatile_scrub_verified);

            println!("\n  [5] ULTRA-OMNIPOTENT-SHIVA :: G650ER TOKYO (RJTT) ⇄ NORFOLK (KORF) AIRLIFT:");
            println!("      • Route & Range     : {:.1} nm each way ({:.1} nm round-trip) | Non-Stop={} (+{:.1} nm reserve)",
                full.shiva_g6_airlift.leg1_outbound.great_circle_nm, full.shiva_g6_airlift.total_roundtrip_nm,
                full.shiva_g6_airlift.leg1_outbound.nonstop_capable, full.shiva_g6_airlift.leg1_outbound.g650er_reserve_margin_nm);
            println!("      • Leg 1 (Outbound)  : {}", full.shiva_g6_airlift.leg1_outbound.passengers.join(" + "));
            println!("      • Leg 2 (Return)    : {}", full.shiva_g6_airlift.leg2_return_with_kids.passengers.join(" + "));
            println!("      • Zeno Settlement   : ¥{} JPY settled | ¥{} JPY broker toll burned",
                full.shiva_g6_airlift.zeno_direct_operator_jpy, full.shiva_g6_airlift.broker_markup_burned_jpy);

            println!("\n  [6] MOMMA BEAR OMNI COVENANT :: LÓDRI ⋈ CHENOWETH REALITY SCAFFOLDING:");
            println!("      • Lineage & Tagline : {} | \"{}\"", full.momma_bear_omni.merged_lineages, full.momma_bear_omni.foundation_tagline);
            println!("      • SpherePOP Harmony : `{}` = {} Hz (replay_ok={})",
                full.momma_bear_omni.spherepop_merge_expr, full.momma_bear_omni.spherepop_harmonic_hz_sum, full.momma_bear_omni.spherepop_replay_verified);
            println!("      • CT-000 & ZA-0006  : CT-000={} | QWave 79B={} | ZA-0006 Zeroized={}",
                full.momma_bear_omni.ct000_momma_bear_omni_holds, full.momma_bear_omni.qwave_79b_precious_tau0_verified, full.momma_bear_omni.zeno_za0006_volatile_zeroized);

            println!("\n  [7] WUPZ INCREMENTAL LINKER & PACKER × MLX-QUANT (v2.0.0-RC1 'Zeno'):");
            println!("      • WUPZ Plan Image   : {}B (magic='{}') | {:.1}% block reuse (repacked={:?}) | windows_touched={:?}",
                full.wupz_mlx_quant.wupz_linker.packed_plan_image_bytes,
                full.wupz_mlx_quant.wupz_linker.magic_header,
                full.wupz_mlx_quant.wupz_linker.incremental_block_reuse_ratio * 100.0,
                full.wupz_mlx_quant.wupz_linker.incremental_blocks_repacked,
                full.wupz_mlx_quant.wupz_linker.incremental_windows_touched);
            println!("      • MLX-QUANT b1.58   : γ={:.5} | MP bound={:.4} (est={:.4}) | NEON sdot={:?} (hw={}) | Fifike {:.0}/{:.0}Hz",
                full.wupz_mlx_quant.mlx_quant.gamma_scale_factor,
                full.wupz_mlx_quant.mlx_quant.marchenko_pastur_smax_bound,
                full.wupz_mlx_quant.mlx_quant.empirical_spectral_norm_est,
                full.wupz_mlx_quant.mlx_quant.aarch64_neon_sdot_lane_sums,
                full.wupz_mlx_quant.mlx_quant.aarch64_neon_hw_executed,
                full.wupz_mlx_quant.mlx_quant.fifike_purr_hz.0,
                full.wupz_mlx_quant.mlx_quant.fifike_purr_hz.1);
        }
    }
}


