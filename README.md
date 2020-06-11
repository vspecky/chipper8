# Chipper8.rs (Version Beta)
Chipper8.rs will be a CHIP-8 emulator/interpreter written in Rust. Just a small hobby project for me to learn Rust.  
Thinking of making this a pure CLI, but I might add a GUI too.\
Update: Implementing a GUI is actually easier so that's what I'm gonna do. The CLI will just refer to the ability to use the command line to load games.\
\
Update 2: It works quite well now. Only thing that remains is adding sound support.

## What is CHIP-8?
CHIP-8 is a simple interpreted programming language that was developed by Joseph Weisbecker back in the 1970s for the COSMAC VIP and Telmac 1800 Microcomputers. It was used to create quite a few games too. More information can be found on the [Wikipedia Article](https://en.wikipedia.org/wiki/CHIP-8).

## How do I use this interpreter?
- Get the public domain games from [here](https://www.zophar.net/pdroms/chip8/chip-8-games-pack.html#:~:text=They%20are%3A%2015%20Puzzle%2C%20Blinky,%2C%20UFO%2C%20Vbrix%20and%20Wipeoff.).  
- Compile the code.  
- Run the game from the terminal as so  
```
./executable <rom_file>
```

## What if I wanna do my own implementation?
In that case, you should check out the following two websites :-
- [Cowgod's CHIP-8 reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)  
- [Matthew Mikolay's "Mastering CHIP-8"](http://mattmik.com/files/chip8/mastering/chip8.html)  

Mattmik's website is great for understanding the CHIP-8 language and Cowgod's website is a very concise and helpful reference.
