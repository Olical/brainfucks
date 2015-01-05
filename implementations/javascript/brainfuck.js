#!/usr/bin/env node

var fs = require('fs');
var parse = require('./src/parse');
var execute = require('./src/execute');

/**
 * Entry point for the application.
 */
function main() {
    var sourcePath = process.argv[2];

    if (sourcePath) {
        if (fs.existsSync(sourcePath)) {
            source = fs.readFileSync(sourcePath, 'utf-8');
            var program = parse(source);

            if (program.ok) {
                execute(program);
            }
            else {
                console.error('"' + sourcePath + '" has unbalanced square braces.');
            }
        }
        else {
            console.error('"' + sourcePath + '" does not exist.');
        }
    }
    else {
        console.error('Usage: brainfuck.js [source]');
    }
}

main();
