//! Configured Multi-Bluetooth DSP topology and separate host observations.
//!
//! Endpoint identities, rates and latencies below are a preset, not measurements
//! of this Mac. Discovery can report Bluetooth connection state; wired presence,
//! aggregate routing, input locks and HFP enforcement are not verified here.
//! CT-000 checks the static routing-policy model only. The optional Swift bridge
//! is a separate host-changing action and its success is not a hardware readback.

use admissibility::keeper_instance::admissible_actions;
use admissibility::Structure;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashSet};
use std::process::Command;

/// Transport medium for an audio endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AudioTransport {
    Wired35mmJack,
    BuiltInSpeakers,
    BluetoothA2dpAac,
    BluetoothBleAudio,
    CoreAudioAggregate,
}

/// Spatial ear / room role assigned to a sink in a Multi-Bluetooth topology.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SpatialChannelRole {
    /// Hard-left ear (`StereoPan = 0.0`, Ch 1 only) — e.g. Wired 3.5mm Jack in Left Ear.
    LeftEarOnly,
    /// Hard-right ear (`StereoPan = 1.0`, Ch 2 only) — e.g. AirPods Pro in Right Ear.
    RightEarOnly,
    /// Full binaural stereo (`Ch 1 + Ch 2`) — e.g. Second pair of AirPods Pro (`Alexandra’s AirPods Pro`).
    BinauralStereo,
    /// Center / Mid Halo (`0.5 * (L + R)`) — e.g. `MacBook Pro Speakers` or portable Bluetooth speaker.
    CenterHalo,
    /// Sub-bass / Low-end room anchor (`< 250 Hz` weighted) — e.g. `Tronsmart Bang SE` / `soundcore Select 4 Go`.
    RoomSubAnchor,
}

/// Latest host observation; configured endpoints start unobserved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionStatus {
    Unknown,
    Connected,
    Disconnected,
}

/// A discovered or configured Bluetooth / CoreAudio endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioEndpoint {
    pub name: String,
    pub mac_or_uid: String,
    pub transport: AudioTransport,
    pub role: SpatialChannelRole,
    /// True only when the latest discovery explicitly reports a connection.
    pub connected: bool,
    /// Membership in the configured DSP topology, independent of host discovery.
    pub planned_enabled: bool,
    pub connection_status: ConnectionStatus,
    pub sample_rate_hz: u32,
    pub master_volume_scalar: f32,
    pub stereo_pan: f32,
    pub drift_compensation: bool,
    /// Configured estimate of acoustic / codec transport latency in milliseconds (`0.0 ms` wired, `22–42 ms` Bluetooth).
    pub acoustic_latency_ms: f32,
    /// Computed sample delay injected by `MultiBluetoothRouter` so all wavefronts arrive in phase at `48,000 Hz`.
    pub injected_delay_samples_48k: usize,
}

/// Preset Multi-Bluetooth + Wired spatial topologies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MultiBluetoothTopology {
    /// M1 MIDI Split-Ear Zen: Left Ear = 3.5mm Jack (`Pan 0.0`), Right Ear = AirPods Pro (`Pan 1.0`).
    SplitEarZen,
    /// Dual AirPods Pro Couple Mode: `Christopher’s AirPods Pro` + `Alexandra’s AirPods Pro` + `External Headphones`.
    DualAirPodsCouple,
    /// Crazy Multi-Channel Constellation Wall: Simultaneous Multi-Bluetooth across AirPods + Portable Speakers + Wired Jack + MBP Halo.
    FullConstellationWall,
}

impl MultiBluetoothTopology {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SplitEarZen => "split-ear-zen",
            Self::DualAirPodsCouple => "dual-airpods-couple",
            Self::FullConstellationWall => "full-constellation-wall",
        }
    }
}

/// Sample-accurate circular delay line for aligning fast wired endpoints (`0 ms`) with Bluetooth A2DP sinks (`24–42 ms`).
#[derive(Debug, Clone)]
pub struct DelayRingBuffer {
    buffer_l: Vec<f32>,
    buffer_r: Vec<f32>,
    write_pos: usize,
    delay_samples: usize,
}

impl DelayRingBuffer {
    pub fn new(delay_samples: usize) -> Self {
        let cap = (delay_samples + 1).max(2);
        Self {
            buffer_l: vec![0.0; cap],
            buffer_r: vec![0.0; cap],
            write_pos: 0,
            delay_samples,
        }
    }

    #[inline]
    pub fn process_frame(&mut self, in_l: f32, in_r: f32) -> (f32, f32) {
        if self.delay_samples == 0 {
            return (in_l, in_r);
        }
        let cap = self.buffer_l.len();
        self.buffer_l[self.write_pos] = in_l;
        self.buffer_r[self.write_pos] = in_r;
        let read_pos = (self.write_pos + cap - self.delay_samples) % cap;
        self.write_pos = (self.write_pos + 1) % cap;
        (self.buffer_l[read_pos], self.buffer_r[read_pos])
    }
}

/// CT-000 certificate for a static policy model, not physical routing evidence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ct000RoutingProof {
    pub theorem_holds: bool,
    pub keeper_canon_verified: bool,
    pub total_states: usize,
    pub total_observations: usize,
    pub guaranteeing_policy: BTreeMap<String, String>,
    /// None means host enforcement has not been observed.
    pub hfp_24khz_input_blocked: Option<bool>,
    pub default_input_locked_to: Option<String>,
    pub evidence_scope: String,
    pub planned_default_input: String,
    pub nominal_sample_rate_hz: u32,
}

/// Full report of the Multi-Bluetooth + CoreAudio engine state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiBluetoothReport {
    pub topology: String,
    pub aggregate_device_name: String,
    pub aggregate_uid: String,
    /// Number explicitly observed connected, not a count of routed outputs.
    pub active_endpoints: usize,
    pub planned_endpoints: usize,
    pub total_known_endpoints: usize,
    /// Maximum configured latency among planned Bluetooth endpoints; not measured.
    pub max_bluetooth_latency_ms: f32,
    pub endpoints: Vec<AudioEndpoint>,
    pub ct000_proof: Ct000RoutingProof,
}

/// The **Multi-Bluetooth + CoreAudio Spatial Router**.
#[derive(Debug, Clone)]
pub struct MultiBluetoothRouter {
    pub topology: MultiBluetoothTopology,
    pub endpoints: Vec<AudioEndpoint>,
    pub delay_lines: Vec<DelayRingBuffer>,
}

impl MultiBluetoothRouter {
    /// Creates a configured DSP plan and attempts a separate Bluetooth observation.
    pub fn new(topology: MultiBluetoothTopology) -> Self {
        let mut router = Self::planned(topology);
        Self::refresh_live_bluetooth_status(&mut router.endpoints);
        router
    }

    /// Deterministic DSP plan without host discovery or audio configuration.
    pub fn planned(topology: MultiBluetoothTopology) -> Self {
        let mut endpoints = Self::fleet_endpoints(topology);
        let delay_lines = Self::compute_delay_compensation(&mut endpoints);
        Self {
            topology,
            endpoints,
            delay_lines,
        }
    }

    /// Historical endpoint preset; these entries do not establish current presence.
    pub fn fleet_endpoints(topology: MultiBluetoothTopology) -> Vec<AudioEndpoint> {
        let all_active = matches!(topology, MultiBluetoothTopology::FullConstellationWall);
        let couple_mode = matches!(
            topology,
            MultiBluetoothTopology::DualAirPodsCouple
                | MultiBluetoothTopology::FullConstellationWall
        );

        vec![
            AudioEndpoint {
                name: "External Headphones (3.5mm Analog Jack)".to_string(),
                mac_or_uid: "BuiltInHeadphoneOutputDevice".to_string(),
                transport: AudioTransport::Wired35mmJack,
                role: SpatialChannelRole::LeftEarOnly,
                connected: false,
                planned_enabled: true,
                connection_status: ConnectionStatus::Unknown,
                sample_rate_hz: 48_000,
                master_volume_scalar: 1.0,
                stereo_pan: 0.0,
                drift_compensation: false,
                acoustic_latency_ms: 0.0,
                injected_delay_samples_48k: 0,
            },
            AudioEndpoint {
                name: "Christopher’s AirPods Pro - Find My".to_string(),
                mac_or_uid: "2c-18-09-d8-14-74".to_string(),
                transport: AudioTransport::BluetoothA2dpAac,
                role: SpatialChannelRole::RightEarOnly,
                connected: false,
                planned_enabled: true,
                connection_status: ConnectionStatus::Unknown,
                sample_rate_hz: 48_000,
                master_volume_scalar: 1.0,
                stereo_pan: 1.0,
                drift_compensation: true,
                acoustic_latency_ms: 24.0,
                injected_delay_samples_48k: 0,
            },
            AudioEndpoint {
                name: "Alexandra’s AirPods Pro - Find My".to_string(),
                mac_or_uid: "2c-32-6a-c3-4a-91".to_string(),
                transport: AudioTransport::BluetoothA2dpAac,
                role: SpatialChannelRole::BinauralStereo,
                connected: false,
                planned_enabled: couple_mode,
                connection_status: ConnectionStatus::Unknown,
                sample_rate_hz: 48_000,
                master_volume_scalar: 1.0,
                stereo_pan: 0.5,
                drift_compensation: true,
                acoustic_latency_ms: 24.0,
                injected_delay_samples_48k: 0,
            },
            AudioEndpoint {
                name: "MacBook Pro Speakers (M3 6-Speaker Halo)".to_string(),
                mac_or_uid: "BuiltInSpeakerDevice".to_string(),
                transport: AudioTransport::BuiltInSpeakers,
                role: SpatialChannelRole::CenterHalo,
                connected: false,
                planned_enabled: true,
                connection_status: ConnectionStatus::Unknown,
                sample_rate_hz: 48_000,
                master_volume_scalar: 1.0,
                stereo_pan: 0.5,
                drift_compensation: true,
                acoustic_latency_ms: 2.5,
                injected_delay_samples_48k: 0,
            },
            AudioEndpoint {
                name: "soundcore Select 4 Go".to_string(),
                mac_or_uid: "00-7f-1d-7f-8d-35".to_string(),
                transport: AudioTransport::BluetoothA2dpAac,
                role: SpatialChannelRole::RoomSubAnchor,
                connected: false,
                planned_enabled: true,
                connection_status: ConnectionStatus::Unknown,
                sample_rate_hz: 48_000,
                master_volume_scalar: 1.0,
                stereo_pan: 0.5,
                drift_compensation: true,
                acoustic_latency_ms: 34.0,
                injected_delay_samples_48k: 0,
            },
            AudioEndpoint {
                name: "Tronsmart Bang SE".to_string(),
                mac_or_uid: "34-00-00-49-d4-49".to_string(),
                transport: AudioTransport::BluetoothA2dpAac,
                role: SpatialChannelRole::RoomSubAnchor,
                connected: false,
                planned_enabled: all_active,
                connection_status: ConnectionStatus::Unknown,
                sample_rate_hz: 48_000,
                master_volume_scalar: 1.0,
                stereo_pan: 0.5,
                drift_compensation: true,
                acoustic_latency_ms: 36.0,
                injected_delay_samples_48k: 0,
            },
            AudioEndpoint {
                name: "JBL GO 2".to_string(),
                mac_or_uid: "70-99-1c-55-cb-49".to_string(),
                transport: AudioTransport::BluetoothA2dpAac,
                role: SpatialChannelRole::CenterHalo,
                connected: false,
                planned_enabled: all_active,
                connection_status: ConnectionStatus::Unknown,
                sample_rate_hz: 48_000,
                master_volume_scalar: 1.0,
                stereo_pan: 0.35,
                drift_compensation: true,
                acoustic_latency_ms: 38.0,
                injected_delay_samples_48k: 0,
            },
            AudioEndpoint {
                name: "Mi Pocket Speaker 2".to_string(),
                mac_or_uid: "74-a3-4a-19-60-ca".to_string(),
                transport: AudioTransport::BluetoothA2dpAac,
                role: SpatialChannelRole::CenterHalo,
                connected: false,
                planned_enabled: all_active,
                connection_status: ConnectionStatus::Unknown,
                sample_rate_hz: 48_000,
                master_volume_scalar: 1.0,
                stereo_pan: 0.65,
                drift_compensation: true,
                acoustic_latency_ms: 38.0,
                injected_delay_samples_48k: 0,
            },
            AudioEndpoint {
                name: "JBL GO".to_string(),
                mac_or_uid: "78-44-05-86-d6-13".to_string(),
                transport: AudioTransport::BluetoothA2dpAac,
                role: SpatialChannelRole::CenterHalo,
                connected: false,
                planned_enabled: false,
                connection_status: ConnectionStatus::Unknown,
                sample_rate_hz: 48_000,
                master_volume_scalar: 1.0,
                stereo_pan: 0.5,
                drift_compensation: true,
                acoustic_latency_ms: 40.0,
                injected_delay_samples_48k: 0,
            },
            AudioEndpoint {
                name: "Aladdin 2".to_string(),
                mac_or_uid: "40-24-b2-4b-11-ce".to_string(),
                transport: AudioTransport::BluetoothA2dpAac,
                role: SpatialChannelRole::BinauralStereo,
                connected: false,
                planned_enabled: false,
                connection_status: ConnectionStatus::Unknown,
                sample_rate_hz: 48_000,
                master_volume_scalar: 1.0,
                stereo_pan: 0.5,
                drift_compensation: true,
                acoustic_latency_ms: 32.0,
                injected_delay_samples_48k: 0,
            },
            AudioEndpoint {
                name: "IA-230D".to_string(),
                mac_or_uid: "41-42-a2-03-b6-2c".to_string(),
                transport: AudioTransport::BluetoothA2dpAac,
                role: SpatialChannelRole::BinauralStereo,
                connected: false,
                planned_enabled: false,
                connection_status: ConnectionStatus::Unknown,
                sample_rate_hz: 48_000,
                master_volume_scalar: 1.0,
                stereo_pan: 0.5,
                drift_compensation: true,
                acoustic_latency_ms: 35.0,
                injected_delay_samples_48k: 0,
            },
            AudioEndpoint {
                name: "[LG] webOS TV OLED77G1PUA".to_string(),
                mac_or_uid: "dc-03-98-5e-e7-d5".to_string(),
                transport: AudioTransport::BluetoothBleAudio,
                role: SpatialChannelRole::BinauralStereo,
                connected: false,
                planned_enabled: false,
                connection_status: ConnectionStatus::Unknown,
                sample_rate_hz: 48_000,
                master_volume_scalar: 1.0,
                stereo_pan: 0.5,
                drift_compensation: true,
                acoustic_latency_ms: 42.0,
                injected_delay_samples_48k: 0,
            },
        ]
    }

    /// Refreshes observations; failures clear stale connectivity to unknown.
    pub fn refresh_live_bluetooth_status(endpoints: &mut [AudioEndpoint]) {
        let output = Command::new("system_profiler")
            .args(["SPBluetoothDataType"])
            .output();
        let text = output
            .ok()
            .filter(|o| o.status.success())
            .and_then(|o| String::from_utf8(o.stdout).ok());
        Self::apply_bluetooth_observation(endpoints, text.as_deref());
    }

    fn apply_bluetooth_observation(endpoints: &mut [AudioEndpoint], text: Option<&str>) {
        for ep in endpoints.iter_mut() {
            ep.connected = false;
            ep.connection_status = ConnectionStatus::Unknown;
        }
        let Some(text) = text else {
            return;
        };
        let mut section = ConnectionStatus::Unknown;
        let mut section_indent = 0;
        for line in text.lines() {
            let label = line.trim();
            let indent = line.len() - line.trim_start().len();
            match label {
                "Connected:" => {
                    section = ConnectionStatus::Connected;
                    section_indent = indent;
                    continue;
                }
                "Not Connected:" => {
                    section = ConnectionStatus::Disconnected;
                    section_indent = indent;
                    continue;
                }
                _ => {}
            }
            if !label.is_empty() && indent <= section_indent {
                section = ConnectionStatus::Unknown;
            }
            if section == ConnectionStatus::Unknown {
                continue;
            }
            for ep in endpoints.iter_mut().filter(|e| {
                matches!(
                    e.transport,
                    AudioTransport::BluetoothA2dpAac | AudioTransport::BluetoothBleAudio
                )
            }) {
                let short_name = ep.name.split(" - ").next().unwrap_or(&ep.name);
                if label
                    .strip_suffix(':')
                    .is_some_and(|name| name == ep.name || name == short_name)
                {
                    ep.connection_status = section;
                    ep.connected = section == ConnectionStatus::Connected;
                }
            }
        }
    }

    /// Computes sample-accurate delay lines so fast endpoints (`0.0 ms` Wired Jack) are delayed
    /// to match the slowest active Bluetooth A2DP endpoint (`24.0–38.0 ms`) at `48,000 Hz`.
    pub fn compute_delay_compensation(endpoints: &mut [AudioEndpoint]) -> Vec<DelayRingBuffer> {
        let max_latency_ms = endpoints
            .iter()
            .filter(|e| e.planned_enabled)
            .map(|e| e.acoustic_latency_ms)
            .fold(0.0f32, f32::max);

        let mut rings = Vec::with_capacity(endpoints.len());
        for ep in endpoints.iter_mut() {
            if ep.planned_enabled {
                let delta_ms = (max_latency_ms - ep.acoustic_latency_ms).max(0.0);
                let delay_samples = ((delta_ms * 48.0).round()) as usize;
                ep.injected_delay_samples_48k = delay_samples;
                rings.push(DelayRingBuffer::new(delay_samples));
            } else {
                ep.injected_delay_samples_48k = 0;
                rings.push(DelayRingBuffer::new(0));
            }
        }
        rings
    }

    /// Verifies CT-000 Epistemic Overlap (`admissibility::Structure`) for the Multi-Bluetooth routing policy.
    pub fn verify_ct000_admissibility(&self) -> Ct000RoutingProof {
        // 1. Verify the canonical keeper_instance admissible_actions witness
        let keeper_actions = admissible_actions(0.20, 0.20, 0.20);
        let keeper_canon_verified = keeper_actions.len() == 3;

        // 2. Construct the Multi-Bluetooth + CoreAudio epistemic overlap structure
        let obs = vec![
            "obs:split-ear-48k".to_string(),
            "obs:split-ear-48k".to_string(),
            "obs:multi-bt-wall-48k".to_string(),
            "obs:bt-hfp-trap-guard".to_string(),
        ];

        let mut safe_0 = HashSet::new();
        safe_0.insert("route:lock-48k-split-pan-and-ext-mic".to_string());

        let mut safe_1 = HashSet::new();
        safe_1.insert("route:lock-48k-split-pan-and-ext-mic".to_string());
        safe_1.insert("route:multi-bt-stacked-drift-comp-127".to_string());

        let mut safe_2 = HashSet::new();
        safe_2.insert("route:multi-bt-stacked-drift-comp-127".to_string());

        let mut safe_3 = HashSet::new();
        safe_3.insert("route:block-24khz-hfp-input-switch".to_string());

        let structure = Structure::new(obs, vec![safe_0, safe_1, safe_2, safe_3]);
        structure.assert_ct000();

        let theorem_holds = structure.no_safe_action_witness().is_none();
        let policy_vec = structure.guaranteeing_policy().unwrap_or_default();
        let mut policy = BTreeMap::new();
        for (o, a) in policy_vec {
            policy.insert(o, a);
        }

        Ct000RoutingProof {
            theorem_holds,
            keeper_canon_verified,
            total_states: structure.worlds(),
            total_observations: 3,
            guaranteeing_policy: policy,
            hfp_24khz_input_blocked: None,
            default_input_locked_to: None,
            evidence_scope: "static-policy-model-only".to_string(),
            planned_default_input: "External Microphone (BuiltInMicrophoneDevice @ 48,000 Hz)"
                .to_string(),
            nominal_sample_rate_hz: 48_000,
        }
    }

    /// Routes a stereo input frame `(in_l, in_r)` across all active endpoints with per-ear matrixing
    /// and sample-accurate delay compensation, returning the master `(left_ear, right_ear)` output.
    #[inline]
    pub fn process_spatial_frame(&mut self, in_l: f32, in_r: f32) -> (f32, f32) {
        let mut out_l = 0.0f32;
        let mut out_r = 0.0f32;
        let mut count_l = 0.0f32;
        let mut count_r = 0.0f32;

        for (idx, ep) in self.endpoints.iter().enumerate() {
            if !ep.planned_enabled {
                continue;
            }
            let (dl, dr) = self.delay_lines[idx].process_frame(in_l, in_r);
            match ep.role {
                SpatialChannelRole::LeftEarOnly => {
                    out_l += dl * ep.master_volume_scalar;
                    count_l += 1.0;
                }
                SpatialChannelRole::RightEarOnly => {
                    out_r += dr * ep.master_volume_scalar;
                    count_r += 1.0;
                }
                SpatialChannelRole::BinauralStereo => {
                    out_l += dl * ep.master_volume_scalar * (1.0 - ep.stereo_pan);
                    out_r += dr * ep.master_volume_scalar * ep.stereo_pan;
                    count_l += 0.5;
                    count_r += 0.5;
                }
                SpatialChannelRole::CenterHalo | SpatialChannelRole::RoomSubAnchor => {
                    let mid = 0.5 * (dl + dr) * ep.master_volume_scalar;
                    out_l += mid * 0.25;
                    out_r += mid * 0.25;
                    count_l += 0.25;
                    count_r += 0.25;
                }
            }
        }

        let final_l = if count_l > 0.0 {
            (out_l / count_l.max(1.0)).clamp(-1.0, 1.0)
        } else {
            in_l
        };
        let final_r = if count_r > 0.0 {
            (out_r / count_r.max(1.0)).clamp(-1.0, 1.0)
        } else {
            in_r
        };
        (final_l, final_r)
    }

    /// Invokes the live macOS CoreAudio split-ear & multi-Bluetooth hardware configurator if present.
    pub fn apply_live_macos_routing(&self) -> Result<String, String> {
        let script_path =
            "/Users/lodripeter/workspace/peterlodri-sec/scripts/m1-midi-split-audio.swift";
        self.apply_macos_routing_script(std::path::Path::new(script_path))
    }

    /// Executes the caller-selected bridge. Process success does not verify hardware state.
    pub fn apply_macos_routing_script(&self, path: &std::path::Path) -> Result<String, String> {
        Self::apply_routing_script(path)
    }

    fn apply_routing_script(script_path: &std::path::Path) -> Result<String, String> {
        if script_path.exists() {
            let output = Command::new("swift")
                .arg(script_path)
                .output()
                .map_err(|e| format!("Failed to execute CoreAudio Swift bridge: {e}"))?;
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            } else {
                Err(String::from_utf8_lossy(&output.stderr).to_string())
            }
        } else {
            Err("CoreAudio routing unavailable: bridge script is missing; host routing was not changed".to_string())
        }
    }

    /// Generates a comprehensive `MultiBluetoothReport`.
    pub fn report(&self) -> MultiBluetoothReport {
        let active_endpoints = self.endpoints.iter().filter(|e| e.connected).count();
        let max_bluetooth_latency_ms = self
            .endpoints
            .iter()
            .filter(|e| {
                e.planned_enabled
                    && matches!(
                        e.transport,
                        AudioTransport::BluetoothA2dpAac | AudioTransport::BluetoothBleAudio
                    )
            })
            .map(|e| e.acoustic_latency_ms)
            .fold(0.0f32, f32::max);

        MultiBluetoothReport {
            topology: self.topology.as_str().to_string(),
            aggregate_device_name: "M1 MIDI: Left Ear Jack + Right Ear AirPods (8b-is Multi-BT)"
                .to_string(),
            aggregate_uid: "com.8b-is.m1-midi-left-jack-right-airpods".to_string(),
            active_endpoints,
            planned_endpoints: self.endpoints.iter().filter(|e| e.planned_enabled).count(),
            total_known_endpoints: self.endpoints.len(),
            max_bluetooth_latency_ms,
            endpoints: self.endpoints.clone(),
            ct000_proof: self.verify_ct000_admissibility(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn observations_replace_stale_connections_without_changing_the_plan() {
        let mut endpoints =
            MultiBluetoothRouter::fleet_endpoints(MultiBluetoothTopology::FullConstellationWall);
        let plan: Vec<_> = endpoints.iter().map(|e| e.planned_enabled).collect();
        MultiBluetoothRouter::apply_bluetooth_observation(&mut endpoints, Some(
            "Bluetooth:\n    Connected:\n        JBL GO 2:\n    Not Connected:\n        JBL GO:\n"));
        assert!(
            endpoints
                .iter()
                .find(|e| e.name == "JBL GO 2")
                .unwrap()
                .connected
        );
        assert_eq!(
            endpoints
                .iter()
                .find(|e| e.name == "JBL GO")
                .unwrap()
                .connection_status,
            ConnectionStatus::Disconnected
        );
        assert_eq!(endpoints[0].connection_status, ConnectionStatus::Unknown);
        MultiBluetoothRouter::apply_bluetooth_observation(
            &mut endpoints,
            Some("Not Connected:\n    JBL GO 2:\n"),
        );
        assert!(
            !endpoints
                .iter()
                .find(|e| e.name == "JBL GO 2")
                .unwrap()
                .connected
        );
        assert_eq!(
            plan,
            endpoints
                .iter()
                .map(|e| e.planned_enabled)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn failed_empty_or_unrecognized_discovery_is_unknown() {
        let mut endpoints =
            MultiBluetoothRouter::fleet_endpoints(MultiBluetoothTopology::FullConstellationWall);
        for input in [
            None,
            Some(""),
            Some("malformed"),
            Some("Connected:\n    JBL GO 2 extra:\n"),
            Some("    Connected:\nOther controller:\n        JBL GO 2:\n"),
        ] {
            for endpoint in &mut endpoints {
                endpoint.connected = true;
                endpoint.connection_status = ConnectionStatus::Connected;
            }
            MultiBluetoothRouter::apply_bluetooth_observation(&mut endpoints, input);
            assert!(endpoints
                .iter()
                .all(|e| !e.connected && e.connection_status == ConnectionStatus::Unknown));
        }
    }

    #[test]
    fn missing_routing_bridge_is_unavailable() {
        let missing = std::env::temp_dir().join(format!(
            "8b-is-music-missing-bridge-{}-{}.swift",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        assert!(!missing.exists());
        let result = MultiBluetoothRouter::apply_routing_script(&missing);
        assert!(
            result.is_err(),
            "Missing bridge must not report routing success"
        );
    }

    #[test]
    fn multi_bluetooth_router_proves_ct000_and_compensates_latency() {
        let mut router =
            MultiBluetoothRouter::planned(MultiBluetoothTopology::FullConstellationWall);
        let report = router.report();
        assert!(report.ct000_proof.theorem_holds);
        assert!(report.ct000_proof.keeper_canon_verified);
        assert!(report.ct000_proof.hfp_24khz_input_blocked.is_none());
        assert!(report.ct000_proof.default_input_locked_to.is_none());
        assert_eq!(
            report.ct000_proof.evidence_scope,
            "static-policy-model-only"
        );
        let json = serde_json::to_value(&report).unwrap();
        assert!(json["ct000_proof"]["hfp_24khz_input_blocked"].is_null());
        assert!(json["ct000_proof"]["default_input_locked_to"].is_null());
        assert!(report.planned_endpoints >= 4);
        assert_eq!(report.max_bluetooth_latency_ms, 38.0);
        assert_eq!(report.active_endpoints, 0);
        assert!(
            report
                .endpoints
                .iter()
                .filter(|e| e.planned_enabled)
                .count()
                >= 4
        );

        let jack = report
            .endpoints
            .iter()
            .find(|e| e.transport == AudioTransport::Wired35mmJack)
            .expect("Wired 3.5mm jack present");
        assert!(jack.injected_delay_samples_48k > 0);

        let (l, r) = router.process_spatial_frame(0.8, -0.6);
        assert!(l.is_finite() && r.is_finite());
    }
}
