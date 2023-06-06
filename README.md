# rustbucket

The ~~Rust~~ Crab multi-emulator project.

## What Is It?

I'm starting this project as an attempt to build a 6502 emulator
from scratch. However, I've decided to structure the project in
such a way that adding more architectures should be possible.
I want to make as much of the code generic and reusable as I can.
Currently this is demonstrated in the `Register` type.

## What's the status?

As of 6/6/23, I've really only just started on the 6502
implementation. I can't really say how long it's going to take
but I am chipping away at it steadily (I'm also doing a monkey
interpreter following Thortsten Ball's excellent book "Writing
An Interpreter In Go").

## Useful Links

- https://wiki.cdot.senecacollege.ca/wiki/6502_Addressing_Modes
- https://www.middle-engine.com/blog/posts/2020/06/23/programming-the-nes-the-6502-in-detail#addressing-modes
- https://www.masswerk.at/6502/6502_instruction_set.html
