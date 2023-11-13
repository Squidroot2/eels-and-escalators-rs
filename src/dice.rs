use rand::RngCore;

#[derive(Copy, Clone)]
enum EEDieState {
    Eel,
    Escalator,
}

impl From<u32> for EEDieState {
    fn from(value: u32) -> Self {
        match value {
            0 => return EEDieState::Eel,
            1 => return EEDieState::Escalator,
            _ => panic!("value {} Out of range!", value),
        }
    }
}

impl Default for EEDieState {
    fn default() -> Self {
        EEDieState::Eel
    }
}

struct EEDie {
    state: EEDieState,
}

impl EEDie {
    fn new() -> Self {
        EEDie {
            state: EEDieState::default(),
        }
    }

    fn roll<R>(&mut self, rng: &mut R)
    where
        R: RngCore,
    {
        let index = rng.next_u32() % 2; // 2 = possible enum states
        self.state = EEDieState::from(index)
    }
}

struct NumberDie {
    start: u32,
    sides: u32,
    state: u32,
}

impl NumberDie {
    fn new_sixed_sided() -> Self {
        NumberDie {
            start: 1,
            sides: 6,
            state: 1,
        }
    }
    fn roll<R>(&mut self, rng: &mut R)
    where
        R: RngCore,
    {
        self.state = rng.next_u32() & self.sides + self.start;
    }
}

pub struct DiceSet {
    number_die: NumberDie,
    ee_die1: EEDie,
    ee_die2: EEDie,
}

impl DiceSet {
    pub fn new() -> Self {
        DiceSet {
            number_die: NumberDie::new_sixed_sided(),
            ee_die1: EEDie::new(),
            ee_die2: EEDie::new(),
        }
    }

    pub fn roll_all<R>(&mut self, rng: &mut R)
    where
        R: RngCore,
    {
        self.number_die.roll(rng);
        self.ee_die1.roll(rng);
        self.ee_die2.roll(rng);
    }

    pub fn get_result(&self) -> RollResult {
        match (
            self.ee_die1.state,
            self.ee_die2.state,
            self.number_die.state,
        ) {
            (EEDieState::Eel, EEDieState::Eel, number) => RollResult::Eels(number),
            (EEDieState::Escalator, EEDieState::Escalator, number) => RollResult::Escalator(number),
            (_, _, number) => RollResult::Number(number),
        }
    }
}

pub enum RollResult {
    Eels(u32),
    Escalator(u32),
    Number(u32),
}
