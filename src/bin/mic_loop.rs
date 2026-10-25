//! mic-loop — the environment listener.
//!
//! Continuous microphone windows seed the Quant65k+ stack: every window is
//! hashed into a deterministic seed phrase, rendered as a short seeded track
//! through the current system output (`afplay`), and logged as a quant sign
//! into the private `8b-is/music-quant-signs` bucket (git push + Hugging
//! Face upload, batched every 10 windows).
//!
//! ```bash
//! cargo run --release --bin mic-loop -- --once            # one window, then exit
//! cargo run --release --bin mic-loop                     # listen continuously
//! ```

use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;

use eight_b_is_music::{
    render_wav_track_seeded, MultiBluetoothTopology, SynthTrack, ZenoDjUltraEngine,
};
use sha2::{Digest, Sha256};

const WINDOW_SECS: u32 = 3;
const SIGNS_BATCH: u32 = 10;

fn run(cmd: &str, args: &[&str]) -> Option<String> {
    Command::new(cmd)
        .args(args)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).into_owned())
}

fn record_window(path: &PathBuf) -> Option<Vec<u8>> {
    // AVFoundation device indices shift when Bluetooth peripherals connect or
    // disconnect — try the first few input slots and keep the first success.
    for idx in 0..4 {
        let _ = Command::new("ffmpeg")
            .args([
                "-y",
                "-f",
                "avfoundation",
                "-i",
                &format!(":{idx}"),
                "-t",
                &WINDOW_SECS.to_string(),
                "-ar",
                "48000",
                "-ac",
                "1",
            ])
            .arg(path)
            .output();
        if let Ok(bytes) = fs::read(path) {
            if !bytes.is_empty() {
                return Some(bytes);
            }
        }
    }
    None
}

fn audio_stats(path: &PathBuf) -> (f32, f32, f32, f32, f32) {
    let Ok(out) = Command::new("ffmpeg")
        .arg("-i")
        .arg(path)
        .args(["-af", "astats=metadata=0", "-f", "null", "-"])
        .output()
    else {
        return (0.0, 0.0, 0.0, 0.0, 0.0);
    };
    let txt = String::from_utf8_lossy(&out.stderr).into_owned();
    let grab = |key: &str| -> f32 {
        txt.lines()
            .find_map(|l| {
                let rest = l.split(&format!("{key}: ")).nth(1)?;
                rest.split_whitespace().next()?.parse().ok()
            })
            .unwrap_or(0.0)
    };
    (
        grab("RMS level dB"),
        grab("Peak level dB"),
        grab("Zero crossings"),
        grab("Crest factor"),
        grab("Flat factor"),
    )
}

fn tern(v: bool) -> i32 {
    if v { 1 } else { 0 }
}

fn play(path: &str) {
    let _ = Command::new("afplay").arg(path).spawn();
}

fn quant_sign_path(seed: &str) -> PathBuf {
    let base = std::env::var("WA_QUANT_DIR").unwrap_or_else(|_| {
        "/Users/lodripeter/workspace/peterlodri-sec/music-quant-signs".to_string()
    });
    PathBuf::from(base)
        .join("signs")
        .join(format!("mic-loop-{seed}.json"))
}

fn sync_bucket(signs_dir: &PathBuf) {
    let dir = signs_dir
        .parent()
        .unwrap_or(&PathBuf::from("."))
        .to_path_buf();
    let _ = run("git", &["add", "signs"]);
    let _ = Command::new("git")
        .args(["commit", "-q", "-m", "chore(mic-loop): sync quant signs"])
        .current_dir(&dir)
        .output();
    let _ = Command::new("git")
        .args(["push", "-q", "origin", "main"])
        .current_dir(&dir)
        .output();
    if let Ok(entries) = fs::read_dir(signs_dir) {
        for e in entries.flatten().filter(|e| e.path().extension().is_some_and(|x| x == "json")) {
            let p = e.path();
            let _ = Command::new("hf")
                .args([
                    "upload",
                    "PeetPedro/music-quant-signs",
                    p.to_str().unwrap_or(""),
                    &format!("signs/{}", p.file_name().unwrap_or_default().to_string_lossy()),
                    "--type",
                    "dataset",
                    "--private",
                ])
                .output();
        }
    }
}

fn one_window(play_audio: bool) -> Option<String> {
    let win = PathBuf::from("/tmp/mic-loop-win.wav");
    let bytes = record_window(&win)?;
    if bytes.is_empty() {
        return None;
    }
    let seed_hex = format!("{:x}", Sha256::digest(&bytes));
    let seed = &seed_hex[..16];

    let (rms, peak, zc, crest, flat) = audio_stats(&win);

    let track = match seed.as_bytes().last().copied().unwrap_or(0) % 4 {
        0 => SynthTrack::FutureWeRemember,
        1 => SynthTrack::QuantshuffleCanon,
        2 => SynthTrack::Mem8WaveAndMemory,
        _ => SynthTrack::AlexandraSanctuary,
    };
    let seconds = 4 + (seed.as_bytes()[0] as u32 % 5);
    let layers = 8192 + ((seed.as_bytes()[1] as usize % 5) * 4096);

    let out = "/tmp/mic-loop-out.wav";

    // Every fifth window runs the full DJ deck — 4 input channels, 6 output
    // buses, the ALL IN / ALL OUT path — so the music channels stay exercised.
    if seed.as_bytes()[2] % 5 == 0 {
        let mut dj = ZenoDjUltraEngine::new(layers);
        let summary = dj
            .render_dj_ultra_mix_wav(seconds as f32, &PathBuf::from(out))
            .ok()?;
        if play_audio {
            play(out);
        }
        let sign = serde_json::json!({
            "id": format!("mic-loop-{seed}"),
            "title": format!("dj deck window — {seed}"),
            "artist": "the room, continuously",
            "year": "2026",
            "genre": "mic-loop",
            "album": "",
            "source": "microphone",
            "kind": "environment",
            "url": "",
            "date": chrono_lite_date(),
            "digest": seed_hex,
            "quant": [
                {"register": "seed", "signal": seed, "ternary": 1},
                {"register": "rms_db", "signal": format!("{rms:.1}"), "ternary": tern(rms > -40.0)},
                {"register": "zero_xings", "signal": format!("{zc:.0}"), "ternary": tern(zc > 200.0)},
                {"register": "channel_path", "signal": "dj-4-in-6-out", "ternary": 1},
                {"register": "output_bus", "signal": summary.output_path, "ternary": 1},
            ],
        });
        let sign_path = quant_sign_path(seed);
        if let Some(parent) = sign_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let _ = fs::write(&sign_path, serde_json::to_string_pretty(&sign).unwrap() + "\n");
        println!(
            "[mic-loop] {seed} rms={rms:.1} zc={zc:.0} -> DJ DECK {seconds}s, {layers} layers"
        );
        return Some(seed.to_string());
    }

    let summary = render_wav_track_seeded(
        seed,
        track,
        seconds as f32,
        layers,
        MultiBluetoothTopology::FullConstellationWall,
        &PathBuf::from(out),
    )
    .ok()?;

    if play_audio {
        play(out);
    }

    let sign = serde_json::json!({
        "id": format!("mic-loop-{seed}"),
        "title": format!("environment window — {seed}"),
        "artist": "the room, continuously",
        "year": "2026",
        "genre": "mic-loop",
        "album": "",
        "source": "microphone",
        "kind": "environment",
        "url": "",
        "date": chrono_lite_date(),
        "digest": seed_hex,
        "quant": [
            {"register": "seed", "signal": seed, "ternary": 1},
            {"register": "rms_db", "signal": format!("{rms:.1}"), "ternary": tern(rms > -40.0)},
            {"register": "peak_db", "signal": format!("{peak:.1}"), "ternary": tern(peak > -6.0)},
            {"register": "zero_xings", "signal": format!("{zc:.0}"), "ternary": tern(zc > 200.0)},
            {"register": "crest", "signal": format!("{crest:.2}"), "ternary": tern(crest > 10.0)},
            {"register": "flatness", "signal": format!("{flat:.2}"), "ternary": if flat < 100.0 { -1 } else { 0 }},
            {"register": "track", "signal": track.title(), "ternary": 1},
            {"register": "layers", "signal": summary.quant65k_layers_active.to_string(), "ternary": 1},
        ],
    });
    let sign_path = quant_sign_path(seed);
    if let Some(parent) = sign_path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::write(&sign_path, serde_json::to_string_pretty(&sign).unwrap() + "\n");

    println!(
        "[mic-loop] {seed} rms={rms:.1} zc={zc:.0} crest={crest:.2} -> {} ({seconds}s, {layers} layers, qt:{})",
        track.title(),
        summary.quanttern_emotion_tag
    );
    Some(seed.to_string())
}

fn chrono_lite_date() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or(Duration::ZERO)
        .as_secs();
    let days = secs / 86400;
    let (y, m, d) = civil_from_days(days as i64);
    format!("{y:04}-{m:02}-{d:02}")
}

fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let once = args.iter().any(|a| a == "--once");
    let play_audio = !args.iter().any(|a| a == "--no-play");

    if once {
        match one_window(play_audio) {
            Some(seed) => println!("[mic-loop] once done: {seed}"),
            None => {
                eprintln!("[mic-loop] window capture failed (mic available?)");
                std::process::exit(1);
            }
        }
        return;
    }

    println!("[mic-loop] listening continuously — {WINDOW_SECS}s windows, batched sync every {SIGNS_BATCH}");
    let mut count = 0u32;
    loop {
        if let Some(seed) = one_window(play_audio) {
            count += 1;
            if count.is_multiple_of(SIGNS_BATCH) {
                sync_bucket(&quant_sign_path(&seed));
                println!("[mic-loop] synced {count} windows to the bucket");
            }
        }
    }
}
