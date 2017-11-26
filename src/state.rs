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
    pub fn from_str(state: &str) -> Option<State> {
        match &state.to_uppercase()[..] {
            "NSW" => Some(NSW),
            "VIC" => Some(VIC),
            "QLD" => Some(QLD),
            "SA" => Some(SA),
            "WA" => Some(WA),
            "TAS" => Some(TAS),
            "NT" => Some(NT),
            "ACT" => Some(ACT),
            _ => None,
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
            ACT | NT => 2,
            _ => 12,
        }
    }

    pub fn all_states() -> Vec<Self> {
        vec![
            NSW,
            VIC,
            QLD,
            SA,
            WA,
            TAS,
            NT,
            ACT
        ]
    }
}
