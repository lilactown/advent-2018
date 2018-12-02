(ns advent.day2
  (:require [clojure.string :as s]
            [clojure.java.io :as io]))

(defn tuples? [s]
  (->> (frequencies s)
       (filter (fn [[k v]] (= v 2)))
       (empty?)
       (not)))

#_(tuples? "aab")

(defn triples? [s]
  (->> (frequencies s)
       (filter (fn [[k v]] (= v 3)))
       (empty?)
       (not)))

#_(triples? "aaab")

(defn part1 [input]
  (let [lines (s/split-lines input)
        tuples (->> (map tuples? lines)
                    (filter true?)
                    (count))
        triples (->> (map triples? lines)
                     (filter true?)
                     (count))]
    (* tuples triples)))

(time (part1 (slurp (io/resource "input"))))

(defn difference [acc c1 c2]
  (if (not= c1 c2)
    (inc acc)
    acc))

(defn differences [s1 s2]
  (reduce-kv difference 0 (zipmap s1 s2)))
