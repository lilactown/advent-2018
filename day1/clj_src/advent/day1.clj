(ns advent.day1
  (:require [clojure.string :as s]
            [clojure.java.io :as io]
            [clojure.test :as t]))

(defn str->int [^String s]
  (Integer/parseInt s))

;; #_(Integer/parseInt "+1")

;; #_(map Integer/parseInt ["1" "2" "3"])

;; #_(str->int "+1")

;; #_(str->int "-1")

;; #_(map str->int ["1" "2" "3"])

(defn- parse [input]
  (s/split-lines input))

(defn part1 [input initial]
  (transduce (map str->int) + 0 (parse input)))

(defn- parse-test [input]
  (s/replace input ", " "\n"))

(t/deftest day1-part1
  (t/is (-> "+1, -2, +3, +1"
            (parse-test)
            (part1 0)
            (= 3)))

  (t/is (-> "+1, +1, +1"
            (parse-test)
            (part1 0)
            (= 3)))

  (t/is (-> "+1, +1, -2"
            (parse-test)
            (part1 0)
            (= 0)))

  (t/is (-> "-1, -2, -3"
            (parse-test)
            (part1 0)
            (= -6))))

#_(-> (slurp (io/resource "day1"))
      (part1 0))

#_(conj #{} 1)

(defn part2 [input]
  (loop [n 0
         n+ (cycle (parse input))
         seen? #{}]
    (if (seen? n)
      n
      (recur (+ n (str->int (first n+)))
             (rest n+)
             (conj seen? n)))))

(t/deftest day1-part2
  (t/is (-> (parse-test "+1, -1")
            (part2)
            (= 0)))
  (t/is (-> (parse-test "+3, +3, +4, -2, -4")
            (part2)
            (= 10)))

  (t/is (-> (parse-test "-6, +3, +8, +5, -6")
            (part2)
            (= 5)))

  (t/is (-> (parse-test "+7, +7, -2, -7, -4")
            (part2)
            (= 14))))

(t/run-tests)

#_(time (-> (slurp (io/resource "day1"))
            (part2)))
