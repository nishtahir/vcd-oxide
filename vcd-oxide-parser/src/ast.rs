#[derive(Debug)]
pub struct ValueChangeDumpDefinition {
    pub declaration_commands: Vec<DeclarationCommand>,
    pub simulation_commands: Vec<SimulationCommand>
}

#[derive(Debug)]
pub enum DeclarationType {
    Comment,
    Date,
    EndDefinitions,
    Scope,
    Timescale,
    Upscope,
    Var,
    Version
}

#[derive(Debug)]
pub struct DeclarationCommand {
    pub ty: DeclarationType,
    pub value: String
}

#[derive(Debug)]
pub enum SimulationCommand {
    KeywordCommand(SimulationKeywordCommand),
    Comment(SimulationComment),
    SimulationTime,
    ValueChange(SimulationValueChange)
}

#[derive(Debug)]
pub struct SimulationKeywordCommand {
    pub ty: SimulationType,
    pub value_changes: Vec<SimulationValueChange>
}

#[derive(Debug)]
pub enum SimulationType {
    DumpAll,
    DumpOff,
    DumpOn,
    DumpVars
}

#[derive(Debug)]
pub struct SimulationComment {
    pub value: String
}

#[derive(Debug)]
pub enum SimulationValueChange  {
    Scalar(ScalarValueChange),
    Vector(VectorValueChange)
}

#[derive(Debug)]
pub struct ScalarValueChange {
    pub value: String,
    pub identifier_code: String
}

#[derive(Debug)]
pub struct VectorValueChange {

}
