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
  (let [boxes (s/split-lines input)
        tuples (->> (map tuples? boxes)
                    (filter true?)
                    (count))
        triples (->> (map triples? boxes)
                     (filter true?)
                     (count))]
    (* tuples triples)))

#_(time (part1 (slurp (io/resource "input"))))

(defn difference [acc [c1 c2]]
  (if (not= c1 c2)
    (conj acc [c1 c2])
    acc))

(defn differences [s1 s2]
  (reduce difference [] (->> (interleave s1 s2)
                             (partition 2))))

(defn without-diffs [s1 s2]
  (-> (reduce (fn [acc [c1 c2]] (if (= c1 c2) (conj acc c1) acc))
              []
              (->> (interleave s1 s2)
                   (partition 2)))
      (s/join)))

#_(differences "aabb" "anb2")

#_(differences "aabb" "abbb")

(defn diff-with [s coll]
  (loop [s' (first coll)
         coll (rest coll)]
    (cond
      (= 1 (count (differences s s'))) s'
      (empty? coll) nil
      :else (recur (first coll)
                   (rest coll)))))

(diff-with "aabb" ["abab" "aabb" "abcd" "ancb" "efas"])

(defn part2 [input]
  (let [boxes (s/split-lines input)]
    (apply without-diffs
           (loop [s (first boxes)
                  boxes (rest boxes)
                  match-diff (diff-with s boxes)]
             (cond
               (not (nil? match-diff)) [s match-diff]
               (empty? boxes) (throw (RuntimeException. "Could not find a matching box ID"))
               :else (recur (first boxes)
                            (rest boxes)
                            (diff-with (first boxes) (rest boxes))))))))

#_(part2 "abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz")

#_(time (part2 (slurp (io/resource "input"))))
