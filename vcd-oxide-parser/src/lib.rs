mod ast;
mod model;

extern crate pest;
extern crate pest_derive;

use crate::ast::*;
use crate::model::*;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};

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
        Rule::vcd_simulation_dumpall => visit_vcd_simulation_dumpall(inner),
        Rule::vcd_simulation_dumpoff => visit_vcd_simulation_dumpoff(inner),
        Rule::vcd_simulation_dumpon => visit_vcd_simulation_dumpon(inner),
        Rule::vcd_simulation_dumpvars => visit_vcd_simulation_dumpvars(inner),
        Rule::simulation_keyword_comment => visit_simulation_keyword_comment(inner),
        Rule::simulation_time => visit_simulation_time(inner),
        Rule::value_change => SimulationCommand::ValueChange(visit_value_change(inner)),
        _ => unreachable!("{:#?}", inner),
    }
}

fn visit_vcd_simulation_dumpall(rule: Pair<Rule>) -> SimulationCommand {
    SimulationCommand::KeywordCommand(SimulationKeywordCommand {
        ty: SimulationType::DumpAll,
        value_changes: visit_value_change_list(rule),
    })
}

fn visit_vcd_simulation_dumpoff(rule: Pair<Rule>) -> SimulationCommand {
    SimulationCommand::KeywordCommand(SimulationKeywordCommand {
        ty: SimulationType::DumpOff,
        value_changes: visit_value_change_list(rule),
    })
}

fn visit_vcd_simulation_dumpon(rule: Pair<Rule>) -> SimulationCommand {
    SimulationCommand::KeywordCommand(SimulationKeywordCommand {
        ty: SimulationType::DumpOn,
        value_changes: visit_value_change_list(rule),
    })
}

fn visit_vcd_simulation_dumpvars(rule: Pair<Rule>) -> SimulationCommand {
    SimulationCommand::KeywordCommand(SimulationKeywordCommand {
        ty: SimulationType::DumpVars,
        value_changes: visit_value_change_list(rule),
    })
}

fn visit_simulation_keyword_comment(rule: Pair<Rule>) -> SimulationCommand {
    SimulationCommand::Comment(GenericComment {
        value: rule.as_str().to_owned(),
    })
}

fn visit_simulation_time(rule: Pair<Rule>) -> SimulationCommand {
    let value = rule
        .into_inner()
        .next()
        .unwrap()
        .as_str()
        .parse::<usize>()
        .unwrap();
    SimulationCommand::SimulationTime(SimulationTime { value })
}

fn visit_value_change_list(rule: Pair<Rule>) -> Vec<SimulationValueChange> {
    let mut inner = rule.into_inner();
    let mut value_changes = vec![];
    for i in inner {
        value_changes.push(visit_value_change(i));
    }
    value_changes
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
    let inner = rule.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::vcd_declaration_vars => DeclarationCommand::Var(visit_vcd_declaration_vars(inner)),
        Rule::vcd_declaration_comment => {
            DeclarationCommand::Comment(visit_vcd_declaration_comment(inner))
        }
        Rule::vcd_declaration_date => DeclarationCommand::Date(visit_vcd_declaration_date(inner)),
        Rule::vcd_declaration_enddefinitions => DeclarationCommand::EndDefinitions,
        Rule::vcd_declaration_scope => {
            DeclarationCommand::Scope(visit_vcd_declaration_scope(inner))
        }
        Rule::vcd_declaration_timescale => {
            DeclarationCommand::Timescale(visit_vcd_declaration_timescale(inner))
        }
        Rule::vcd_declaration_upscope => DeclarationCommand::Upscope,
        Rule::vcd_declaration_version => {
            DeclarationCommand::Version(visit_vcd_declaration_version(inner))
        }
        _ => unreachable!("{:#?}", inner),
    }
}

fn visit_vcd_declaration_date(rule: Pair<Rule>) -> DeclarationDate {
    let inner = rule.into_inner().next().unwrap();
    DeclarationDate {
        value: inner.as_str().to_owned(),
    }
}

fn visit_vcd_declaration_version(rule: Pair<Rule>) -> DeclarationVersion {
    let inner = rule.into_inner().next().unwrap();
    DeclarationVersion {
        value: inner.as_str().to_owned(),
    }
}

fn visit_vcd_declaration_timescale(rule: Pair<Rule>) -> DeclarationTimescale {
    let mut inner = rule.into_inner();
    let time_number = inner.next().unwrap().as_str().parse::<usize>().unwrap();
    let time_unit = inner.next().unwrap().as_str().to_owned();

    DeclarationTimescale {
        time_number,
        time_unit,
    }
}

fn visit_vcd_declaration_scope(rule: Pair<Rule>) -> DeclarationScope {
    let mut inner = rule.into_inner();
    let scope_type = inner.next().unwrap().as_str().to_owned();
    let scope_identifier = inner.next().unwrap().as_str().to_owned();

    DeclarationScope {
        scope_type,
        scope_identifier,
    }
}

fn visit_vcd_declaration_vars(rule: Pair<Rule>) -> DeclarationVar {
    let mut inner = rule.into_inner();
    let var_type = inner.next().unwrap().as_str().to_owned();
    let size = inner.next().unwrap().as_str().parse::<usize>().unwrap();
    let identifier_code = inner.next().unwrap().as_str().to_owned();
    let reference = inner.next().unwrap().as_str().to_owned();

    DeclarationVar {
        var_type,
        size,
        identifier_code,
        reference,
    }
}

fn visit_vcd_declaration_comment(rule: Pair<Rule>) -> GenericComment {
    let inner = rule.into_inner().next().unwrap();
    GenericComment {
        value: inner.as_str().to_owned(),
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

impl ValueChangeDump {
    pub fn fromDefinition(definition: ValueChangeDumpDefinition) -> Self {
        let mut dump = ValueChangeDump::default();
        let mut active_scope = dump.root_scope.clone();
        for declaration in definition.declaration_commands {
            match declaration {
                DeclarationCommand::Comment(_) => {
                    // Ignore
                }
                DeclarationCommand::EndDefinitions => {
                    // Ignore
                }
                DeclarationCommand::Date(date) => dump.date = date.value,
                DeclarationCommand::Timescale(timescale) => {
                    dump.timescale = format!("{}{}", timescale.time_number, timescale.time_unit)
                }
                DeclarationCommand::Scope(scope) => {
                    let scope = Rc::<RefCell<ValueChangeDumpScope>>::new(RefCell::new(
                        ValueChangeDumpScope {
                            name: scope.scope_identifier,
                            parent: Some(Rc::<RefCell<ValueChangeDumpScope>>::downgrade(
                                &active_scope,
                            )),
                            ..Default::default()
                        },
                    ));
                    active_scope.borrow_mut().scopes.push(scope.clone());
                    active_scope = scope;
                }
                DeclarationCommand::Upscope => {
                    active_scope = active_scope
                        .clone()
                        .borrow()
                        .parent
                        .as_ref()
                        .unwrap()
                        .upgrade()
                        .unwrap();
                }
                DeclarationCommand::Var(var) => {
                    let signal_id = var.identifier_code;
                    active_scope
                        .borrow_mut()
                        .signals
                        .push(ValueChangeDumpSignal {
                            kind: var.var_type.to_owned(),
                            identifier: signal_id.to_owned(),
                            reference: var.reference.to_owned(),
                            size: var.size,
                            ..Default::default()
                        });

                    dump.wave_map
                        .insert(signal_id.to_owned(), ValueChangeDumpWave::default());
                }
                DeclarationCommand::Version(version) => dump.version = version.value,
            }
        }

        let mut simulation_time = 0;
        for sim in definition.simulation_commands {
            match sim {
                SimulationCommand::KeywordCommand(command) => {
                    for elem in &command.value_changes {
                        dump.wave_map
                            .get_mut(&elem.identifier_code())
                            .unwrap()
                            .value_changes
                            .push(ValueChange {
                                value: elem.value(),
                                time: simulation_time,
                            });
                    }
                }
                SimulationCommand::Comment(_) => {
                    // Ignore
                }
                SimulationCommand::SimulationTime(time) => simulation_time = time.value,
                SimulationCommand::ValueChange(value_change) => {
                    dump.wave_map
                        .get_mut(&value_change.identifier_code())
                        .unwrap()
                        .value_changes
                        .push(ValueChange {
                            value: value_change.value(),
                            time: simulation_time,
                        });
                }
            }
        }

        dump
    }

    fn create_value_change(time: usize, svc: SimulationValueChange) -> ValueChange {
        match svc {
            SimulationValueChange::Scalar(s) => ValueChange {
                time,
                value: s.value,
            },
            SimulationValueChange::Vector(v) => match v {
                VectorValueChange::Binary(b) => ValueChange {
                    time,
                    value: b.value,
                },
                VectorValueChange::Real(r) => ValueChange {
                    time,
                    value: r.value,
                },
            },
        }
    }
}

impl SimulationValueChange {
    fn identifier_code(&self) -> String {
        let s = match self {
            SimulationValueChange::Scalar(s) => &s.identifier_code,
            SimulationValueChange::Vector(v) => match v {
                VectorValueChange::Binary(b) => &b.identifier_code,
                VectorValueChange::Real(r) => &r.identifier_code,
            },
        };
        s.to_owned()
    }

    fn value(&self) -> String {
        let s = match self {
            SimulationValueChange::Scalar(s) => &s.value,
            SimulationValueChange::Vector(v) => match v {
                VectorValueChange::Binary(b) => &b.value,
                VectorValueChange::Real(r) => &r.value,
            },
        };
        s.to_owned()
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

    #[test]
    fn test_model_against_uart() {
        let declerations = include_str!("../test/UartRxTest.vcd");
        let ast = parse(declerations);
        let model = ValueChangeDump::fromDefinition(ast);
        assert_debug_snapshot!(model)
    }
}
