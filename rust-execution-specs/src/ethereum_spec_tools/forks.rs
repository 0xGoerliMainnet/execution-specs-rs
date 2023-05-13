///! Hard Forks of the Ethereum blockchain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum HardFork {
    Frontier,
    Homestead,
    DaoFork,
    TangerineWhistle,
    SpuriousDragon,
    Byzantium,
    Constantinople,
    Petersburg,
    Istanbul,
    MuirGlacier,
    Berlin,
    London,
    ArrowGlacier,
    GrayGlacier,
    Paris,
    #[default]
    Shanghai,
}