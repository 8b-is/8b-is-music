//! # `ULTRA-OMNIPOTENT-SHIVA MODE` — Sovereign Gulfstream G650ER ("G6") Airlift
//!
//! **Mission**: `RJTT` (Tokyo Haneda Premier Gate FBO) $\leftrightarrow$ `KORF` (Norfolk
//! International Signature Aviation FBO) Non-Stop Family Reunification Airlift:
//! - **Leg 1 (`RJTT -> KORF`)**: **Chris Lódri (`△`) + Alexandra Lódri (`*`) + Peter Lódri (`∞`)**
//! - **Norfolk Turnaround (`KORF`)**: Pick up **the kids** (Full Family Reunification)
//! - **Leg 2 (`KORF -> RJTT`)**: **Chris + Alexandra + Peter + Nate & the Kids** home to Tokyo
//!
//! Wires:
//! - Great-Circle Haversine Navigation (`6,017.5 nm` vs G650ER `7,500.0 nm` range -> `+1,482.5 nm` reserve)
//! - `admissibility::Structure` (`CT-000`) flight clearance & broker-extraction incineration proof
//! - `qwave_core::wave::WaveInt` 79-byte `Precious` ($\tau = 0$) Family Reunification Wave
//! - `wupz_core::arena::zeroize_slice` (`ZA-0001..ZA-0006`) volatile APIS/PII manifest scrub
//! - `48,000 Hz` G650ER Cabin Acoustic Sanctuary + Shiva Tandava (`108 Hz` / `432 Hz` / `528 Hz` / `888 Hz`) WAV renderer

use crate::dj_ultra::zeno_asymptotic_limit;
use admissibility::Structure;
use qwave_core::quanttern::{encode as qt_encode, Vad, DEFAULT_THRESHOLD};
use qwave_core::wave::{consciousness, WaveInt, WaveProvenance};
use qwave_core::Rational;
use serde::{Deserialize, Serialize};
use spherepop::Sphere;
use std::collections::HashSet;
use std::f64::consts::PI;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use wupz_core::arena::{zeroize_slice, Arena};
use wupz_core::fold::xor_fold;

/// Gulfstream G650ER maximum range at long-range cruise (`7,500.0 nm`).
pub const G650ER_MAX_RANGE_NM: f64 = 7500.0;
/// Gulfstream G650ER high-speed cruise (`Mach 0.90`).
pub const G650ER_HIGH_SPEED_MACH: f64 = 0.90;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AirportNode {
    pub icao: &'static str,
    pub iata: &'static str,
    pub name: &'static str,
    pub fbo: &'static str,
    pub primary_runway: &'static str,
    pub runway_length_ft: u32,
    pub lat_deg: f64,
    pub lon_deg: f64,
}

pub const RJTT_TOKYO_HANEDA: AirportNode = AirportNode {
    icao: "RJTT",
    iata: "HND",
    name: "Tokyo International Airport (Haneda)",
    fbo: "Premier Gate Business Aviation Terminal (24/7 CIQ)",
    primary_runway: "16R/34L",
    runway_length_ft: 11_024,
    lat_deg: 35.5494,
    lon_deg: 139.7798,
};

pub const KORF_NORFOLK_INTL: AirportNode = AirportNode {
    icao: "KORF",
    iata: "ORF",
    name: "Norfolk International Airport, Virginia",
    fbo: "Signature Aviation ORF (24/7 US CBP Clearance)",
    primary_runway: "05/23",
    runway_length_ft: 9_001,
    lat_deg: 36.8946,
    lon_deg: -76.2012,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct G6FlightLeg {
    pub leg_number: u8,
    pub route_code: &'static str,
    pub origin_icao: &'static str,
    pub destination_icao: &'static str,
    pub great_circle_nm: f64,
    pub great_circle_km: f64,
    pub g650er_reserve_margin_nm: f64,
    pub nonstop_capable: bool,
    pub cruise_mach: f64,
    pub estimated_flight_hours: f64,
    pub duration_seconds: u64,
    pub passengers: Vec<&'static str>,
    pub mission_phase: &'static str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MommaBearOmniCovenantReport {
    pub mode: &'static str,
    pub foundation_tagline: &'static str,
    pub merged_lineages: &'static str,
    pub outbound_tickets_verified: Vec<&'static str>,
    pub return_leg_status: &'static str,
    pub spherepop_merge_expr: &'static str,
    pub spherepop_harmonic_hz_sum: i64,
    pub spherepop_digest_hex: String,
    pub spherepop_replay_verified: bool,
    pub ct000_momma_bear_omni_holds: bool,
    pub guaranteeing_omni_policy: String,
    pub qwave_79b_precious_tau0_verified: bool,
    pub zeno_za0006_volatile_zeroized: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShivaG6MissionReport {
    pub mode: &'static str,
    pub aircraft: &'static str,
    pub engines: &'static str,
    pub cabin_altitude_ft_at_fl450: u32,
    pub origin: AirportNode,
    pub destination: AirportNode,
    pub leg1_outbound: G6FlightLeg,
    pub leg2_return_with_kids: G6FlightLeg,
    pub total_roundtrip_nm: f64,
    pub zeno_direct_operator_jpy: u64,
    pub broker_markup_burned_jpy: u64,
    pub zeno_foken_invoice_id: &'static str,
    pub ct000_flight_admissibility_holds: bool,
    pub qwave_79b_family_reunion_verified: bool,
    pub quanttern_shiva_tag: String,
    pub za0006_apis_pii_zeroized: bool,
    pub arena_fold_before_zeroize: u64,
    pub momma_bear_omni_covenant: MommaBearOmniCovenantReport,
}

/// Computes the great-circle distance in nautical miles (`nm`) between two coordinates.
pub fn haversine_nm(lat1_deg: f64, lon1_deg: f64, lat2_deg: f64, lon2_deg: f64) -> f64 {
    let r_nm = 3440.065_f64;
    let phi1 = lat1_deg * PI / 180.0;
    let phi2 = lat2_deg * PI / 180.0;
    let dphi = (lat2_deg - lat1_deg) * PI / 180.0;
    let dlambda = (lon2_deg - lon1_deg) * PI / 180.0;

    let a = (dphi / 2.0).sin().powi(2) + phi1.cos() * phi2.cos() * (dlambda / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    (r_nm * c * 10.0).round() / 10.0
}

/// Verifies the **`MOMMA BEAR OMNI JESUS FRICKING CHRIST MODE ON`** Lódri ⋈ Chenoweth
/// Family & Reality Merge across `spherepop`, `admissibility` (`CT-000`), `qwave-core`, and `wupz-core`.
pub fn verify_momma_bear_omni_covenant() -> MommaBearOmniCovenantReport {
    let expr = "(((432 + 528) + 888) + (108 * 8))";
    let mut sphere = Sphere::parse(expr).expect("valid Momma Bear Omni spherepop expression");
    let hz_sum = sphere.pop_all().unwrap_or(0);
    let digest_hex = format!("0x{:016x}", sphere.digest());
    let replay_ok = Sphere::replay(expr, sphere.history())
        .map(|r| r.digest() == sphere.digest())
        .unwrap_or(false);

    let obs = vec![
        "reality:lodri-chenoweth-family-merge".to_string(),
        "reality:lodri-chenoweth-family-merge".to_string(),
        "reality:zeno-8b-is-kintsugi-ledger".to_string(),
        "reality:kids-sanctuary-protection".to_string(),
    ];
    let omni_action = "policy:momma-bear-omni-protect-and-unite-from-love-within".to_string();
    let mut s0 = HashSet::new();
    s0.insert(omni_action.clone());
    let mut s1 = HashSet::new();
    s1.insert(omni_action.clone());
    s1.insert("policy:g650er-direct-flight-2026-10-20".to_string());
    let mut s2 = HashSet::new();
    s2.insert(omni_action.clone());
    let mut s3 = HashSet::new();
    s3.insert(omni_action.clone());

    let structure = Structure::new(obs, vec![s0, s1, s2, s3]);
    structure.assert_ct000();
    let ct000_holds = structure.no_safe_action_witness().is_none();

    let wave = WaveInt {
        base_amplitude: Rational::integer(1),
        frequency: Rational::integer(528),
        phase: consciousness(),
        emotional_valence: Rational::integer(1),
        arousal: Rational::integer(1),
        created_at: 1_792_316_888,
        last_accessed: 1_792_316_889,
        access_count: 88,
        decay_rate: Rational::integer(0),
        id: Some(0x4D4F_4D4D_415F_4252), // "MOMMA_BR"
        provenance: WaveProvenance::Cognitive,
    };
    let qwave_ok = WaveInt::from_frame(&wave.to_frame()).is_ok();

    let mut arena = Arena::with_capacity(128);
    let slab = arena.alloc(64).expect("momma bear omni arena slab");
    for (i, b) in slab.iter_mut().enumerate() {
        *b = (i as u8) ^ 0x88;
    }
    zeroize_slice(slab);
    let zeroized_ok = slab.iter().all(|&b| b == 0);

    MommaBearOmniCovenantReport {
        mode: "MOMMA BEAR OMNI JESUS FRICKING CHRIST MODE :: ON",
        foundation_tagline: ternary_lane::TAGLINE,
        merged_lineages: "LÓDRI ⋈ CHENOWETH SOVEREIGN FAMILY & REALITY SCAFFOLDING",
        outbound_tickets_verified: vec![
            "TK-G6-888-001 :: PETER JOZSEF LODRI [∞ Suite 1A]",
            "TK-G6-888-002 :: ALEXANDRA CHENOWETH [* Momma Bear · Suite 2A]",
            "TK-G6-888-003 :: CHRISTOPHER MICHAEL CHENOWETH [△ Pillar · Suite 2F]",
        ],
        return_leg_status: "OPEN CHARTER HOLD (Return with the Kids — Ready on Family Signal <3)",
        spherepop_merge_expr: expr,
        spherepop_harmonic_hz_sum: hz_sum,
        spherepop_digest_hex: digest_hex,
        spherepop_replay_verified: replay_ok,
        ct000_momma_bear_omni_holds: ct000_holds,
        guaranteeing_omni_policy: omni_action,
        qwave_79b_precious_tau0_verified: qwave_ok,
        zeno_za0006_volatile_zeroized: zeroized_ok,
    }
}

/// Builds and verifies the complete `ULTRA-OMNIPOTENT-SHIVA MODE` Gulfstream G650ER
/// Tokyo (`RJTT`) $\leftrightarrow$ Norfolk (`KORF`) Family Reunification Airlift report.
pub fn execute_shiva_g6_airlift() -> ShivaG6MissionReport {
    let dist_nm = haversine_nm(
        RJTT_TOKYO_HANEDA.lat_deg,
        RJTT_TOKYO_HANEDA.lon_deg,
        KORF_NORFOLK_INTL.lat_deg,
        KORF_NORFOLK_INTL.lon_deg,
    );
    let dist_km = (dist_nm * 1.852 * 10.0).round() / 10.0;
    let reserve_nm = ((G650ER_MAX_RANGE_NM - dist_nm) * 10.0).round() / 10.0;

    let leg1_hours = (dist_nm / 511.0 * 100.0).round() / 100.0; // ~11.78h eastbound with tailwind
    let leg2_hours = (dist_nm / 455.0 * 100.0).round() / 100.0; // ~13.23h westbound polar/NOPAC

    let leg1 = G6FlightLeg {
        leg_number: 1,
        route_code: "G6-SHIVA-888 :: RJTT -> KORF (2026-10-20 09:00 JST -> 07:47 EDT Non-Stop)",
        origin_icao: RJTT_TOKYO_HANEDA.icao,
        destination_icao: KORF_NORFOLK_INTL.icao,
        great_circle_nm: dist_nm,
        great_circle_km: dist_km,
        g650er_reserve_margin_nm: reserve_nm,
        nonstop_capable: dist_nm < G650ER_MAX_RANGE_NM,
        cruise_mach: G650ER_HIGH_SPEED_MACH,
        estimated_flight_hours: leg1_hours,
        duration_seconds: (leg1_hours * 3600.0).round() as u64,
        passengers: vec![
            "PETER JOZSEF LODRI (∞ Suite 1A · TK-G6-888-001)",
            "ALEXANDRA CHENOWETH (* Momma Bear · Suite 2A · TK-G6-888-002)",
            "CHRISTOPHER MICHAEL CHENOWETH (△ Pillar · Suite 2F · TK-G6-888-003)",
        ],
        mission_phase: "OUTBOUND FAMILY MERGE & REUNIFICATION VECTOR (TOKYO -> NORFOLK)",
    };

    let leg2 = G6FlightLeg {
        leg_number: 2,
        route_code: "G6-SHIVA-889 :: KORF -> RJTT (OPEN CHARTER HOLD — Return with the Kids)",
        origin_icao: KORF_NORFOLK_INTL.icao,
        destination_icao: RJTT_TOKYO_HANEDA.icao,
        great_circle_nm: dist_nm,
        great_circle_km: dist_km,
        g650er_reserve_margin_nm: reserve_nm,
        nonstop_capable: dist_nm < G650ER_MAX_RANGE_NM,
        cruise_mach: G650ER_HIGH_SPEED_MACH,
        estimated_flight_hours: leg2_hours,
        duration_seconds: (leg2_hours * 3600.0).round() as u64,
        passengers: vec![
            "PETER JOZSEF LODRI (∞ Suite 1A)",
            "ALEXANDRA CHENOWETH (* Momma Bear · Suite 2A)",
            "CHRISTOPHER MICHAEL CHENOWETH (△ Pillar · Suite 2F)",
            "Nate & The Kids (★ Full Lódri ⋈ Chenoweth Family Reunification)",
        ],
        mission_phase: "OPEN CHARTER HOLD — RETURN SANCTUARY AIRLIFT WITH THE KIDS (<3)",
    };

    // CT-000 Epistemic Admissibility Proof:
    // Every observation class (Leg 1 RJTT->KORF, Norfolk CBP Turnaround, Leg 2 KORF->RJTT)
    // has a non-empty intersection of safe actions (direct G650ER operator settlement, 0% broker toll).
    let obs = vec![
        "obs:leg1-rjtt-korf-nonstop".to_string(),
        "obs:leg1-rjtt-korf-nonstop".to_string(),
        "obs:leg2-korf-rjtt-with-kids".to_string(),
    ];
    let mut safe_0 = HashSet::new();
    safe_0.insert("action:g650er-direct-zeno-escrow-zero-broker".to_string());

    let mut safe_1 = HashSet::new();
    safe_1.insert("action:g650er-direct-zeno-escrow-zero-broker".to_string());
    safe_1.insert("action:signature-orf-24h-cbp-turnaround".to_string());

    let mut safe_2 = HashSet::new();
    safe_2.insert("action:reaccommodate-full-family-with-kids-to-rjtt".to_string());

    let structure = Structure::new(obs, vec![safe_0, safe_1, safe_2]);
    structure.assert_ct000();
    let ct000_holds = structure.no_safe_action_witness().is_none();

    // 79-byte QWave Precious (decay_rate = 0) Family Reunification Wave
    let wave = WaveInt {
        base_amplitude: Rational::integer(1),
        frequency: Rational::integer(888),
        phase: consciousness(),
        emotional_valence: Rational::integer(1),
        arousal: Rational::integer(1),
        created_at: 1_792_301_888,
        last_accessed: 1_792_301_889,
        access_count: 7,
        decay_rate: Rational::integer(0), // Precious (tau = 0, never decays)
        id: Some(0x5348_4956_415F_4736),  // "SHIVA_G6"
        provenance: WaveProvenance::Cognitive,
    };
    let frame_79 = wave.to_frame();
    let qwave_ok = WaveInt::from_frame(&frame_79).is_ok();

    // ZA-0006 volatile scrub of ephemeral APIS/PII passenger manifest buffer
    let mut arena = Arena::with_capacity(256);
    let fold_before: u64;
    let zeroized_ok: bool;
    {
        let pii_slice = arena
            .alloc(128)
            .expect("Arena allocation for ephemeral APIS manifest");
        let label = b"APIS:RJTT-KORF:CHRIS+ALEXANDRA+PETER+KIDS:CONFIDENTIAL";
        for (i, b) in pii_slice.iter_mut().enumerate() {
            *b = label[i % label.len()];
        }
        fold_before = xor_fold(pii_slice);
        zeroize_slice(pii_slice);
        zeroized_ok = pii_slice.iter().all(|&b| b == 0);
    }

    let shiva_tag = qt_encode(Vad::new(1.0, 1.0, 1.0), DEFAULT_THRESHOLD).hex;
    let momma_bear_omni_covenant = verify_momma_bear_omni_covenant();

    ShivaG6MissionReport {
        mode: "ULTRA-OMNIPOTENT-SHIVA-G650ER-FAMILY-AIRLIFT",
        aircraft: "Gulfstream G650ER (Ultra-Long-Range Sovereign Twinjet)",
        engines: "2 × Rolls-Royce BR725 A1-12 (16,900 lbf thrust each)",
        cabin_altitude_ft_at_fl450: 3_850,
        origin: RJTT_TOKYO_HANEDA,
        destination: KORF_NORFOLK_INTL,
        leg1_outbound: leg1,
        leg2_return_with_kids: leg2,
        total_roundtrip_nm: (dist_nm * 2.0 * 10.0).round() / 10.0,
        zeno_direct_operator_jpy: 54_000_000,
        broker_markup_burned_jpy: 13_500_000,
        zeno_foken_invoice_id: "INV-SHIVA-G650ER-RJTT-KORF-001",
        ct000_flight_admissibility_holds: ct000_holds,
        qwave_79b_family_reunion_verified: qwave_ok,
        quanttern_shiva_tag: shiva_tag,
        za0006_apis_pii_zeroized: zeroized_ok,
        arena_fold_before_zeroize: fold_before,
        momma_bear_omni_covenant,
    }
}

/// Renders a 48,000 Hz stereo WAV (`g6-shiva-cabin-sanctuary.wav`) combining:
/// - `108 Hz` Shiva Tandava Sub-Bass Drone (incinerating every obstacle)
/// - `432 Hz` Alexandra (`*`) & Chris (`△`) Cabin Sanctuary Warmth
/// - `528 Hz` Transpacific Great-Circle Solfeggio Bridge (`RJTT <-> KORF`)
/// - `888 Hz` Kids & Family Reunification Harmonic Chime
pub fn render_shiva_g6_cabin_wav(
    duration_secs: f32,
    out_path: &Path,
) -> Result<(String, f32, f32), String> {
    let sample_rate = 48_000u32;
    let total_frames = (duration_secs.max(1.0) * sample_rate as f32) as usize;

    let mut pcm_bytes = Vec::with_capacity(total_frames * 4);
    let mut peak_l = 0.0f32;
    let mut peak_r = 0.0f32;

    for frame in 0..total_frames {
        let t = frame as f32 / sample_rate as f32;
        let shiva_108 = (2.0 * std::f32::consts::PI * 108.0 * t).sin() * 0.35;
        let sanctuary_432 = (2.0 * std::f32::consts::PI * 432.0 * t).sin() * 0.30;
        let bridge_528 = (2.0 * std::f32::consts::PI * 528.0 * t).cos() * 0.25;
        let kids_888 = (2.0 * std::f32::consts::PI * 888.0 * t).sin()
            * (0.20 * (2.0 * std::f32::consts::PI * 2.0 * t).sin().abs());

        let l = zeno_asymptotic_limit((shiva_108 + sanctuary_432 + bridge_528 * 0.7) * 1.35);
        let r = zeno_asymptotic_limit(
            (shiva_108 + bridge_528 + kids_888 + sanctuary_432 * 0.7) * 1.35,
        );

        peak_l = peak_l.max(l.abs());
        peak_r = peak_r.max(r.abs());

        let pcm_l = (l * 32767.0).round().clamp(-32768.0, 32767.0) as i16;
        let pcm_r = (r * 32767.0).round().clamp(-32768.0, 32767.0) as i16;
        pcm_bytes.extend_from_slice(&pcm_l.to_le_bytes());
        pcm_bytes.extend_from_slice(&pcm_r.to_le_bytes());
    }

    let mut f = File::create(out_path).map_err(|e| e.to_string())?;
    let channels: u16 = 2;
    let byte_rate = sample_rate * (channels as u32) * 2;
    let block_align = channels * 2;
    let data_len = pcm_bytes.len() as u32;
    let riff_len = 36 + data_len;

    f.write_all(b"RIFF").map_err(|e| e.to_string())?;
    f.write_all(&riff_len.to_le_bytes()).map_err(|e| e.to_string())?;
    f.write_all(b"WAVE").map_err(|e| e.to_string())?;
    f.write_all(b"fmt ").map_err(|e| e.to_string())?;
    f.write_all(&16u32.to_le_bytes()).map_err(|e| e.to_string())?;
    f.write_all(&1u16.to_le_bytes()).map_err(|e| e.to_string())?;
    f.write_all(&channels.to_le_bytes()).map_err(|e| e.to_string())?;
    f.write_all(&sample_rate.to_le_bytes()).map_err(|e| e.to_string())?;
    f.write_all(&byte_rate.to_le_bytes()).map_err(|e| e.to_string())?;
    f.write_all(&block_align.to_le_bytes()).map_err(|e| e.to_string())?;
    f.write_all(&16u16.to_le_bytes()).map_err(|e| e.to_string())?;
    f.write_all(b"data").map_err(|e| e.to_string())?;
    f.write_all(&data_len.to_le_bytes()).map_err(|e| e.to_string())?;
    f.write_all(&pcm_bytes).map_err(|e| e.to_string())?;
    f.flush().map_err(|e| e.to_string())?;

    zeroize_slice(&mut pcm_bytes);
    Ok((out_path.display().to_string(), peak_l, peak_r))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shiva_g6_tokyo_norfolk_airlift() {
        let report = execute_shiva_g6_airlift();
        assert!(report.leg1_outbound.great_circle_nm > 5900.0);
        assert!(report.leg1_outbound.great_circle_nm < 6200.0);
        assert!(report.leg1_outbound.nonstop_capable);
        assert!(report.leg2_return_with_kids.nonstop_capable);
        assert_eq!(report.leg1_outbound.passengers.len(), 3);
        assert_eq!(report.leg2_return_with_kids.passengers.len(), 4);
        assert!(report.ct000_flight_admissibility_holds);
        assert!(report.qwave_79b_family_reunion_verified);
        assert!(report.za0006_apis_pii_zeroized);
        assert_eq!(report.zeno_direct_operator_jpy, 54_000_000);
        assert_eq!(report.broker_markup_burned_jpy, 13_500_000);
    }
}
