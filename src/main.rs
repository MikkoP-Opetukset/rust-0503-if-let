#![allow(dead_code, unused_variables, unused_mut)]

/// Entry point for Chapter 6.3: Concise Control Flow with `if let` and
/// `let...else`
///
/// `if let` runs code when one pattern is the case we care about. It is a
/// concise alternative to a `match` when the other variants need no handling.
/// `let...else` extracts a value or leaves the current scope immediately.
fn main() {
    reacting_to_one_event();
    using_if_let_with_option();
    choosing_between_patterns();
    extracting_a_required_reward();
}

/// A game event.
enum GameEvent {
    LevelUp(u32),
    Damage(u32),
    Chat(String),
    Quit,
}

/// A reward that may be present in a treasure chest.
struct Reward {
    name: String,
    gold: u32,
}

/// # Reacting to One Event
/// - `if let` combines a pattern and an `if` condition.
/// - The block runs only when the value matches the pattern.
/// - Unlike `match`, `if let` does not require handling every other variant.
/// - Use it when ignoring the other variants makes the intent clearer.
fn reacting_to_one_event() {
    println!("\n{:=>80}", "");
    println!("reacting_to_one_event()\n");

    let event = GameEvent::LevelUp(5);

    // This checks whether `event` is the `LevelUp` variant. If it is, the
    // number stored inside the variant is bound to `level` for this block.
    if let GameEvent::LevelUp(level) = event {
        println!("Level up! Level {level}.");
    }

    // This `if let` is equivalent to this `match`, but `if let` makes it
    // more obvious that only one case matters.
    //
    // match event {
    //     GameEvent::LevelUp(level) => println!("Level up! Level {level}."),
    //     _ => {}
    // }

    let event = GameEvent::Damage(12);

    // This event does not match `LevelUp`, so the block is skipped. There is
    // no need to write match arms for Damage, Chat, and Quit when we do not
    // want to do anything with them.
    if let GameEvent::LevelUp(level) = event {
        println!("This message is not printed: level {level}.");
    }
}

/// # Using `if let` with `Option<T>`
/// - `Some(value)` contains a value and `None` means no value is available.
/// - `if let Some(value) = option` unwraps only the `Some` case.
/// - An `else` block handles the non-matching case when it needs a response.
fn using_if_let_with_option() {
    println!("\n{:=>80}", "");
    println!("using_if_let_with_option()\n");

    let healing_potion = Some(30_u32);
    let empty_slot: Option<u32> = None;

    announce_potion(healing_potion);
    announce_potion(empty_slot);
}

/// Announces the effect of an inventory slot that may contain a potion.
fn announce_potion(potion_healing: Option<u32>) {
    if let Some(health) = potion_healing {
        println!("The hero drinks a potion and restores {health} health.");
    } else {
        println!("The inventory slot is empty.");
    }
}

/// # Choosing Between Patterns
/// - `else if let` lets several patterns have distinct, concise actions.
/// - The first matching condition wins, just like an ordinary `if` chain.
/// - A final `else` can handle every remaining possibility.
fn choosing_between_patterns() {
    println!("\n{:=>80}", "");
    println!("choosing_between_patterns()\n");

    let event = GameEvent::Chat(String::from("Meet me at the village gate."));

    if let GameEvent::Chat(message) = event {
        println!("Party chat: \"{message}\"");
    } else if let GameEvent::Quit = event {
        println!("The player left the game.");
    } else {
        println!("The event does not need a notification.");
    }
}

/// # Extracting a Required Value with `let...else`
/// - A `let` binding normally needs a pattern that always matches.
/// - `let...else` handles a pattern that might not match.
/// - The `else` branch must diverge: for example, it can `return`, `break`,
///   `continue`, or panic.
/// - After `let...else`, the extracted value is available in the rest of the
///   surrounding scope without another level of indentation.
fn extracting_a_required_reward() {
    println!("\n{:=>80}", "");
    println!("extracting_a_required_reward()\n");

    equip_reward(Some(Reward {
        name: String::from("Moon Key"),
        gold: 50,
    }));
    equip_reward(None);
}

/// Equips a reward only when the chest contained one.
fn equip_reward(chest_reward: Option<Reward>) {
    let Some(reward) = chest_reward else {
        println!("The chest is empty, so there is nothing to equip.");
        return;
    };

    println!(
        "The hero equips {} and finds {} gold.",
        reward.name, reward.gold
    );

    // The following is not allowed because its `else` block would finish with
    // `()`, the unit type. A `let...else` block must instead have type `!`,
    // Rust's never type, so execution cannot reach the println! without a
    // reward to use:
    // let Some(reward) = chest_reward else {
    //     println!("The chest is empty.");
    // };
    // println!("The hero equips {}.", reward.name);
    //
    // Add `return`, `break`, `continue`, or a panic to make the `else` branch
    // diverge before code after the binding can run.

    // Without `else`, this refutable pattern is not allowed in a normal let:
    // let Some(reward) = chest_reward; // Uncomment to see the error.
}
