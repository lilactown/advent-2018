(ns advent.day3
  (:require [clojure.string :as s]))

(defn str->int [^String s]
  (Integer/parseInt s))

(defn ->claim [claim-str]
  (let [[_ id left
         top
         width
         height]
        (re-matches #"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)" claim-str)]
    {:id id
     :left (str->int left)
     :top (str->int top)
     :width (str->int width)
     :height (str->int height)}))

(->claim "#1 @ 1,3: 4x4")

(defn claim->coords [{:keys [top left width height] :as claim}]
  (for [left (range left (+ left width))
        top (range top (+ top height))]
    (list left top)))

(->> "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2"
     (s/split-lines)
     (map ->claim)
     (map claim->coords)
     (apply concat))
