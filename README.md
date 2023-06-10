# rustbucket

My Rust 6502 emulator project.

## What Is It?

An emulator for the venerable MOS Technology 6502 microprocessor from the 70's. The brain that powered the Apple II. the NES, and more

## What's the status?

Still a work in progress with many instructions not implemented. Those that are implemented are incomplete (status flags and page boundaries).

In short, no it's not usable quite yet. :grin:

## Is there anything special about it?

~My goal is to make the emulator as accurate as possible, down to emulating individual clock cycles at a realistic speed. To achieve this,
I've overengineered a "task queue" system so that instructions that take multiple clock cycles to complete will take that many iterations
through the main loop to complete. Then each iteration can use a calculated delay to maintain a realistic clock speed (most likely 1Mhz).
When an instruction is decoded, a series of tasks get pushed into a `VecDeque`. On each iteration of the main loop, if this queue is empty,
it simply decodes the next instruction. If the queue is not empty, the next task is popped off the front of the queue and a corresponding
method is called.~

~I'm using a mutable struct field pointer (spooky unsafe stuff) to maintain a "target register" for tasks that require reading or writing
to a register. I'm still only vaguely intermediate with Rust and I fought with the type system trying to implement this task system in a
smart way and failed. I tried multiple different ways and in the end just settled on a mutable raw pointer with the unsafe accesses cordoned
off into safe methods.~

The above strategy has been ditched in favor of performing a single instruction entirely per each main loop iteration. The task queue system
was cool, but was much more difficult to write and debug.

I've added a special opcode (`0xFF`) that isn't used on the real hardware as a halt instruction. Without real pins to pull high or low, I
I needed a way to break out of the main loop (else tests would run the instruction pointer out of index bounds and panic).

## Useful Links

- https://wiki.cdot.senecacollege.ca/wiki/6502_Addressing_Modes
- https://www.middle-engine.com/blog/posts/2020/06/23/programming-the-nes-the-6502-in-detail#addressing-modes
- https://www.masswerk.at/6502/6502_instruction_set.html
