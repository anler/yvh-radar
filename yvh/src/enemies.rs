/// Denotes the number and kind of enemies.
pub struct Enemies {
    pub kind: EnemyKind,
    pub number: u32,
}

impl Enemies {
    /// Returns true if it's a mech enemy.
    pub fn is_mech(&self) -> bool {
        match self.kind {
            EnemyKind::Mech => true,
            _ => false,
        }
    }
}

/// Denotes the different kinds of enemy.
pub enum EnemyKind {
    Soldier,
    Mech,
}
