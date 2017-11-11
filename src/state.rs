use self::State::*;

/// Enum for states and territories.
#[derive(Clone, Copy)]
pub enum State {
    NSW,
    VIC,
    QLD,
    SA,
    WA,
    TAS,
    NT,
    ACT
}

impl State {
    pub fn from_str(state: &str) -> Result<State, ()> {
        match &state.to_uppercase()[..] {
            "NSW" => Ok(NSW),
            "VIC" => Ok(VIC),
            "QLD" => Ok(QLD),
            "SA" => Ok(SA),
            "WA" => Ok(WA),
            "TAS" => Ok(TAS),
            "NT" => Ok(NT),
            "ACT" => Ok(ACT),
            _ => Err(()),
        }
    }

    pub fn to_str(&self) -> &'static str {
        match *self {
            NSW => "NSW",
            VIC => "VIC",
            QLD => "QLD",
            SA => "SA",
            WA => "WA",
            TAS => "TAS",
            NT => "NT",
            ACT => "ACT",
        }
    }

    pub fn num_senators(&self) -> usize {
        match *self {
            ACT | NT => 6,
            _ => 12,
        }
    }
}
