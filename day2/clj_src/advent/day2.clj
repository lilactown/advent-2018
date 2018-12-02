(ns advent.day2
  (:require [clojure.string :as s]
            [clojure.java.io :as io]))

(defn tuples? [s]
  (->> (frequencies s)
       (filter (fn [[k v]] (= v 2)))
       (empty?)
       (not)))

(tuples? "aab")

(defn triples? [s]
  (->> (frequencies s)
       (filter (fn [[k v]] (= v 3)))
       (empty?)
       (not)))

(triples? "aaab")

(defn part1 [input]
  (let [lines (s/split-lines input)
        tuples (->> (map tuples? lines)
                    (filter true?)
                    (count))
        triples (->> (map triples? lines)
                     (filter true?)
                     (count))] 
       (* tuples triples)))

(part1 (slurp (io/resource "input")))
