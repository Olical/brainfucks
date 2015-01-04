var pipedInput = [];

/**
 * Performs the first read from stdin and stores it in pipedInput.
 *
 * readOne will later shift values from the front of that string before attempting to read anything new.
 */
function initialiseStdin() {
    process.stdin.setEncoding('utf8');

    if (process.stdin.isTTY) {
        process.stdin.setRawMode(true);
    }
    else {
        process.stdin.resume();
        process.stdin.once('data', function(data) {
            pipedInput = data.split('').slice(0, -1);
        });
    }
}

initialiseStdin();

/**
 * Reads one character from stdin and passes it back to the callback.
 *
 * @param {Function} callback
 */
function readOne(callback) {
    var pipedCharacter = pipedInput.shift();

    if (pipedCharacter) {
        setImmediate(callback.bind(null, pipedCharacter));
    }
    else {
        process.stdin.resume();
        process.stdin.once('data', function(data) {
            callback(data);
        });
    }
}

/**
* Provides a map between brainfuck commands and the functions that the state should subsequently be applied to.
*
* @type {Object}
*/
module.exports = {
    '>': function (state) {
        state.pointers.memory += 1;

        if (state.pointers.memory === state.memory.length) {
            state.memory.push(0);
        }
    },
    '<': function (state) {
        state.pointers.memory -= 1;

        if (state.pointers.memory === -1) {
            state.memory.unshift(0);
            state.pointers.memory = 0;
        }
    },
    '+': function (state) {
        state.memory[state.pointers.memory] += 1;
    },
    '-': function (state) {
        state.memory[state.pointers.memory] -= 1;
    },
    '.': function (state) {
        var character = String.fromCharCode(state.memory[state.pointers.memory]);
        process.stdout.write(character);
    },
    ',': function (state, program, callback) {
        readOne(function (character) {
            state.memory[state.pointers.memory] = character.charCodeAt(0);
            callback();
        });
    },
    '[': function (state, program) {
        if (state.memory[state.pointers.memory] === 0) {
            state.pointers.program = program.jumps[state.pointers.program];
        }
    },
    ']': function (state, program) {
        if (state.memory[state.pointers.memory] !== 0) {
            state.pointers.program = program.jumps[state.pointers.program];
        }
    }
};
