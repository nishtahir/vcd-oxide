mod types;

use crate::types::WaveJson;

#[cfg(test)]
mod test {

    use insta::assert_debug_snapshot;

    use super::*;

    #[test]
    fn test_deserialize_sample_wavejson() {
        let sample = include_str!("../test/res/simple.json");
        let wave: WaveJson = serde_json::from_str(&sample).unwrap();
        assert_debug_snapshot!(wave);
    }
}
