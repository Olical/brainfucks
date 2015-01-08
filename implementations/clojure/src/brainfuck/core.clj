(ns brainfuck.core)

(defn file-exists? [path]
  "Checks if a file exists by path."
  (.exists (clojure.java.io/file path)))

(defn has-balanced-braces? [source]
  "Checks if a source string has balanced square braces."
  (let [f (frequencies source)]
    (= (f \[) (f \]))))

(def valid-tokens #{\+ \- \< \> \[ \] \. \,})

(defn tokenise [source]
  "Turns a source string into a sequence of valid tokens."
  (map #(hash-map :token %) (filter #(contains? valid-tokens %) source)))

(defn find-jumps [tokens]
  "Finds all jumps to square braces in both directions."
  (loop [remaining tokens
         stack []
         jumps {}
         index 0]
    (if (empty? remaining)
      jumps
      (let [token (:token (first remaining))
            last-on-stack (last stack)
            next-stack (case token
                         \[ (conj stack index)
                         \] (pop stack)
                         stack)
            next-jumps (if (= token \])
                         (assoc jumps
                                last-on-stack index
                                index last-on-stack)
                         jumps)]
        (recur (rest remaining)
               next-stack
               next-jumps
               (+ index 1))))))

(defn match-braces [tokens]
  "Attaches brace jump location meta data to tokens in a sequence of tokens."
  (let [jumps (find-jumps tokens)]
    (map-indexed (fn [index token]
                   (let [match (get jumps index)]
                     (if (nil? match)
                       token
                       (assoc token :destination match)))) tokens)))

(defn parse [source]
  "Takes a brainfuck source string and turns it into an executable program."
  (-> source
      tokenise
      match-braces))

(defn read-character [_]
  "Reads a character from STDIN"
  (.read System/in))

(defn safe-inc [n]
  "Treat nil values as zero."
  (inc (if (= n nil) 0 n)))

(defn safe-dec [n]
  "Treat nil values as zero."
  (dec (if (= n nil) 0 n)))

(defmulti step (fn [program state] (:token (nth program (:program-pointer state)))))
(defmethod step \> [program state] (update-in state [:memory-pointer] inc))
(defmethod step \< [program state] (update-in state [:memory-pointer] dec))
(defmethod step \+ [program state] (update-in state [:memory (:memory-pointer state)] safe-inc))
(defmethod step \- [program state] (update-in state [:memory (:memory-pointer state)] safe-dec))
(defmethod step \. [program state] (print (char (get-in state [:memory (:memory-pointer state)]))) state)
(defmethod step \, [program state] (update-in state [:memory (:memory-pointer state)] read-character))
(defmethod step \[ [program state] (if (= 0 (get-in state [:memory (:memory-pointer state)]))
                                     (update-in state [:program-pointer] #(:destination (nth program %)))
                                     state))
(defmethod step \] [program state] (if (not (= 0 (get-in state [:memory (:memory-pointer state)])))
                                     (update-in state [:program-pointer] #(:destination (nth program %)))
                                     state))

(defn execute [program]
  "Executes a compiled brainfuck program."
  (let [program-length (count program)]
    (loop [state {:memory (hash-map 0 0)
                  :memory-pointer 0
                  :program-pointer 0}]
      (if (< (:program-pointer state) program-length)
        (recur (update-in (step program state) [:program-pointer] inc))))))

(defn -main
  "Entry point into the brainfuck interpreter. Takes a source file path to execute."
  ([source-file]
   (if (file-exists? source-file)
     (let [source (slurp source-file)]
       (if (has-balanced-braces? source)
         (execute (parse source))
         (println (str "\"" source-file "\"") "has unbalanced square braces.")))
     (println (str "\"" source-file "\"") "does not exist.")))
  ([]
   (println "Usage: lein trampoline run [source]")))
