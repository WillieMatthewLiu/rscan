//! 作者: 拖延蟹@B1tf0rce
//! 创建日期: 2025-11-14
//! 最后修改: 2025-11-14
//! 版本: 1.0.1
//!
//! 修改记录:
//! - 2025-11-14: 异步任务资源池
use once_cell::sync::OnceCell;
use std::future::Future;
use std::sync::{Arc, Mutex};
use tokio::runtime::{Builder, Runtime};
use tokio::task::JoinHandle;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct TaskPoolConfig {
    pub worker_threads: usize,
    pub thread_name: String,
    pub enable_io: bool,
    pub enable_time: bool,
}

impl Default for TaskPoolConfig {
    fn default() -> Self {
        Self {
            worker_threads: std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(1),
            thread_name: "global-async-worker".to_string(),
            enable_io: true,
            enable_time: true,
        }
    }
}

/// 异步任务池
pub struct AsyncTaskPool {
    runtime: Arc<Runtime>,
}

impl AsyncTaskPool {
    /// 初始化全局任务池
    pub fn init(config: TaskPoolConfig) -> Result<AsyncTaskPool, Box<dyn Error>> {
        let mut builder = Builder::new_multi_thread()
            .worker_threads(config.worker_threads)
            .thread_name(&config.thread_name);

        // 然后继续使用 builder 进行后续操作
        if config.enable_io {
            builder = builder.enable_io();
        }

        if config.enable_time {
            builder = builder.enable_time();
        }

        let runtime = builder.build()?;
        Ok(AsyncTaskPool {
            runtime: Arc::new(runtime),
        })
    }

    /// 在任务池中执行异步任务
    pub fn spawn<F, T>(&self, future: F) -> JoinHandle<T>
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        self.runtime.spawn(future)
    }

    /// 执行阻塞任务
    pub fn spawn_blocking<F, T>(&self, f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        self.runtime.spawn_blocking(f)
    }

    /// 获取运行时引用
    pub fn runtime(&self) -> Arc<Runtime> {
        self.runtime.clone()
    }
}

/// 全局任务池实例
pub struct GlobalTaskPool {
    pool: OnceCell<AsyncTaskPool>,
}

impl GlobalTaskPool {
    pub fn new() -> Self {
        GlobalTaskPool {
            pool: OnceCell::new(),
        }
    }

    pub fn init(&self, config: TaskPoolConfig) -> Result<(), Box<dyn Error>> {
        let pool = AsyncTaskPool::init(config)?;
        self.pool.set(pool).map_err(|_| "Global task pool already initialized")?;
        Ok(())
    }

    pub fn get(&self) -> Result<Arc<Runtime>, &'static str> {
        self.pool
            .get()
            .map(|pool| pool.runtime())
            .ok_or("Task pool not initialized")
    }
}

/// 便捷宏
#[macro_export]
macro_rules! spawn_async {
    ($($t:tt)*) => {
        $crate::GlobalTaskPool::get().unwrap().spawn(async { $($t)* })
    };
}

// 单元测试模块
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::Duration;

    #[tokio::test] // 添加这个宏来使测试函数异步
    fn test_task_pool_init() {
        let global_task_pool = GlobalTaskPool::new();
        let config = TaskPoolConfig::default();
        global_task_pool.init(config).unwrap();
        
        let runtime = global_task_pool.get().unwrap();
        assert!(runtime.is_running());
    }

   #[tokio::test] // 添加这个宏来使测试函数异步
    async fn test_async_task_pool() -> Result<(), Box<dyn std::error::Error>> {
        // 初始化任务池
        AsyncTaskPool::init(TaskPoolConfig {
            worker_threads: 2,
            thread_name: "test-worker".to_string(),
            ..Default::default()
        })?;

        // 测试异步任务
        let handle = AsyncTaskPool::spawn(async {
            tokio::time::sleep(Duration::from_millis(100)).await;
            100
        });

        let result = handle.await?;
        assert_eq!(result, 100);

        // 测试多个任务
        let tasks: Vec<_> = (0..5)
            .map(|i| {
                AsyncTaskPool::spawn(async move {
                    tokio::time::sleep(Duration::from_millis(50)).await;
                    i * 2
                })
            })
            .collect();

        let mut results = Vec::new();
        for task in tasks {
            results.push(task.await?);
        }

        assert_eq!(results, vec![0, 2, 4, 6, 8]);
        
        Ok(())
    }


    fn get_cpus_test(){
        match std::thread::available_parallelism() {
            Ok(parallelism) => {
                println!("可用并行度: {}", parallelism);
                println!("线程数: {}", parallelism.get());
            }
            Err(e) => {
                eprintln!("获取并行度失败: {}", e);
            }
        }
    }
}