var async = require('async');
var commands = require('./commands');

/**
 * Executes a parsed brainfuck program.
 *
 * @param {Object} program
 */
function execute(program) {
    var state = getInitialState();
    async.whilst(
        isProgramIncomplete.bind(null, program, state),
        onStep.bind(null, program, state),
        onComplete
    );
}

/**
 * Constructs an initial state object.
 *
 * @return {Object}
 */
function getInitialState() {
    return {
        memory: [0],
        pointers: {
            program: 0,
            memory: 0
        }
    };
}

/**
 * Checks if the pointer has not hit the end of the program.
 *
 * @param {Object} program
 * @param {Object} state
 * @return {Boolean}
 */
function isProgramIncomplete(program, state) {
    return state.pointers.program < program.tokens.length;
}

/**
 * Executed every step. Will call the correct command for the token passing in the program and state.
 *
 * @param {Object} program
 * @param {Object} state
 * @param {Function} callback To be called when the step is finished.
 */
function onStep(program, state, callback) {
    var programPointer = state.pointers.program;
    var token = program.tokens[programPointer];
    var command = commands[token];

    command(state, program, callback);
    state.pointers.program += 1;

    if (command.length < 3) {
        setImmediate(callback);
    }
}

/**
 * Does nothing, but is executed on completion of the main program loop.
 */
function onComplete() {
}

module.exports = execute;
