mod types;

use types::WaveJsonSignal;
use vcd_oxide_core::{Signal, EdgeDirection::{Positive, Negative}};
use crate::types::WaveJson;

impl From<Signal> for WaveJsonSignal {
    fn from(signal: Signal) -> Self {
        let mut stringified_wave = String::new();
        if let Some(edge_direction) = signal.edge_direction {
            match edge_direction {
                Positive => stringified_wave += "p",
                Negative => stringified_wave += "n",
            }
        }

        WaveJsonSignal {
            name: Some(signal.name),
            wave: Some(stringified_wave.to_owned()),
            data: None,
        }
    }
}

#[cfg(test)]
mod test {

    use insta::{assert_debug_snapshot, assert_json_snapshot};
    use vcd_oxide_core::{EdgeDirection, Signal};

    use super::*;

    #[test]
    fn test_serialize_empty_signal() {
        let wave_json = WaveJson {
            signal: vec![],
            head: None,
            foot: None,
        };

        assert_json_snapshot!(wave_json);
    }

    #[test]
    fn test_deserialize_sample_wavejson() {
        let sample = include_str!("../test/res/simple.json");
        let wave: WaveJson = serde_json::from_str(&sample).unwrap();
        assert_debug_snapshot!(wave);
    }

    #[test]
    fn test_serialize_positive_edge_wave() {
        let sig = Signal {
            name: "test".to_owned(),
            edge_direction: Some(EdgeDirection::Positive),
            states: vec![],
        };

        let wave_json = WaveJson {
            signal: vec![sig.into()],
            head: None,
            foot: None,
        };

        assert_json_snapshot!(wave_json);
    }

}
