/**
* Provides a map between brainfuck commands and the functions that the state should subsequently be applied to.
*
* @type {Object}
*/
module.exports = {
    '>': function (s) {
        s.pointers.memory += 1;
        if (s.pointers.memory === s.memory.length) {
            s.memory.push(0);
        }
    },
    '<': function (s) {
        s.pointers.memory -= 1;
        if (s.pointers.memory === -1) {
            s.memory.unshift(0);
            s.pointers.memory = 0;
        }
    },
    '+': function (s) {
        s.memory[s.pointers.memory] += 1;
    },
    '-': function (s) {
        s.memory[s.pointers.memory] -= 1;
    },
    '.': function (s) {
        var c = String.fromCharCode(s.memory[s.pointers.memory]);
        process.stdout.write(c);
    },
    ',': function (s, callback) {
        process.stdin.once('data', function (key) {
            s.memory[s.pointers.memory] = key.charCodeAt(0);
            callback();
        });
    },
    '[': function (s) {
        if (s.memory[s.pointers.memory] === 0) {
            s.pointers.program = s.jumps[s.pointers.program];
        }
    },
    ']': function (s) {
        if (s.memory[s.pointers.memory] !== 0) {
            s.pointers.program = s.jumps[s.pointers.program];
        }
    }
};
