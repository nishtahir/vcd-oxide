use std::str::FromStr;

use serde::{Deserialize, Serialize};
use vcd_oxide_parser::{ValueChangeDump, ValueChangeDumpWave};

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WaveJson {
    pub signal: Vec<WaveJsonSignal>,
    #[serde(skip)]
    pub head: Option<Head>,
    #[serde(skip)]
    pub foot: Option<Foot>,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WaveJsonSignal {
    pub name: Option<String>,
    pub wave: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Head {}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
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
            let wave = vcd_wave_to_wavejson_signal(sig, raw_wave, max_value_change_len);
            wavejson_signals.push(wave);
        }

        WaveJson {
            signal: wavejson_signals,
            head: None,
            foot: None,
        }
    }
}

fn vcd_wave_to_wavejson_signal(
    sig: vcd_oxide_parser::ValueChangeDumpSignal,
    wave: &ValueChangeDumpWave,
    max_value_change_len: usize,
) -> WaveJsonSignal {
    let mut result = String::from_str("").unwrap();
    let mut signal_iter = wave.value_changes.iter().peekable();
    let mut data = vec![];
    while let Some(value_change) = signal_iter.next() {
        let mut repeat = 0;
        if let Some(next) = signal_iter.peek() {
            repeat = next.time - value_change.time;
        }

        let value = value_change.value.as_str();
        match value {
            "0" => {
                result += "l";
            }
            "1" => {
                result += "h";
            }
            "x" => {
                result += "x";
            }
            "z" => {
                result += "z";
            }
            _ => {
                result += "=";
                data.push(value.to_owned());
            }
        };
        if repeat > 1 {
            result += &".".repeat(repeat - 1);
        }
    }

    let wave = format!("{:.<width$}", result, width = max_value_change_len);
    WaveJsonSignal {
        name: Some(sig.reference),
        wave: Some(wave),
        data: Some(data),
    }
}

impl WaveJson {
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}
