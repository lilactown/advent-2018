(ns advent.day4
  (:require [clojure.string :as s]
            [clojure.java.io :as io]))

(def test-case
  "[1518-11-01 00:00] Guard #10 begins shift
  [1518-11-01 00:05] falls asleep
  [1518-11-01 00:25] wakes up
  [1518-11-01 00:30] falls asleep
  [1518-11-01 00:55] wakes up
  [1518-11-01 23:58] Guard #99 begins shift
  [1518-11-02 00:40] falls asleep
  [1518-11-02 00:50] wakes up
  [1518-11-03 00:05] Guard #10 begins shift
  [1518-11-03 00:24] falls asleep
  [1518-11-03 00:29] wakes up
  [1518-11-04 00:02] Guard #99 begins shift
  [1518-11-04 00:36] falls asleep
  [1518-11-04 00:46] wakes up
  [1518-11-05 00:03] Guard #99 begins shift
  [1518-11-05 00:45] falls asleep
  [1518-11-05 00:55] wakes up")

(defn str->int [^String s]
  (Integer/parseInt s))

(def guard-start
  #"\[1518-(\d+)-(\d+) (\d+):(\d+)\] Guard #(\d+) begins shift")

(def guard-sleeps
  #"\[1518-(\d+)-(\d+) (\d+):(\d+)\] falls asleep")

(def guard-wakes
  #"\[1518-(\d+)-(\d+) (\d+):(\d+)\] wakes up")

(format "%02d" (inc (str->int "12")))

(defn ->start [[_ MM dd hh mm id]]
  (if (= hh "23") ;; started on the previous day, just roll them over
                  ;;to the next for easy reporting
    {:id id :start {:MM MM :dd (format "%02d" (inc (str->int dd))) :hh "00" :mm "00"}}
    {:id id :start {:MM MM :dd dd :hh hh :mm mm}}))

(defn ->sleep [[_ MM dd hh mm]]
  {:type :sleep :MM MM :dd dd :hh hh :mm mm})

(defn ->wake [[_ MM dd hh mm]]
  {:type :wake :MM MM :dd dd :hh hh :mm mm})

(defn pattern->entity [lines pattern entity]
  (->> lines
       (map #(re-matches pattern %))
       (filter (comp not nil?))
       (map entity)))

(defn duration [[sleep wake]]
  (- (str->int (:mm wake))
     (str->int (:mm sleep))))

(defn input->logs [input]
  (let [lines (->> (s/split-lines input) (map s/trim))
        starts (pattern->entity lines guard-start ->start)
        sleeps (pattern->entity lines guard-sleeps ->sleep)
        wakes (pattern->entity lines guard-wakes ->wake)]
    (for [{:keys [id start] :as log} starts]
      (let [{:keys [MM dd]} start
            sleep-log (->> (concat (filter (every-pred #(= (:MM %) MM) #(= (:dd %) dd)) sleeps)
                                   (filter (every-pred #(= (:MM %) MM) #(= (:dd %) dd)) wakes))
                           (sort-by :mm)
                           (partition 2))]
        (assoc log
               :log sleep-log
               :sleep-duration (->> sleep-log
                                    (map duration)
                                    (reduce +)))))))

#_(input->logs test-case)

(defn logs-per-guard [pg log]
  (update-in pg [(:id log) :days] conj (dissoc log :id)))

#_(def lpg (reduce logs-per-guard {} (input->logs test-case)))

(defn sleep-per-guard [logs' id {:keys [days] :as log}]
  (let [total-sleep (->> days (map :sleep-duration) (reduce +))]
    (conj logs' (assoc log :id id :total-sleep total-sleep))))

#_(def spg (reduce-kv sleep-per-guard [] lpg))

#_(def sleepiest (apply max-key :total-sleep spg))

(defn freqs-range [freqs [start end]]
  (map-indexed (fn [i freq]
                 (if (and (>= i start) (< i end))
                   (inc freq)
                   freq))
               (or freqs (repeat 60 0))))

#_(def sleep-freqs
    (->> (map :log (:days sleepiest))
         (flatten)
         (map (comp str->int :mm))
         (partition 2)
         (reduce freqs-range nil)
         (map-indexed #(assoc {} :t %1 :f %2))
         (apply max-key :f)))

(defn debug>> [msg x]
  (println x msg)
  x)

(defn part1 [input]
  (let [sleepiest (->> (input->logs input)
                       #_(debug>> "input->logs")
                       (reduce logs-per-guard {})
                       #_(debug>> "lpg")
                       (reduce-kv sleep-per-guard [])
                       #_(debug>> "spg")
                       (apply max-key :total-sleep))
        most-sleepy (->> sleepiest
                         :days
                         (map :log)
                         (flatten)
                         (map (comp str->int :mm))
                         #_(debug>> "flatten comp")
                         (partition 2)
                         (reduce freqs-range nil)
                         #_(debug>> "freqs-range")
                         (map-indexed #(assoc {} :t %1 :f %2))
                         (apply max-key :f)
                         :t)]
    (* (str->int (:id sleepiest))
       most-sleepy)))

#_(part1 test-case)

#_(part1 (slurp (io/resource "input")))
