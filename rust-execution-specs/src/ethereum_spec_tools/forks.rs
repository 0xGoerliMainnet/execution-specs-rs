use std::{fmt::Debug};

///! Hard Forks of the Ethereum blockchain.
#[derive(Debug, Hash, Default)]
pub enum HardFork {
    #[default]
    Frontier,
    // Homestead,
    // DaoFork,
    // TangerineWhistle,
    // SpuriousDragon,
    // Byzantium,
    // Constantinople,
    // Petersburg,
    // Istanbul,
    // MuirGlacier,
    // Berlin,
    // London,
    // ArrowGlacier,
    // GrayGlacier,
    // Paris,
    // #[default]
    // Shanghai,
}
