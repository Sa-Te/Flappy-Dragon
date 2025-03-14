# Flappy Dragon

**Flappy Dragon** is a Rust-based clone of the classic Flappy Bird game, built using the [bracket-lib](https://github.com/amethyst/bracket-lib) for rendering and simple game logic. In this game, you control a dragon that must navigate through obstacles without colliding with them.

## Features

- **Simple gameplay:** Tap the spacebar to make the dragon flap its wings and avoid obstacles.
- **Dynamic obstacles:** Obstacles move from right to left and become more challenging as the score increases.
- **Score tracking:** Earn points by successfully passing obstacles.
- **Menu system:** Play the game, restart after death, or quit from the main menu.

## Requirements

- [Rust](https://www.rust-lang.org/) (latest stable version recommended)
- [bracket-lib](https://github.com/amethyst/bracket-lib)

## How to Run

1. **Clone the repository** (if you haven't already):
   ```bash
   git clone https://github.com/your-username/your-repo.git
   cd your-repo

    Build and run the project:

    cargo run

    Gameplay:
        Press SPACE to make the dragon flap.
        Avoid obstacles and try to score as high as possible!

##Project Structure

    main.rs: Contains the main game loop, game state management, and the implementation of the game logic including player movement, obstacle generation, and collision detection.
    Cargo.toml: Contains project metadata and dependency definitions.

##Contributing

Feel free to fork the repository, submit issues, or create pull requests if you have any suggestions or improvements.
License

This project is open-source and available under the MIT License.
