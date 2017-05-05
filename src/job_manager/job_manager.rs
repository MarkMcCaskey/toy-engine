//! The system for taking tasks, breaking them up and executing them
//! acts as a blackbox.  Tasks are given with (possibly multidimensional) priority and the job manager
//! does its best to schedule and execute theam

//! The system will be able to either pass back messages requesting that other systems reduce the
//! amount of work being sent (i.e. lower the LoD) or it will do this itself

//! It will include (possibly optional) run-time profiling to accomplish and provide data needed
//! to further optimize the engine

use futures::{Future, Stream};
use tokio_core::reactor::Core;
use std::collections::HashMap;


pub struct JobManager {
    /// a custom structure will likely be faster
    /// this has the benefit of being dynamic, which is useful for now
    job_list: HashMap<JobPriority, Vec<Job>>,
}

struct Job {

}

/// Used to determine how important jobs are,
/// currently only a rough outline and single-dimensional
/// Ideally it should be much finer grained than this
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum JobPriority {

    /// The highest priority.  If the job can't be done, it should be treated like a `panic!`
    /// i.e., if the job can't be done, it's better to crash than attempt to continue
    /// This will likely be rare, and will be able to be treated as `SubCritical` in production
    Critical,

    /// Like `Critical` but failure to accomplish the task will likely only cause massive inconvience
    /// or serious problems
    SubCritical,

    /// The task must be completed before the next Frame
    Frame,

    /// The task must be done roughly within a second 
    SubSecond,
    /// Long-term, the task needs to be done but can be delayed for a while
    LongTerm,

    /// Failure to accomplish task is fully acceptable and has very little impact on the game
    Optional,
}

impl JobManager {
    pub fn new() -> JobManager {
        JobManager {
            job_list: HashMap::new(),
        }
    }

    pub fn run_jobs() {
        
    }
}
