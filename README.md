<div align="center">

# Rusty Bird

![Github testing](https://github.com/mawilms/rusty-bird/actions/workflows/testing.yml/badge.svg)
![GitHub issues](https://img.shields.io/github/issues/mawilms/rusty-bird)
![GitHub downloads](https://img.shields.io/github/downloads/mawilms/rusty-bird/total)
![GitHub Latest version](https://img.shields.io/github/v/release/mawilms/rusty-bird?include_prereleases)

</div>

Another Flappy Bird clone? :duck: Rusty bird is more than a Flappy Bird clone. Rusty bird was programmed to make your entry into reinforcement learning as easy as possible. With the help of Rusty Bird you can train your own independent agent to play the game.

Want to build an agent classically in Python?
Maybe something more exotic like Haskell or Elixir? No problem. Rusty Bird uses a TCP stream that you can simply connect to and get the state and current score of the game.

You should definitely try Elixir! :eyes:

## Table of contents

- [Features](#features)
- [Usage](#usage)
- [FAQ](#faq)
- [License](#license)

## Features

- Full fledged Flappy bird game
- TCP Server to extract the state and score out of the game

## Usage

Rust bird is a CLI application which starts the flappy bird game. Currently the only thing you need is the executable and Rust installed.

You can start the game with `rusty-bird start`.

## FAQ

### **_Will you add new features to Rusty Bird?_**

Definitely! This project is a great learning experience for me. There are already a few features planned to make the development for your agents easier. Since I'm not a data scientist, I need your help! :blush:

## License

Rusty Bird is release under the [GNU GPLv3](https://github.com/mawilms/lembas/blob/main/LICENSE)
