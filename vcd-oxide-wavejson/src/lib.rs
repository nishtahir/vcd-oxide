use std::{os::macos::raw, str::FromStr};

use serde::{Deserialize, Serialize};
use vcd_oxide_parser::{ValueChange, ValueChangeDump, ValueChangeDumpSignal, ValueChangeDumpWave};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WaveJson {
    pub signal: Vec<WaveJsonSignal>,
    #[serde(skip)]
    pub head: Option<Head>,
    #[serde(skip)]
    pub foot: Option<Foot>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WaveJsonSignal {
    pub name: Option<String>,
    pub wave: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Head {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Foot {}

impl From<ValueChangeDump> for WaveJson {
    fn from(vcd: ValueChangeDump) -> Self {
        let mut wavejson_signals = vec![];
        let vcd_signals = vcd.signals();
        let max_value_change_len = vcd
            .wave_map
            .values()
            .map(|sig| sig.value_changes.len())
            .max()
            .unwrap_or(0);

        for sig in vcd_signals {
            let raw_wave = vcd.wave_map.get(&sig.identifier).unwrap();
            wavejson_signals.push(WaveJsonSignal {
                name: Some(sig.reference),
                wave: Some(vcd_wave_to_string(raw_wave, max_value_change_len)),
                data: None,
            });
        }

        WaveJson {
            signal: wavejson_signals,
            head: None,
            foot: None,
        }
    }
}

fn vcd_wave_to_string(sig: &ValueChangeDumpWave, max_len: usize) -> String {
    let mut result = String::from_str("").unwrap();
    let mut signal_iter = sig.value_changes.iter().peekable();
    while let Some(value_change) = signal_iter.next() {
        let mut repeat = 0;
        if let Some(next) = signal_iter.peek() {
            repeat = next.time - value_change.time;
        }

        match value_change.value.as_str() {
            "0" | "b0" => {
                result += &"l";
            }
            "1" | "b1" => {
                result += &"h";
            }
            _ => unimplemented!("{}", value_change.value),
        };
        if repeat > 1 {
            result += &".".repeat(repeat - 1);
        }
    }

    format!("{:.<width$}", result, width = max_len)
}

impl WaveJson {
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}
