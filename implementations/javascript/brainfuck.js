#!/usr/bin/env node

var fs = require('fs');
var sourceFile = process.argv[2];

if(fs.existsSync(sourceFile)) {
    var source = fs.readFileSync(sourceFile, 'utf-8');

    if (isBalanced(source, ['[', ']'])) {
        run(source);
    }
    else {
        console.error('Source has unbalanced square braces.');
    }
}
else {
    console.error('Usage: brainfuck.js [source file]');
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
    var jumps = matchPairs(source, '[', ']');

    while (state.pointers.program < source.length) {
        state.pointers.program += 1;
    }
}

/**
 * Finds the paired indexes of an opening and closing character.
 *
 * @param {String} source The subject to search through.
 * @param {String} lhc Left hand opening character. Not the Large Hadron Collider.
 * @param {String} rhc Right hand closing character.
 * @return {Object} A map linking indexes of lhc to rhc and rhc to lhc.
 */
function matchPairs(source, lhc, rhc) {
    var pairs = {};
    var stack = [];
    var last;

    Array.prototype.forEach.call(source, function (c, index) {
        if (c === lhc) {
            stack.push(index);
        }
        else if (c === rhc) {
            last = stack[stack.length - 1];
            pairs[last] = index;
            pairs[index] = last;
            stack.pop();
        }
    });

    return pairs;
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
