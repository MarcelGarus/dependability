use super::Task;
use crate::priority_queue::PriorityQueue;
use crate::task::BehaviorWhenDeadlineMissed;
use crate::{task::TaskId, time::Timestamp};
use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use alloc::task::Wake;
use core::task::{Context, Poll, Waker};

#[cfg(feature = "std")]
use crate::time::StdTimer;
use crate::time::Timer;

#[derive(Debug)]
pub enum ExecutorError {
    MissedDeadline(u64),
}

pub struct Executor<T: Timer> {
    tasks: BTreeMap<TaskId, Task>,
    task_queue: Arc<PriorityQueue<TaskId, Timestamp>>,
    waker_cache: BTreeMap<TaskId, Waker>,
    timer: T,
}

#[cfg(feature = "std")]
impl Default for Executor<StdTimer> {
    fn default() -> Self {
        Self {
            tasks: Default::default(),
            task_queue: Arc::new(PriorityQueue::new()),
            waker_cache: Default::default(),
            timer: Default::default(),
        }
    }
}

#[cfg(feature = "std")]
impl Executor<StdTimer> {
    pub fn new() -> Executor<StdTimer> {
        Default::default()
    }
}

impl<T: Timer> Executor<T> {
    #[cfg(not(feature = "std"))]
    pub fn new(timer: T) -> Executor<T> {
        Self {
            tasks: BTreeMap::new(),
            task_queue: Arc::new(PriorityQueue::new()),
            waker_cache: BTreeMap::new(),
            timer,
        }
    }

    pub fn spawn(&mut self, task: Task) {
        let task_id = task.id;
        let deadline = self.timer.now() + task.deadline;
        if self.tasks.insert(task.id, task).is_some() {
            panic!("A task with the same ID already exists.");
        }
        self.task_queue.push(task_id, deadline);
    }

    fn run_ready_tasks(&mut self) -> Result<(), ExecutorError> {
        while let Some((task_id, _)) = self.task_queue.pop() {
            let task = match self.tasks.get_mut(&task_id) {
                Some(task) => task,
                None => continue,
            };
            let waker = self
                .waker_cache
                .entry(task_id)
                .or_insert_with(|| TaskWaker::new(task_id, task.deadline, self.task_queue.clone()));
            let mut context = Context::from_waker(waker);
            match task.poll(&mut context) {
                Poll::Ready(()) => {
                    self.tasks.remove(&task_id);
                    self.waker_cache.remove(&task_id);
                }
                Poll::Pending => {
                    let now = self.timer.now();
                    println!("Now: {} Deadline: {}", now, task.deadline);

                    if task.deadline <= now {
                        match &task.behavior {
                            BehaviorWhenDeadlineMissed::ReturnError => {
                                return Err(ExecutorError::MissedDeadline(task_id.0))
                            }
                            BehaviorWhenDeadlineMissed::ContinueRunning => {
                                self.task_queue.push(task_id, task.deadline - now);
                            }
                            BehaviorWhenDeadlineMissed::InsteadApproximate(other_task) => {
                                todo!("Spawn the other task instead.");
                            }
                        }
                    } else {
                        self.task_queue.push(task_id, task.deadline - now);
                    }
                }
            }
        }

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), ExecutorError> {
        while !self.task_queue.is_empty() {
            self.run_ready_tasks()?;
        }
        Ok(())
    }
}

struct TaskWaker {
    task_id: TaskId,
    deadline: Timestamp,
    task_queue: Arc<PriorityQueue<TaskId, Timestamp>>,
}

impl TaskWaker {
    #[allow(clippy::new_ret_no_self)]
    fn new(
        task_id: TaskId,
        deadline: Timestamp,
        task_queue: Arc<PriorityQueue<TaskId, Timestamp>>,
    ) -> Waker {
        Waker::from(Arc::new(TaskWaker {
            task_id,
            deadline,
            task_queue,
        }))
    }

    fn wake_task(&self) {
        self.task_queue.push(self.task_id, self.deadline);
    }
}

impl Wake for TaskWaker {
    fn wake(self: Arc<Self>) {
        self.wake_task();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.wake_task();
    }
}
