(ns advent.day7
  (:require [clojure.java.io :as io]
            [clojure.string :as s]
            [clojure.set :as set]))

(def input (slurp (io/resource "input")))

(def test-case "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.")

(def step-re #"Step (.) must be finished before step (.) can begin.")

(defn build-adjacency [adj [v e]]
  (update adj v conj e))

(defn adjacencies [input]
  (->> input
       (s/split-lines)
       (map #(re-matches step-re %))
       (map rest)
       (reduce build-adjacency {})))

#_(adjacencies test-case)

(defn first-step [adj]
  (let [ks (set (keys adj) )
        vs (-> (vals adj) (flatten) (set))]
    (set/difference ks vs)))

#_(adjacencies input)

(defn )
