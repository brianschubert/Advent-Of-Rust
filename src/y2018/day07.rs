//! Solution for Advent of Code [2018 Day 07](https://adventofcode.com/2018/day/7).

use crate::common::puzzle;
use std::collections::HashSet;
use std::str::FromStr;

/// The maximum number of steps that can exist.
const MAX_STEP_COUNT: usize = 26;

/// The number of elves available to help, plus one.
///
/// For Part 2.
const WORKER_COUNT: usize = 5;

/// The base amount of time each step takes to complete.
///
/// For Part 2.
const BASE_STEP_DURATION: Second = 60;

/// Integral type used to represent time in seconds.
///
/// For Part 2.
type Second = u32;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// A step identifier that identifies a step. It's the ID of a step - a Step ID.
struct StepID(u8);

#[derive(Debug, Copy, Clone)]
/// A description of a single dependency between two steps.
struct DependencyEntry {
    target: StepID,
    requires: StepID,
}

#[derive(Debug, Clone, Copy)]
/// A worker that can work on a step for a duration of time.
///
/// For Part 2.
struct Worker {
    task: Option<StepID>,
    time_left: Second,
}

#[derive(Clone)]
/// A `StepSimulator` simulates the process of completing a sequence
/// of dependent steps.
///
/// Solves both parts of the problem.
struct StepSimulator {
    /// A mapping between `StepID`s and their unsatisfied dependencies.
    ///
    /// `Some(HashSet)` at position `i` indicates that the step with
    /// ID `(i + b'A') as char` is waiting for the steps in the hash
    /// set to be completed before it can be begun.
    ///
    /// `None` at position `i` indicates that the step with ID
    /// `(i + b'A') as char` has already been started or completed
    /// and need not be considered when determining the next step to complete.
    requirements_map: [Option<HashSet<StepID>>; MAX_STEP_COUNT],

    /// The steps that have been completed so far in the simulation.
    ///
    /// Used for both parts.
    completed_steps: Vec<StepID>,

    /// The workers for Part 2.
    workers: Vec<Worker>,

    /// The current time in the simulation for Part 2.
    current_time: Second,

    /// The base amount of time each step takes to complete.
    ///
    /// For Part 2.
    base_step_duration: Second,
}

impl StepID {
    /// Converts the specified ASCII byte into a step ID.
    ///
    /// Panics if the given byte is no an ASCII uppercase letter.
    fn from_ascii_unstable(b: u8) -> Self {
        if !b.is_ascii_uppercase() {
            panic!("StepID must be a valid ASCII uppercase letter");
        }
        Self(b - b'A')
    }

    /// Converts the specified letter index into a step ID.
    ///
    /// Panics if the given index is not in `0..26`.
    fn from_letter_index(i: usize) -> Self {
        if i > 25 {
            panic!("StepID must be a valid ASCII uppercase letter");
        };
        Self(i as u8)
    }

    #[inline]
    /// Returns this step ID as a letter index.
    fn as_index(self) -> usize {
        self.0 as usize
    }

    #[inline]
    /// Returns this step ID as an ASCII uppercase letter.
    fn as_char(self) -> char {
        (self.0 + b'A') as char
    }

    #[inline]
    /// Returns the time that it takes to complete this step.
    ///
    /// For Part 2.
    fn duration(self) -> Second {
        Second::from(self.0) + 1
    }
}

impl FromStr for DependencyEntry {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const REQUIREMENT_POS: usize = 5;
        const TARGET_POS: usize = 36;
        let bytes = s.as_bytes();

        let requires = StepID::from_ascii_unstable(bytes[REQUIREMENT_POS]);
        let target = StepID::from_ascii_unstable(bytes[TARGET_POS]);

        Ok(Self { target, requires })
    }
}

impl Worker {
    /// Updates this worker to begin working to the specified step.
    fn work_on(&mut self, task: StepID) {
        self.time_left = task.duration();
        self.task.replace(task);
    }

    /// Reduces this workers time until completion by the specified
    /// number of seconds.
    fn work_for(&mut self, time: Second) {
        if self.time_left >= time {
            self.time_left -= time;
        }
    }

    #[inline]
    /// Returns `true` if this work has finished its step or is not working
    /// on a task.
    fn is_idle(self) -> bool {
        self.time_left == 0
    }

    #[inline]
    /// Returns `true` is this worker is currently assigned to a task, even
    /// if said task is finished.
    fn has_task(self) -> bool {
        self.task.is_some()
    }
}

impl Default for Worker {
    fn default() -> Self {
        Self {
            task: None,
            time_left: 0,
        }
    }
}

impl StepSimulator {
    /// Constructs a `StepSimulator` from the given list of step dependencies
    /// with the specified number of workers.
    fn new(steps: &[DependencyEntry], worker_count: usize, base_step_duration: Second) -> Self {
        let mut requirements_map: [Option<HashSet<StepID>>; MAX_STEP_COUNT] = Default::default();
        for &step in steps {
            let req_set = requirements_map[step.target.as_index()].get_or_insert_with(HashSet::new);
            req_set.insert(step.requires);
            // Ensure the step with no dependencies is `Some` with an empty `HashSet` rather than `None`
            requirements_map[step.requires.as_index()].get_or_insert_with(HashSet::new);
        }
        Self {
            requirements_map,
            completed_steps: Vec::new(),
            workers: vec![Default::default(); worker_count],
            current_time: 0,
            base_step_duration,
        }
    }

    /// Compute the order in which the steps may be completed.
    ///
    /// Solves Part 1 of the problem.
    fn compute_timeless_step_order(mut self) -> Vec<StepID> {
        while let Some(next_step) = self.ready_steps().iter().copied().min() {
            self.begin_step(next_step);
            self.complete_step(next_step);
        }
        self.completed_steps
    }

    /// Compute how long it will take for this simulators workers to complete
    /// all of the steps.
    ///
    /// Returns 1) the order in which the steps where completed, and 2) the
    /// total time required to finish said steps.
    ///
    /// Solves Part 2 of the problem.
    fn simulate_tasks_brute_force(mut self) -> (Vec<StepID>, Second) {
        loop {
            // Fetch all steps whose requirements have been satisfied and
            // who have yet to be started by a worker.
            let ready_steps: Vec<StepID> = self.ready_steps();

            if !ready_steps.is_empty() {
                // Make a copy of this simulator's base_step_duration, since
                // we are about to borrow self mutable for the rest of this
                // branch
                let base_step_duration = self.base_step_duration;
                // Check if there is a worker available to begin one of the ready
                // steps
                let free_worker: Option<&mut Worker> = self.find_free_worker_mut();
                match free_worker {
                    Some(worker_ref) => {
                        // A worker is free, and a step is ready to be completed
                        let ready_step = *ready_steps.first().unwrap();
                        worker_ref.work_on(ready_step);
                        worker_ref.time_left += base_step_duration;
                        self.begin_step(ready_step);
                    }
                    None => {
                        // Steps are ready to be started, but all of the workers
                        // are busy, so wait until one finishes their step.
                        let worker_near_completion = self.worker_nearest_completion_mut().unwrap();
                        let finished_step = worker_near_completion.task.take().unwrap();
                        let time = worker_near_completion.time_left;
                        self.wait(time);
                        self.complete_step(finished_step);
                    }
                }
            } else {
                // No steps are ready to be started, so wait until a worker
                // finishes their step
                match self.worker_nearest_completion_mut() {
                    Some(worker_ref) => {
                        let finished_step = worker_ref.task.take().unwrap();
                        let time = worker_ref.time_left;
                        self.wait(time);
                        self.complete_step(finished_step);
                    }
                    None => {
                        // No tasks are ready to be started, but no workers
                        // are currently working on a task
                        if self.requirements_map.iter().all(|step| step.is_none()) {
                            // All tasks have been completed. Return to caller.
                            break;
                        } else {
                            panic!("simulation failed: none of the remaining steps are ready, but all workers are idle - perhaps a cyclic dependency exists in the steps?")
                        }
                    }
                }
            }
        }
        (self.completed_steps, self.current_time)
    }

    /// Returns the steps whose requirements have been satisfied and who have
    /// yet to be begun.
    ///
    /// For both parts.
    fn ready_steps(&self) -> Vec<StepID> {
        self.requirements_map
            .iter()
            .enumerate()
            // Map (usize, &Option<HashSet>) to (usize, &HashSet)
            .filter_map(|(i, maybe_req)| maybe_req.as_ref().map(|req_set| (i, req_set)))
            // Find all the sets that have no unsatisfied dependencies
            .filter(|&(_, req_set)| req_set.is_empty())
            .map(|(letter_index, _)| StepID::from_letter_index(letter_index))
            .collect()
    }

    /// Remove the specified step from the steps ready to be begun.
    ///
    /// For both parts.
    fn begin_step(&mut self, step: StepID) {
        self.requirements_map[step.as_index()] = None;
    }

    /// Marks the specified step as being completed.
    ///
    /// For both parts.
    fn complete_step(&mut self, step: StepID) {
        for step_reqs in self.requirements_map.iter_mut() {
            if let Some(req_set) = step_reqs {
                req_set.remove(&step);
            }
        }
        self.completed_steps.push(step)
    }

    /// Returns a mutable reference to a worker that is not currently working
    /// on a step, if such as worker exists.
    ///
    /// For Part 2.
    fn find_free_worker_mut(&mut self) -> Option<&mut Worker> {
        self.workers.iter_mut().find(|&&mut w| w.is_idle())
    }

    /// Fast-forwards this simulators current time while updating each
    /// worker's time until completion.
    ///
    /// For Part 2.
    fn wait(&mut self, time: Second) {
        self.current_time += time;
        for worker in self.workers.iter_mut() {
            worker.work_for(time);
        }
    }

    /// Returns a mutable reference to the worker that has the least
    /// amount of time until they complete their step, if such a worker
    /// exists.
    ///
    /// For Part 2.
    fn worker_nearest_completion_mut(&mut self) -> Option<&mut Worker> {
        self.workers
            .iter_mut()
            .filter(|&&mut w| w.has_task())
            .min_by_key(|&&mut w| w.time_left)
    }
}

pub fn solve(puzzle: &puzzle::Selection) -> puzzle::Result {
    let input = puzzle::fetch_lines(puzzle)?;
    let dependencies: Vec<DependencyEntry> = input.iter().map(|s| s.parse().unwrap()).collect();

    let simulator = StepSimulator::new(&dependencies, WORKER_COUNT, BASE_STEP_DURATION);

    solve_parts!(
        1 => simulator
            .clone()
            .compute_timeless_step_order()
            .into_iter()
            .map(StepID::as_char)
            .collect::<String>(),
        2 => simulator.simulate_tasks_brute_force().1,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_STEP_DESCRIPTION: &[&str] = &[
        "Step C must be finished before step A can begin.",
        "Step C must be finished before step F can begin.",
        "Step A must be finished before step B can begin.",
        "Step A must be finished before step D can begin.",
        "Step B must be finished before step E can begin.",
        "Step D must be finished before step E can begin.",
        "Step F must be finished before step E can begin.",
    ];
    const EXAMPLE_WORKER_COUNT: usize = 2;

    const EXAMPLE_STEP_DURATION: Second = 0;

    #[test]
    fn solution() {
        assert_solution!(
            "CQSWKZFJONPBEUMXADLYIGVRHT",
            914,
            puzzle::Selection::new(2018, 7)
        )
    }

    #[test]
    fn ex1() {
        let dependencies: Vec<DependencyEntry> = EXAMPLE_STEP_DESCRIPTION
            .iter()
            .map(|s| s.parse().unwrap())
            .collect();
        let step_order: String =
            StepSimulator::new(&dependencies, EXAMPLE_WORKER_COUNT, EXAMPLE_STEP_DURATION)
                .compute_timeless_step_order()
                .into_iter()
                .map(StepID::as_char)
                .collect();
        assert_eq!(step_order, "CABDFE");
    }

    #[test]
    fn ex2() {
        let dependencies: Vec<DependencyEntry> = EXAMPLE_STEP_DESCRIPTION
            .iter()
            .map(|s| s.parse().unwrap())
            .collect();
        let mut simulator =
            StepSimulator::new(&dependencies, EXAMPLE_WORKER_COUNT, EXAMPLE_STEP_DURATION);
        simulator.base_step_duration = 0;
        let completion_time = simulator.simulate_tasks_brute_force().1;
        assert_eq!(completion_time, 15)
    }
}
