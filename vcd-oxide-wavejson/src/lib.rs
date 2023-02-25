use serde::{ser::SerializeSeq, Deserialize, Serialize};
use vcd_oxide_parser::{ValueChangeDump, ValueChangeDumpWave};

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WaveJson {
    pub signal: Vec<WaveJsonSignalItem>,
    #[serde(skip)]
    pub head: Option<Head>,
    #[serde(skip)]
    pub foot: Option<Foot>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WaveJsonSignalItem {
    Signal(WaveJsonSignal),
    Group(WaveJsonGroup),
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
pub struct WaveJsonGroup {
    pub name: Option<String>,
    pub signals: Vec<WaveJsonSignal>,
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

impl WaveJson {
    pub fn from_vcd(vcd: ValueChangeDump, expand_busses: bool) -> Self {
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
            if sig.size > 1 && expand_busses {
                let group = vcd_wave_to_wavejson_group(sig, raw_wave, max_value_change_len);
                wavejson_signals.push(WaveJsonSignalItem::Group(group));
            } else {
                let wave =
                    vcd_wave_to_wavejson_signal(sig, raw_wave, max_value_change_len, expand_busses);
                wavejson_signals.push(WaveJsonSignalItem::Signal(wave));
            };
        }

        WaveJson {
            signal: wavejson_signals,
            head: None,
            foot: None,
        }
    }
}

fn vcd_wave_to_wavejson_group(
    sig: vcd_oxide_parser::ValueChangeDumpSignal,
    wave: &ValueChangeDumpWave,
    max_value_change_len: usize,
) -> WaveJsonGroup {
    let mut signals = vec![];
    for i in 0..sig.size {
        let mut result = "".to_owned();
        let mut signal_iter = wave.value_changes.iter().peekable();
        while let Some(value_change) = signal_iter.next() {
            let mut repeat = 0;
            if let Some(next) = signal_iter.peek() {
                repeat = next.time - value_change.time;
            }
            let mut value = value_change.value.as_str();
            // remove the "b" or "B" prefix
            if value.starts_with("b") || value.starts_with("B") {
                value = &value[1..];
            }

            // pad the value with leading zeros to the signal size
            let value = &format!("{:0>width$}", value, width = sig.size)[i..i + 1];
            map_signal_value_to_wavejson_value(value, repeat, &mut result);
        }
        let wave = format!("{:.<width$}", result, width = max_value_change_len);
        signals.push(WaveJsonSignal {
            name: Some(format!("{}[{}]", sig.reference, i)),
            wave: Some(wave),
            ..Default::default()
        });
    }
    WaveJsonGroup {
        name: Some(sig.reference),
        signals,
    }
}

fn vcd_wave_to_wavejson_signal(
    sig: vcd_oxide_parser::ValueChangeDumpSignal,
    wave: &ValueChangeDumpWave,
    max_value_change_len: usize,
    expand_busses: bool,
) -> WaveJsonSignal {
    let mut result = "".to_owned();
    let mut signal_iter = wave.value_changes.iter().peekable();
    let mut data = vec![];
    while let Some(value_change) = signal_iter.next() {
        let mut repeat = 0;
        if let Some(next) = signal_iter.peek() {
            repeat = next.time - value_change.time;
        }

        let value = value_change.value.as_str();
        if sig.size > 1 && !expand_busses {
            data.push(value.to_owned());
        }

        map_signal_value_to_wavejson_value(value, repeat, &mut result);
    }

    let wave = format!("{:.<width$}", result, width = max_value_change_len);
    WaveJsonSignal {
        name: Some(sig.reference),
        wave: Some(wave),
        data: Some(data),
    }
}

fn map_signal_value_to_wavejson_value(value: &str, repeat: usize, result: &mut String) {
    match value {
        "0" => {
            *result += "l";
        }
        "1" => {
            *result += "h";
        }
        "x" => {
            *result += "x";
        }
        "z" => {
            *result += "z";
        }
        _ => {
            *result += "=";
        }
    }

    if repeat > 1 {
        *result += &".".repeat(repeat - 1);
    }
}

impl WaveJson {
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}

// TODO - deserialize the group correctly
impl Serialize for WaveJsonGroup {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.signals.len() + 1))?;
        seq.serialize_element(&self.name)?;
        for e in &self.signals {
            seq.serialize_element(e)?;
        }
        seq.end()
    }
}
