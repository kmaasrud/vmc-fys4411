use std::sync::mpsc::{channel, Sender};
use std::sync::Mutex;
use std::sync::Arc;

pub struct ThreadPool {
    handles: Vec<std::thread::JoinHandle<()>>,
    sender: Sender<Box<dyn Fn() + Send>>
}

impl ThreadPool {
    pub fn new(num_threads: u8) -> Self {
        let (sender, receiver) = channel::<Box<dyn Fn() + Send>>(); //Spawn a sender and receiver in order to pass instructions to threads, these has to be boxed, and cointain the trait Send in order to be able to send, dyn Fn is because the function to send must be dynamic.
        let reciever = Arc::new(Mutex::new(receiver));              //Make a mutex (which gives explicit access for muting, maybe not needed?) and then in an Arc, letting all threads access it.
        let mut handles: Vec<std::thread::JoinHandle<()>> = vec![]; //Init vector of thread handles, also called workers.
        for _ in 0..num_threads {                                   //Looping over threads
            let clone = reciever.clone();                           //The receiver needs to be cloned in order to let the next thread in the loop clone it again
            let handle = std::thread::spawn(move || loop {          //This block basically spawns the thread, move means moving ownerhsip of all inside thread.
                let work = match clone.lock().unwrap().recv(){      //Defining the work, if Ok, then good, if Err, break the thread.
                    Ok(work) => work,
                    Err(_) => break,
                };
                //println!("Start {:?}", std::thread::current().id());
                work();                                             //Doing the work
                break;                                              //Kill thread when done
            });
            handles.push(handle);                                   //This is ofc done before work() is completed, so the handle is added to vec of handles!
        }

        Self { handles, sender }                                    //Returning self in order to grab it from execute() and join_all()
    }

    pub fn execute<T: Fn() + Send + 'static>(&self, work: T) {      //Only borrow self in order to not "spend it" when the function is used somewhere else
        self.sender.send(Box::new(work)).unwrap();                  //Sends workload to the worker
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