# MHWorld CLI cheat tool

A tool that automatically applies some cheats to Monster Hunter World. The cheats applied are:
1. Add 1000000 money.
2. Add 1000000 points.
3. Add 1000 of every material possible to the item box. NOTE: Unless you naturally get the item from the monster you won't be able to use it to make weapons or armor even though you'll have 1000 units of it. This cheat removes the farming aspect from the game, but not the progression. You will still need to hunt monsters at least one time to be able to use their weapons.

Cheats that I may work on in the future:
- Give 1000 units of every item to the player (ammo, consumable items, etc), not just materials.

## Building

Run the following command in the project dir:
```sh
cargo build --release
```

# Usage instructions

1. Open Monster Hunter World and create a offline game.
2. Get the PID for the game using some utility like `ps`.
3. Run the software with `cargo run --release <PID>`, where the PID is the one you got in the last step (remove the "<" and ">").
