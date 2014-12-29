This is my reference implementation since JavaScript is my strongest language. It will take one file name as an argument, read it and then begin execution.

> The brainfuck language uses a simple machine model consisting of the program and instruction pointer, as well as an array of at least 30,000 byte cells initialized to zero; a movable data pointer (initialized to point to the leftmost byte of the array); and two streams of bytes for input and output (most often connected to a keyboard and a monitor respectively, and using the ASCII character encoding).

So I need at least 30,000 cells, lets make that infinite expansion to the left and right. Then two pointers, one for the program and one for the memory cells. We then need to handle commands which will do things with the current pointer. Will also have to work out how to do loops properly, nested loops could be a problem if my solution is dumb.

Instead of actually allocating cells, I may as well push and unshift values into the memory where required. I'm not going to bother with any kind of GC, although it probably wouldn't be too hard to implement. All trailing zero cells would be trimmed because all cells are presumed zero by default anyway.

The original implementation also mentions 8 bit cells, but I don't want to impose artificial limitations. I'll just use JavaScript's built in numbers.

Here's the CLI wrapper with a warning if you don't provide a path. It currently reads and logs the provided file.

```javascript
#!/usr/bin/env node

var fs = require('fs');
var sourceFile = process.argv[2];

if(fs.existsSync(sourceFile)) {
    var source = fs.readFileSync(sourceFile, 'utf-8');
    console.log(source);
}
else {
    console.error('Usage: brainfuck.js [source file]');
}
```

I'm going to take a fairly functional approach to this by having one main function that relies on other very simple functions to manipulate a state object. This is where I wish I had immutable data structures (if I was allowing myself dependencies I'd add Immutable.js http://facebook.github.io/immutable-js/).

Here's the outline of my main loop.

```javascript
/**
 * Provides a map between brainfuck commands and the functions that the state should subsequently be applied to.
 *
 * @type {Object}
 */
var commands = {
    '>': null,
    '<': null,
    '+': null,
    '-': null,
    '.': null,
    ',': null,
    '[': null,
    ']': null
};

/**
 * Executes brainfuck source. Will pull from stdin and print to stdout where required. Here be side effects.
 *
 * @param {String} source A brainfuck application.
 */
function run(source, initialState) {
    var state = getInitialState();

    while (state.pointers.program < source.length) {
        state.pointers.program += 1;
    }
}

/**
 * Constructs the initial state object.
 *
 * @return {Object}
 */
function getInitialState() {
    return {
        memory: [],
        pointers: {
            program: 0,
            memory: 0
        }
    };
}
```

Before I begin implementing the commands (which will be very easy) I want to get the looping right. I have to build some sort of pairing between the left and right hand square braces. If they're unbalanced I'll exit early too.

```javascript
if (isBalanced(source, ['[', ']'])) {
    run(source);
}
else {
    console.error('Source has unbalanced square braces.');
}

/**
 * Checks if the string has the exact same amount of every string in the delimiters array.
 *
 * @param {String} source Subject to validate.
 * @param {String[]} delimiters Characters to count.
 * @return {Boolean} True if balanced, false if not.
 */
function isBalanced(source, delimiters) {
    var frequency = delimiters.map(countFrequency.bind(null, source));
    return frequency.every(function (count) {
        return count === frequency[0];
    });
}

/**
 * Counts the frequency of a string in a string.
 *
 * @param {String} source The thing to look inside.
 * @param {String} substring The thing to look for.
 * @return {Number} The amount of times the substring appears.
 */
function countFrequency(source, substring) {
    return source.split(substring).length - 1;
}
```
