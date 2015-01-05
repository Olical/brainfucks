(ns brainfuck.core)

(defn file-exists? [path]
  "Checks if a file exists by path."
  (.exists (clojure.java.io/file path)))

(defn -main
  "Entry point into the brainfuck interpreter. Takes a source file path to execute."
  ([source-file]
   (if (file-exists? source-file)
     (println "GO!")
     (println (str "\"" source-file "\"") "does not exist.")))
  ([]
   (println "Usage: lein trampoline run [source]")))

;; "" has unbalanced square braces.
