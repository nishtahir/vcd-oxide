#[derive(Debug)]
pub struct ValueChangeDumpDefinition {
    pub declaration_commands: Vec<DeclarationCommand>,
    pub simulation_commands: Vec<SimulationCommand>,
}

#[derive(Debug)]
pub enum DeclarationCommand {
    Comment(GenericComment),
    Date(DeclarationDate),
    EndDefinitions,
    Scope(DeclarationScope),
    Timescale(DeclarationTimescale),
    Upscope,
    Var(DeclarationVar),
    Version(DeclarationVersion),
}

#[derive(Debug)]
pub struct DeclarationDate {
    pub value: String,
}

#[derive(Debug)]
pub struct DeclarationVersion {
    pub value: String,
}

#[derive(Debug)]
pub struct DeclarationTimescale {
    pub time_number: usize,
    pub time_unit: String,
}

#[derive(Debug)]
pub struct DeclarationScope {
    pub scope_type: String,
    pub scope_identifier: String,
}

#[derive(Debug)]
pub struct DeclarationVar {
    pub var_type: String,
    pub size: usize,
    pub identifier_code: String,
    pub reference: String,
}

#[derive(Debug)]
pub enum SimulationCommand {
    KeywordCommand(SimulationKeywordCommand),
    Comment(GenericComment),
    SimulationTime,
    ValueChange(SimulationValueChange),
}

#[derive(Debug)]
pub struct SimulationKeywordCommand {
    pub ty: SimulationType,
    pub value_changes: Vec<SimulationValueChange>,
}

#[derive(Debug)]
pub enum SimulationType {
    DumpAll,
    DumpOff,
    DumpOn,
    DumpVars,
}

#[derive(Debug)]
pub enum SimulationValueChange {
    Scalar(ScalarValueChange),
    Vector(VectorValueChange),
}

#[derive(Debug)]
pub struct ScalarValueChange {
    pub value: String,
    pub identifier_code: String,
}

#[derive(Debug)]
pub enum VectorValueChange {
    Binary(BinaryVectorValueChange),
    Real(RealVectorValueChange),
}

#[derive(Debug)]
pub struct BinaryVectorValueChange {
    pub value: String,
    pub identifier_code: String,
}

#[derive(Debug)]
pub struct RealVectorValueChange {
    pub value: String,
    pub identifier_code: String,
}

#[derive(Debug)]
pub struct GenericComment {
    pub value: String,
}