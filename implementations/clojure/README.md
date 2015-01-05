Clojure isn't my strongest language, yet, but this is my brainfuck implementation built with it. I've never really built a full application with it up until now but I've read a few books. I hope my approach doesn't cause you physical discomfort.

Run it with `lein trampoline run [path]`. Trampoline will avoid running the application in a sub-process of the original leinigen Java process. This allows stdin to work.

I'm taking the same approach as my JavaScript implementation.

 * Validate arguments.
 * Make sure the file exists.
 * Read it.
 * Attempt to parse / compile it.
 * Execute if it compiled successfully.
