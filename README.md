<div align="center">

# Rusty Bird

![Github testing](https://github.com/mawilms/rusty-bird/actions/workflows/build.yml/badge.svg)
![GitHub issues](https://img.shields.io/github/issues/mawilms/rusty-bird)
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

### **_How does the state information look like?_**

The state is send as a JSON struct. It contains the `y-position` of the Player, the `score` and the distance of the first three pipe obstacles and the player.

This is how the schema is looking:

```json
{
  "player": <player y-position>,
  "score": <int>,
  "pipes": [
    [
      [<lower x distance>, <lower y distance>],
      [<upper x distance>, <upper y distance>]
    ],
    [
      [<lower x distance>, <lower y distance>],
      [<upper x distance>, <upper y distance>]
    ],
    [
      [<lower x distance>, <lower y distance>],
      [<upper x distance>, <upper y distance>]
    ]
  ]
}
```

This is example data:

```json
{
  "player": 103.79996,
  "score": 0,
  "pipes": [
    [
      [168.0, 267.20004],
      [168.0, -182.79996]
    ],
    [
      [418.0, 98.20004],
      [418.0, -351.79996]
    ],
    [
      [668.0, 111.20004],
      [668.0, -338.79996]
    ]
  ]
}
```

### **_Will you add new features to Rusty Bird?_**

Definitely! This project is a great learning experience for me. There are already a few features planned to make the development for your agents easier. Since I'm not a data scientist, I need your help! :blush:

## License

Rusty Bird is release under the [GNU GPLv3](https://github.com/mawilms/rusty-bird/blob/main/LICENSE)
