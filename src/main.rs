//! `8b-is-music` CLI & Sovereign Audio Orchestrator.
//!
//! Commands:
//! - `8b-is-music status [--layers N] [--topology split|couple|wall] [--json]`
//! - `8b-is-music shiva-g6 [--seconds S] [--out FILE.wav] [--play] [--json]`
//! - `8b-is-music dj [--layers N] [--seconds S] [--out FILE.wav] [--apply-live] [--play] [--json]`
//! - `8b-is-music quant65k [--layers N] [--inspect-layer L] [--json]`
//! - `8b-is-music bluetooth [--topology split|couple|wall] [--apply-live] [--json]`
//! - `8b-is-music render [--track future|quantshuffle|mem8] [--seconds S] [--layers N] [--out FILE.wav] [--play]`

use eight_b_is_music::{
    build_full_system_report, execute_shiva_g6_airlift, render_shiva_g6_cabin_wav,
    render_wav_track, verify_momma_bear_omni_covenant, MultiBluetoothRouter,
    MultiBluetoothTopology, Quant65kEngine, SynthTrack, ZenoDjUltraEngine,
    DEFAULT_QUANT65K_PLUS_LAYERS,
};
use std::env;
use std::path::PathBuf;
use std::process::Command;

fn parse_topology(val: &str) -> MultiBluetoothTopology {
    match val.to_ascii_lowercase().as_str() {
        "couple" | "dual" | "dual-airpods" => MultiBluetoothTopology::DualAirPodsCouple,
        "wall" | "constellation" | "all" | "multi" => MultiBluetoothTopology::FullConstellationWall,
        _ => MultiBluetoothTopology::SplitEarZen,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let subcmd = args.get(1).map(|s| s.as_str()).unwrap_or("status");

    let mut layers = DEFAULT_QUANT65K_PLUS_LAYERS;
    let mut inspect_layer: Option<usize> = None;
    let mut topology = MultiBluetoothTopology::FullConstellationWall;
    let mut apply_live = false;
    let mut json_mode = false;
    let mut track = SynthTrack::FutureWeRemember;
    let mut seconds: f32 = 8.0;
    let mut out_path = PathBuf::from("8b-is-music-master.wav");
    let mut play_after = false;

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
            "--play" => play_after = true,
            _ => {}
        }
        i += 1;
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
                    "    • {:<5} | {:<52} | {} Hz | gain={:.2} | HFP-Guard={} | trit={:+}",
                    ch.id, ch.name, ch.sample_rate_hz, ch.gain_scalar, ch.hfp_guard_locked, ch.quanttern_trit
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

            if apply_live {
                println!("\n  [CoreAudio Live Hardware Bridge]");
                if let Ok(out) = dj.bt_router.apply_live_macos_routing() {
                    println!("{}", out.trim());
                }
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
                println!("  ▶ Playing {} across M1-ULTRA DJ Split-Ear + Multi-Bluetooth rig...", mix_summary.output_path);
                let _ = Command::new("afplay").arg(&mix_summary.output_path).status();
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
            println!("  • Active Topology      : {} ({}/{} endpoints active)", report.topology, report.active_endpoints, report.total_known_endpoints);
            println!("  • Aggregate Device     : {} [{}]", report.aggregate_device_name, report.aggregate_uid);
            println!("  • Max BT A2DP Latency  : {:.1} ms (phase-aligned at 48,000 Hz)", report.max_bluetooth_latency_ms);
            println!("  • CT-000 Admissibility : theorem_holds={} | keeper_canon={} | HFP 24kHz Blocked={}",
                report.ct000_proof.theorem_holds, report.ct000_proof.keeper_canon_verified, report.ct000_proof.hfp_24khz_input_blocked);
            println!("  • Default Input Guard  : {}", report.ct000_proof.default_input_locked_to);
            println!("\n  Discovered & Configured Endpoints:");
            for ep in &report.endpoints {
                let state = if ep.connected { "ACTIVE  " } else { "STANDBY " };
                println!(
                    "    [{}] {:<38} | {:<18} | role={:?} | pan={:.2} | lat={:>4.1}ms | +{:>4} samples",
                    state, ep.name, ep.mac_or_uid, ep.role, ep.stereo_pan, ep.acoustic_latency_ms, ep.injected_delay_samples_48k
                );
            }
            if apply_live {
                println!("\n  Applying live macOS CoreAudio split-ear & Multi-Bluetooth configuration...");
                match router.apply_live_macos_routing() {
                    Ok(out) => println!("{}", out.trim()),
                    Err(e) => eprintln!("CoreAudio bridge warning: {e}"),
                }
            }
        }
        "render" => {
            let summary = render_wav_track(track, seconds, layers, topology, &out_path)
                .expect("Failed to render WAV track");
            if json_mode {
                println!("{}", serde_json::to_string_pretty(&summary).unwrap());
            } else {
                println!("╔══════════════════════════════════════════════════════════════════════════╗");
                println!("║  8b-is-music :: QUANT65K+ MULTI-CHANNEL WAV RENDER COMPLETE              ║");
                println!("╚══════════════════════════════════════════════════════════════════════════╝");
                println!("  • Track                : {}", summary.track_title);
                println!("  • Output WAV           : {}", summary.output_path);
                println!("  • Format               : {} Hz, {}-ch 16-bit PCM ({:.1}s, {} frames)",
                    summary.sample_rate_hz, summary.channels, summary.duration_seconds, summary.total_frames);
                println!("  • Quant65k+ Layers     : {} simultaneous ternary wave layers", summary.quant65k_layers_active);
                println!("  • Stereo Peak Levels   : Left={:.4} | Right={:.4}", summary.peak_left, summary.peak_right);
                println!("  • QuantTern & WUPZ ABI : tag={} | wupz_xor=0x{:02x}", summary.quanttern_emotion_tag, summary.wupz_abi_xor);
            }
            if play_after {
                println!("  ▶ Playing {} via macOS CoreAudio (`afplay`)...", summary.output_path);
                let _ = Command::new("afplay").arg(&summary.output_path).status();
            }
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

            let wav_file = if out_path == PathBuf::from("8b-is-music-master.wav") {
                PathBuf::from("g6-shiva-cabin-sanctuary.wav")
            } else {
                out_path
            };
            if let Ok((saved, pl, pr)) = render_shiva_g6_cabin_wav(seconds, &wav_file) {
                println!("    • G6 Cabin WAV       : {} (108/432/528/888 Hz | Peak L={:.4}, R={:.4})", saved, pl, pr);
                if play_after {
                    let _ = Command::new("afplay").arg(&saved).status();
                }
            }
        }
        "youtube" | "youtube-full" | "/youtube-full" => {
            let url = args
                .get(2)
                .map(|s| s.as_str())
                .unwrap_or("https://youtu.be/4NRXx6U8ABQ?list=RD4NRXx6U8ABQ");
            let mut dj = ZenoDjUltraEngine::new(layers);
            let _ = dj.bt_router.apply_live_macos_routing();
            let report = dj.verify_zeno_iouqt();
            println!("╔══════════════════════════════════════════════════════════════════════════╗");
            println!("║  8b-is-music :: YOUTUBE-FULL -> M1-ULTRA DJ IOuQT DECK STREAM            ║");
            println!("╚══════════════════════════════════════════════════════════════════════════╝");
            println!("  • Stream URL       : {}", url);
            println!("  • IOuQT SIMD Lane  : `{}` (Zeno ceiling={:.6}, QuantTern={})",
                report.uq_native_simd_lane, report.zeno_ceiling_q16, report.iouqt_master_quanttern_tag);
            println!("  • Output Routing   : M1 MIDI: Left Ear Jack (Pan 0.0) + Right Ear AirPods (Pan 1.0) @ 48,000 Hz");
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
            println!("      • Aggregate         : {} ({}/{} endpoints active)", full.bluetooth.aggregate_device_name, full.bluetooth.active_endpoints, full.bluetooth.total_known_endpoints);
            println!("      • CT-000 Proof      : holds={} | keeper_canon={} | HFP 24kHz Blocked={}",
                full.bluetooth.ct000_proof.theorem_holds, full.bluetooth.ct000_proof.keeper_canon_verified, full.bluetooth.ct000_proof.hfp_24khz_input_blocked);
            println!("      • Phase Alignment   : max BT latency {:.1} ms compensated at 48,000 Hz", full.bluetooth.max_bluetooth_latency_ms);

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
        }
    }
}


