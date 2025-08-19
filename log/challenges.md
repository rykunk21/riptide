

### Notes
The purpose of traits is to simplify contracts. Where statements and messaging validation occur because contracts can be defined for types that impl a particular trait. (eg the readable actors can have a contract with the reader actors)


### Development Paradigm  
Actors are modeled as enums representing their discrete states. Each actor’s impl block defines actions that operate on its own state and potentially manipulate other actors by passing ownership or mutable references, aiming for strong type safety and clarity.

### Roadblock  
Rust’s ownership and borrowing rules prevent passing concrete enum actors by mutable reference or ownership between actors without complex lifetimes or trait abstractions. This complicates direct state manipulation and hampers testing with mocks (KEY), while traits add unwanted complexity.

### Solution  
Adopt a **message-passing pattern** where actors communicate by sending typed messages (commands/events). Each actor owns and updates its own state based on incoming messages, eliminating direct mutable borrows, preserving enum clarity, enabling easier testing, and aligning naturally with Rust’s ownership model.

