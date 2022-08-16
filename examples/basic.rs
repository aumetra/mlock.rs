use mlock::Mlock;

fn generate_private_key() -> [u8; 32] {
    [0xFF; 32]
}

fn main() {
    // Look out! We're generating some cryptographic secrets here!
    let secrets = generate_private_key();

    // ..but we don't want them to be written to swap
    let locked_secrets = Mlock::new(secrets).unwrap();

    // ..so we lock them into the RAM and prevent them from getting swapped!

    // ..and now we want to unlock the stuff (for some reason)
    let _unlocked_secrets = locked_secrets.unlock().unwrap();
}
