mod ast;

extern crate pest;
extern crate pest_derive;

use crate::ast::{
    DeclarationCommand, DeclarationType, ScalarValueChange, SimulationCommand, SimulationComment,
    SimulationKeywordCommand, SimulationType, SimulationValueChange, ValueChangeDumpDefinition,
    VectorValueChange,RealVectorValueChange,BinaryVectorValueChange
};
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser, Debug)]
#[grammar = "grammar/vcd.pest"]
pub struct ValueChangeDumpParser;

fn visit_value_change_dump_definitions(rule: Pair<Rule>) -> ValueChangeDumpDefinition {
    let mut declaration_commands = vec![];
    let mut simulation_commands = vec![];

    for item in rule.into_inner() {
        match item.as_rule() {
            Rule::declaration_command => {
                let declaration_command = visit_declaration_command(item);
                declaration_commands.push(declaration_command);
            }
            Rule::simulation_command => {
                let simulation_command = visit_simulation_command(item);
                simulation_commands.push(simulation_command);
            }
            _ => unreachable!("{:#?}", item),
        };
    }

    ValueChangeDumpDefinition {
        declaration_commands,
        simulation_commands,
    }
}

fn visit_simulation_command(rule: Pair<Rule>) -> SimulationCommand {
    let inner = rule.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::simulation_keyword_command => visit_simulation_keyword_command(inner),
        Rule::simulation_keyword_comment => visit_simulation_keyword_comment(inner),
        Rule::simulation_time => SimulationCommand::SimulationTime,
        Rule::value_change => SimulationCommand::ValueChange(visit_value_change(inner)),
        _ => unreachable!("{:#?}", inner),
    }
}

fn visit_simulation_keyword_command(rule: Pair<Rule>) -> SimulationCommand {
    let mut inner = rule.into_inner();
    let ty = visit_simulation_keyword(inner.next().unwrap());
    let mut value_changes = vec![];
    for i in inner {
        value_changes.push(visit_value_change(i));
    }

    SimulationCommand::KeywordCommand(SimulationKeywordCommand { ty, value_changes })
}

fn visit_simulation_keyword(rule: Pair<Rule>) -> SimulationType {
    let keyword = rule.as_str();
    match keyword {
        "$dumpall" => SimulationType::DumpAll,
        "$dumpoff" => SimulationType::DumpOff,
        "$dumpon" => SimulationType::DumpOn,
        "$dumpvars" => SimulationType::DumpVars,
        _ => unreachable!("{:#?}", rule),
    }
}

fn visit_simulation_keyword_comment(rule: Pair<Rule>) -> SimulationCommand {
    SimulationCommand::Comment(SimulationComment {
        value: rule.as_str().to_owned(),
    })
}

fn visit_value_change(rule: Pair<Rule>) -> SimulationValueChange {
    let inner = rule.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::scalar_value_change => visit_scalar_value_change(inner),
        Rule::vector_value_change => visit_vector_value_change(inner),
        _ => unreachable!("{:#?}", inner),
    }
}

fn visit_scalar_value_change(rule: Pair<Rule>) -> SimulationValueChange {
    let mut inner = rule.into_inner();
    let value = inner.next().unwrap();
    let identifier_code = inner.next().unwrap();
    SimulationValueChange::Scalar(ScalarValueChange {
        value: value.as_str().to_owned(),
        identifier_code: identifier_code.as_str().to_owned(),
    })
}

fn visit_vector_value_change(rule: Pair<Rule>) -> SimulationValueChange {
    let mut inner = rule.into_inner();
    let value = inner.next().unwrap();
    let identifier_code = inner.next().unwrap();
    SimulationValueChange::Vector(match value.as_rule() {
        Rule::real_value => VectorValueChange::Real(RealVectorValueChange {
            value: value.as_str().to_owned(),
            identifier_code: identifier_code.as_str().to_owned(),
        }),
        Rule::binary_value => VectorValueChange::Binary(BinaryVectorValueChange {
            value: value.as_str().to_owned(),
            identifier_code: identifier_code.as_str().to_owned(),
        }),
        _ => unreachable!("{:#?}", inner),
    })
}

fn visit_declaration_command(rule: Pair<Rule>) -> DeclarationCommand {
    let mut inner = rule.clone().into_inner();
    let declaration_keyword = inner.next().unwrap();
    let declaration_keyword = visit_declaration_keyword(declaration_keyword);
    let command_text = inner.next();
    let command_text = visit_command_text(command_text);

    DeclarationCommand {
        ty: declaration_keyword,
        value: command_text,
    }
}

fn visit_declaration_keyword(rule: Pair<Rule>) -> DeclarationType {
    let keyword = rule.as_str();
    match keyword {
        "$comment" => DeclarationType::Comment,
        "$date" => DeclarationType::Date,
        "$enddefinitions" => DeclarationType::EndDefinitions,
        "$scope" => DeclarationType::Scope,
        "$timescale" => DeclarationType::Timescale,
        "$upscope" => DeclarationType::Upscope,
        "$var" => DeclarationType::Var,
        "$version" => DeclarationType::Version,
        _ => unreachable!("{:#?}", rule),
    }
}

fn visit_command_text(rule: Option<Pair<Rule>>) -> String {
    return rule.map_or("", |r| r.as_str()).to_owned();
}

fn parse(input: &str) -> ValueChangeDumpDefinition {
    let mut root = ValueChangeDumpParser::parse(Rule::file, input).unwrap();
    let inner = root.next().unwrap();
    match inner.as_rule() {
        Rule::value_change_dump_definitions => visit_value_change_dump_definitions(inner),
        _ => unreachable!("{:#?}", inner),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use insta::assert_debug_snapshot;

    #[test]
    fn test_vcd_declaration_command() {
        let declerations = include_str!("../test/declaration_command.vcd.test");
        let ast = parse(declerations);
        assert_debug_snapshot!(ast)
    }

    #[test]
    fn test_vcd_file() {
        let declerations = include_str!("../test/UartRxTest.vcd");
        let ast = parse(declerations);
        assert_debug_snapshot!(ast)
    }

    #[test]
    #[ignore = "Runs for > 350s. Requires parser rewrite to be reasonable"]
    fn test_large_vcd_file() {
        let declerations = include_str!("../test/NextCoreTest.vcd");
        let ast = parse(declerations);
        assert_debug_snapshot!(ast)
    }
}
