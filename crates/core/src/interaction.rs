#[derive(Debug)]
pub enum InteractionType {
    Interact,
    Attack,
    InteractAt,
}

#[derive(Debug)]
pub enum InteractHand {
    Main,
    Offhand,
}
