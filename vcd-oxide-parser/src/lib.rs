extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser, Debug)]
#[grammar = "grammar/vcd.pest"]
pub struct ValueChangeDumpFile;

#[cfg(test)]
mod test {
    use super::*;
    use insta::assert_debug_snapshot;

    #[test]
    fn test_vcd_declaration_command() {
        let declatations = include_str!("../test/declaration_command.vcd.test");
        let file = ValueChangeDumpFile::parse(Rule::value_change_dump_definitions, declatations);
        assert_debug_snapshot!(file.unwrap())
    }

    
}
