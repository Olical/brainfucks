This is the JavaScript reference implementation of my brainfuck interpreters. Install dependencies with `npm install` and execute with `./brainfuck.js [file]`.

My initial attempt (860a93d643) was very dumb and synchronous, it got too stringy, so I'm going to rely on some libraries that make JavaScript slightly more bearable.

## Application flow

 * Validate and read in file. Die early if something's wrong.
 * Tokenise the contents into an array of valid tokens.
 * Parse those tokens into a jump list and initial state.
 * Iterate though the code applying the operations to the state objects asynchronously.
