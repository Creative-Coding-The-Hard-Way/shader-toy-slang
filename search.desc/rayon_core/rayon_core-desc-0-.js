searchState.loadedDescShard("rayon_core", 0, "Rayon-core houses the core stable APIs of Rayon.\nProvides context to a closure called by <code>broadcast</code>.\nContains the rayon thread pool configuration. Use …\nWork was found and executed.\nProvides the calling context to a closure called by …\nNo available work was found.\nRepresents a fork-join scope which can be used to spawn …\nRepresents a fork-join scope which can be used to spawn …\nThread builder used for customization via …\nRepresents a user created thread-pool.\nError when initializing a thread pool.\nUsed to create a new <code>ThreadPool</code> or to configure the global …\nResult of <code>yield_now()</code> or <code>yield_local()</code>.\n<strong>(DEPRECATED)</strong> Suggest to worker threads that they execute …\nDeprecated in favor of <code>ThreadPoolBuilder::breadth_first</code>.\nExecutes <code>op</code> within every thread in the current threadpool. …\nExecutes <code>op</code> within every thread in the threadpool. Any …\nCreates a new <code>ThreadPool</code> initialized using this …\nDeprecated in favor of <code>ThreadPoolBuilder::build</code>.\nInitializes the global thread pool. This initialization is …\nCreates a scoped <code>ThreadPool</code> initialized using this …\nReturns the number of threads in the current registry. If …\nReturns the (current) number of threads in the thread pool.\nIf called from a Rayon worker thread, indicates whether …\nReturns true if the current worker thread currently has “…\nIf called from a Rayon worker thread, returns the index of …\nIf called from a Rayon worker thread in this thread-pool, …\nSets a callback to be invoked on thread exit.\nDeprecated in favor of <code>ThreadPoolBuilder::exit_handler</code>.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCreates a “fork-join” scope <code>s</code> and invokes the closure …\nCreates a scope that spawns work into this thread-pool.\nCreates a “fork-join” scope <code>s</code> with FIFO order, and …\nCreates a scope that spawns work into this thread-pool in …\nOur index amongst the broadcast threads (ranges from …\nGets the index of this thread in the pool, within …\nDeprecated in favor of <code>ThreadPoolBuilder::build_global</code>.\nExecutes <code>op</code> within the threadpool. Any attempts to use <code>join</code>…\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nTakes two closures and <em>potentially</em> runs them in parallel. …\nExecute <code>oper_a</code> and <code>oper_b</code> in the thread-pool and return …\nIdentical to <code>join</code>, except that the closures have a …\nReturns the maximum number of threads that Rayon supports …\nReturns <code>true</code> if the closure was called from a different …\nGets the string that was specified by …\nDeprecated in favor of <code>ThreadPoolBuilder::build</code>.\nCreates and returns a valid rayon thread pool builder, but …\nCreates and return a valid rayon thread pool …\nThe number of threads receiving the broadcast in the …\nSets the number of threads to be used in the rayon …\nDeprecated in favor of <code>ThreadPoolBuilder::num_threads</code>.\nNormally, whenever Rayon catches a panic, it tries to …\nDeprecated in favor of <code>ThreadPoolBuilder::panic_handler</code>.\nExecutes the main loop for this thread. This will not …\nCreates a “fork-join” scope <code>s</code> and invokes the closure …\nCreates a scope that executes within this thread-pool. …\nCreates a “fork-join” scope <code>s</code> with FIFO order, and …\nCreates a scope that executes within this thread-pool. …\nPuts the task into the Rayon threadpool’s job queue in …\nSpawns a job into the fork-join scope <code>self</code>. This job will …\nSpawns an asynchronous task in this thread-pool. This task …\nSpawns an asynchronous task on every thread in this …\nSpawns a job into every thread of the fork-join scope <code>self</code>…\nSpawns a job into every thread of the fork-join scope <code>self</code>…\nSpawns an asynchronous task on every thread in this …\nFires off a task into the Rayon threadpool in the “static…\nSpawns a job into the fork-join scope <code>self</code>. This job will …\nSpawns an asynchronous task in this thread-pool. This task …\nSets a custom function for spawning threads.\nGets the value that was specified by …\nSets the stack size of the worker threads\nDeprecated in favor of <code>ThreadPoolBuilder::stack_size</code>.\nSets a callback to be invoked on thread start.\nDeprecated in favor of <code>ThreadPoolBuilder::start_handler</code>.\nSets a closure which takes a thread index and returns the …\nDeprecated in favor of <code>ThreadPoolBuilder::thread_name</code>.\nUse the current thread as one of the threads in the pool.\nCooperatively yields execution to local Rayon work.\nCooperatively yields execution to local Rayon work.\nCooperatively yields execution to Rayon.\nCooperatively yields execution to Rayon.")