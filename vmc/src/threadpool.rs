use std::sync::mpsc::{channel, Sender};
use std::sync::Mutex;
use std::sync::Arc;

pub struct ThreadPool {
    handles: Vec<std::thread::JoinHandle<()>>,
    sender: Sender<Box<dyn Fn() + Send>>
}

impl ThreadPool {
    pub fn new(num_threads: u8) -> Self {
        let (sender, reciever) = channel::<Box<dyn Fn() + Send>>();
        let reciever = Arc::new(Mutex::new(reciever));
        let mut handles: Vec<std::thread::JoinHandle<()>> = vec![];
        for _ in 0..num_threads {
            let clone = reciever.clone();
            let handle = std::thread::spawn(move || loop {
                let work = match clone.lock().unwrap().recv(){
                    Ok(work) => work,
                    Err(_) => break,
                };
                //println!("Start {:?}", std::thread::current().id());
                work();
                //println!("End{:?}", std::thread::current().id());
                break;
            });
            handles.push(handle);
        }

        Self { handles, sender }
    }

    pub fn execute<T: Fn() + Send + 'static>(&self, work: T) { //Only borrow self in order to not "spend it" when the function is used somewhere else
        self.sender.send(Box::new(work)).unwrap();
    } 

    pub fn join_all(self) {
        for handle in self.handles {
            let _ = handle.join();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*; //pulls ThreadPool into scope
    #[test]     fn test_threadpool() {
        let pool = ThreadPool::new(3);
        pool.execute( || println!("Hello from {:?}", std::thread::current().id()));
        pool.execute( || println!("Hello from {:?}", std::thread::current().id()));
        pool.execute( || println!("Hello from {:?}", std::thread::current().id()));
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
}