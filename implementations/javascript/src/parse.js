var commands = require('./commands');

/**
 * Parses a raw brainfuck source file into a set of tokens and a jump list between square bracket pairs.
 *
 * @param {String} source
 * @return {Object} A tokens array, jump map and success boolean called "ok". If false, the square braces will be bad.
 */
function parse(source) {
    var result = {
        tokens: getValidTokens(source),
        ok: null
    };

    if (isBalanced(result.tokens)) {
        result.ok = true;
        result.jumps = getJumps(result.tokens);
    }
    else {
        result.ok = false;
    }

    return result;
}

/**
 * Tokenises a source string into an array of valid tokens.
 *
 * @param {String} source
 * @return {String[]}
 */
function getValidTokens(source) {
    var commandNames = Object.keys(commands);

    return source.split('').filter(function (character) {
        return commandNames.indexOf(character) !== -1;
    });
}

/**
 * Checks if the tokens have balanced square braces.
 *
 * @param {String[]} tokens
 * @return {Boolean}
 */
function isBalanced(tokens) {
    return count('[', tokens) === count(']', tokens);
}

/**
 * Counts the amount of times the token appears.
 *
 * @param {String} needle Token to look for.
 * @param {String[]} haystack Tokens to look inside.
 */
function count(needle, haystack) {
    return haystack.filter(function (character) {
        return character === needle;
    }).length;
}

/**
 * Finds the jumps within a set of tokens. So it matches a [ to a ] as well as ] to [.
 *
 * @param {String[]} tokens
 * @return {Object}
 */
function getJumps(tokens) {
    var jumps = {};
    var stack = [];
    var last;

    tokens.forEach(function (character, index) {
        if (character === '[') {
            stack.push(index);
        }
        else if (character === ']') {
            last = stack[stack.length - 1];
            jumps[last] = index;
            jumps[index] = last;
            stack.pop();
        }
    });

    return jumps;
}

module.exports = parse;
