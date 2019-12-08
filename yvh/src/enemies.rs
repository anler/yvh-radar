/// Denotes the number and kind of enemies.
pub struct Enemies {
    kind: EnemyKind,
    number: u32,
}

/// Denotes the different kinds of enemy.
pub enum EnemyKind {
    Soldier,
    Mech,
}
