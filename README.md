[![Build Status](https://travis-ci.org/GiedriusS/civ.svg?branch=master)](https://travis-ci.org/GiedriusS/civ)

# civ
CI pipeline graphical viewer

# Motivation
CI pipelines are getting more and more cluttered which means that sometimes it might be hard to understand the relationships between different steps. This is where we need some kind of software that shows those things graphically because it is easier to understand that for humans.

Written in Rust to study that new and upcoming language which gets rid of all memory-related errors. Ought to work pretty flawlessly since there will be no unsafe blocks here, barring algorithmic errors.

# Options

| Option      | Purpose                             | Example           |
|-------------|-------------------------------------|-------------------|
| -i/--input  | Input FILE. Pass '-' for stdin      | /tmp/test         |
| -o/--output | Output FILE. Pass '-' for stdout    | /tmp/otpt         |
| -f/--ifmt   | Input FORMAT. Valid values: DRONE   | DRONE             |
| -m/--ofmt   | Output FORMAT. Valid values: SVG    | SVG               |

# Examples of input and output

## Simple pipeline
