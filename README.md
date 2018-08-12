# Ludum Dare 42: "Running Out of Space"

This is my entry for the 42nd edition of the [Ludum Dare](https://ldjam.com/)
game jam.  The theme for this edition is "Running Out of Space".


## Introduction

My current idea is to have the game take place in space.  An expanding
destructive gravitational anomaly is growing, quickly taking over the entire
observable universe.  You are in control of humanity's last hope at surviving:
A space ship, capable of FTL (faster-than-light) travel, sent from Earth as the
"Growth" is reaching the edge of the Solar System.  Your goal is to run towards
the edge of explored space, and try to find a way to escape our Universe to
escape the Growth.  Space is running out, so you're running _out_ of Space.

(You might note that this concept draws a lot of inspiration from a story in
the podcast ["The Adventure
Zone"](http://www.maximumfun.org/shows/adventure-zone).)

This project is made in Rust with the [ggez](http://ggez.rs/) game engine.  I
chose Rust because I am currently learning the language, and I thought it would
be a fun way to practise.

![Ship physics and controls](https://raw.githubusercontent.com/xlambein/ldjam42/master/gifs/03.gif)


## Gameplay

What I have in mind for now is the following.  You control a space ship in a 2D
solar system, which you can navigate to find resources.  The ship can be moved
around and obeys the laws of physics.  You can surf on the surface of gas
giants to collect gases, and land onto rock planets to collect solids and
liquids.  A clock indicates how long you have until the Growth reaches the
outer edge of this system.

Once the Growth has arrived, you have a handful of seconds to activate your FTL
engine and jump to another system.  When you do this, a new interface shows up,
indicating the systems within range, and the outer edge of the Growth.

Your cargo has limited resources, which you need to replenish regularly,
ideally everytime you reach a new system.  So far I'm thinking of three kinds
of resources:

- Deuterium, which powers the fusion core of the ship, providing energy to run
  the computers, produce food, etc.
- Hydrogen-Oxygen, which power the subluminal (slower-than-light) engines,
  which you use to navigate around a solar system.
- Some rare resource, "stabilized exotic particles" or something like this,
  which powers the superluminal (faster-than-light) engine.

I have a bunch of other ideas which I might add if I have enough time (spoiler
alert: I won't), or if I go back to work on this game later.


## To-Do List

### Main Game

- [x] Solar system rendering and physics
- [x] Scaling for HDPI monitors
- [x] Basic random system generation
- [x] Spaceship rendering, control and basic physics
- [ ] Landing onto planets
- [ ] Advanced random system generation
- [ ] Resources UI
- [ ] Collecting resources
- [ ] Growth rendering and physics
- [ ] Better config file

### FTL Jump Screen

- [ ] Universe rendering
- [ ] Random universe generation
- [ ] Growth rendering
- [ ] Selecting a solar system
- [ ] Basic UI + distance between growth and systems

### Start Menu

- [ ] Start/Exit buttons
- [ ] Config submenu
- [ ] Game intro with context and lore

