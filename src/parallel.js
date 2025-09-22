/**
 * Package Fast Parallel Processing Utilities
 * Tools for parallel execution of tasks with controlled concurrency
 */

const os = require('os');

class TaskQueue {
  constructor(options = {}) {
    this.concurrency = options.concurrency || Math.min(16, os.cpus().length + 2);
    this.tasks = [];
    this.running = [];
    this.results = [];
    this.errors = [];
    this.completed = false;
  }

  add(task) {
    if (this.completed) {
      throw new Error('Cannot add tasks to a completed queue');
    }
    this.tasks.push(task);
  }

  async run() {
    if (this.completed) {
      throw new Error('Cannot run an already completed queue');
    }
    
    // Process tasks in batches to reduce overhead
    while (this.tasks.length > 0 || this.running.length > 0) {
      // Start new tasks up to concurrency limit
      while (this.running.length < this.concurrency && this.tasks.length > 0) {
        const task = this.tasks.shift();
        const promise = this.executeTask(task)
          .then(result => {
            this.results.push(result);
            // Remove from running
            const index = this.running.indexOf(promise);
            if (index > -1) {
              this.running.splice(index, 1);
            }
          })
          .catch(error => {
            this.errors.push(error);
            // Remove from running
            const index = this.running.indexOf(promise);
            if (index > -1) {
              this.running.splice(index, 1);
            }
          });
        
        this.running.push(promise);
      }
      
      // Wait for at least one task to complete
      if (this.running.length > 0) {
        await Promise.race(this.running);
      }
    }
    
    this.completed = true;
    
    if (this.errors.length > 0) {
      const error = new Error(`Task queue completed with ${this.errors.length} errors`);
      error.errors = this.errors;
      throw error;
    }
    
    return this.results;
  }

  async executeTask(task) {
    if (typeof task === 'function') {
      return await task();
    } else if (task && typeof task.execute === 'function') {
      return await task.execute();
    } else {
      throw new Error('Invalid task format');
    }
  }
}

class WorkerPool {
  constructor(options = {}) {
    this.concurrency = options.concurrency || Math.min(16, os.cpus().length + 2);
    this.workers = [];
    this.idleWorkers = [];
    this.taskQueue = [];
  }

  async execute(task) {
    return new Promise((resolve, reject) => {
      this.taskQueue.push({ task, resolve, reject });
      this.processQueue();
    });
  }

  async processQueue() {
    if (this.taskQueue.length === 0) return;
    if (this.idleWorkers.length === 0 && this.workers.length >= this.concurrency) return;
    
    const { task, resolve, reject } = this.taskQueue.shift();
    
    let worker;
    if (this.idleWorkers.length > 0) {
      worker = this.idleWorkers.pop();
    } else {
      worker = new Worker();
      this.workers.push(worker);
    }
    
    worker.execute(task)
      .then(resolve)
      .catch(reject)
      .finally(() => {
        this.idleWorkers.push(worker);
        this.processQueue();
      });
  }

  async destroy() {
    await Promise.all(this.workers.map(worker => worker.destroy()));
    this.workers = [];
    this.idleWorkers = [];
    this.taskQueue = [];
  }
}

class Worker {
  async execute(task) {
    // In a real implementation, this would use actual worker threads
    // For now, we'll just execute the task directly
    if (typeof task === 'function') {
      return await task();
    } else {
      throw new Error('Invalid task format');
    }
  }

  async destroy() {
    // Clean up worker resources
  }
}

// Utility function for parallel map
async function parallelMap(array, mapper, options = {}) {
  const concurrency = options.concurrency || Math.min(16, os.cpus().length + 2);
  const results = new Array(array.length);
  
  // Create task queue
  const queue = new TaskQueue({ concurrency });
  
  // Add all mapping tasks
  array.forEach((item, index) => {
    queue.add(async () => {
      const result = await mapper(item, index, array);
      results[index] = result;
      return result;
    });
  });
  
  await queue.run();
  return results;
}

// Utility function for parallel filter
async function parallelFilter(array, predicate, options = {}) {
  const mapped = await parallelMap(array, predicate, options);
  return array.filter((_, index) => mapped[index]);
}

// Utility function for parallel reduce
async function parallelReduce(array, reducer, initialValue, options = {}) {
  // For reduce, we need to be more careful about order
  // We'll chunk the array and reduce each chunk in parallel, then reduce the results
  
  const concurrency = options.concurrency || Math.min(16, os.cpus().length + 2);
  const chunkSize = Math.ceil(array.length / concurrency);
  
  // Create chunks
  const chunks = [];
  for (let i = 0; i < array.length; i += chunkSize) {
    chunks.push(array.slice(i, i + chunkSize));
  }
  
  // Reduce each chunk in parallel
  const chunkResults = await Promise.all(chunks.map(chunk => 
    chunk.reduce(reducer, undefined)
  ));
  
  // Reduce the chunk results sequentially
  return chunkResults.reduce(reducer, initialValue);
}

module.exports = {
  TaskQueue,
  WorkerPool,
  parallelMap,
  parallelFilter,
  parallelReduce
};