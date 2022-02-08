#[derive(Debug, PartialEq, DbEnum)]
#[DieselType = "Part_kind"]
pub enum PartKind {
    Resistor,
    Capacitor,
    Transistor,
    Diode,
    Potentiometer,
    Switch,
}
