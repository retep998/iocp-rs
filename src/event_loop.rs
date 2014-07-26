
use std::mem;
use std::rt::exclusive::Exclusive;
use std::rt::rtio;
use std::sync::Arc;
use std::sync::atomics::{AtomicBool, SeqCst};

use io;

pub struct Loop {
    factory: io::Factory,
    work: Vec<proc(): Send>,
    remotes: Vec<(uint, Box<rtio::Callback + Send>)>,
    next_remote: uint,
    messages: Arc<Exclusive<Vec<Message>>>,
    idle: Option<Box<rtio::Callback + Send>>,
    idle_active: Option<Arc<AtomicBool>>,
}

impl Loop {
    pub fn new() -> Loop {
        Loop {
            factory: io::Factory::new(),
            work: vec![],
            idle: None,
            idle_active: None,
            next_remote: 0,
            remotes: vec![],
            messages: Arc::new(Exclusive::new(Vec::new())),
        }
    }
    fn work(&mut self) {
        while self.work.len() > 0 {
            for work in mem::replace(&mut self.work, vec![]).move_iter() {
                work();
            }
        }
    }
    fn remote_work(&mut self) {
        let messages = unsafe {
            mem::replace(&mut *self.messages.lock(), Vec::new())
        };
        for message in messages.move_iter() {
            self.message(message);
        }
    }
    fn message(
        &mut self,
        message: Message
    ) {
        match message {
            RunRemote(i) => {
                match self.remotes.mut_iter().find(|& &(id, _)| id == i) {
                    Some(&(_, ref mut f)) => f.call(),
                    None => unreachable!()
                }
            }
            RemoveRemote(i) => {
                match self.remotes.iter().position(|&(id, _)| id == i) {
                    Some(i) => { self.remotes.remove(i).unwrap(); }
                    None => unreachable!()
                }
            }
        }
    }
    fn idle(&mut self) {
        match self.idle {
            Some(ref mut idle) => {
                if self.idle_active.get_ref().load(SeqCst) {
                    idle.call();
                }
            }
            None => {}
        }
    }
    fn has_idle(&self) -> bool {
        self.idle.is_some() && self.idle_active.get_ref().load(SeqCst)
    }
}

impl rtio::EventLoop for Loop {
    fn run(&mut self) {
        while self.remotes.len() > 0 || self.work.len() > 0 || self.has_idle() {
            self.work();
            self.remote_work();
            if self.has_idle() {
                self.idle();
                continue
            }
            unsafe {
                let mut messages = self.messages.lock();
                if self.remotes.len() > 0 && messages.len() == 0 &&
                   self.work.len() == 0 {
                    messages.wait()
                }
            }
        }
    }
    fn callback(
        &mut self,
        cb: proc(): Send
    ) {
        self.work.push(cb);
    }
    fn pausable_idle_callback(
        &mut self,
        cb: Box<rtio::Callback + Send>
    ) -> Box<rtio::PausableIdleCallback + Send> {
        iocpassert!(self.idle.is_none());
        self.idle = Some(cb);
        let a = Arc::new(AtomicBool::new(true));
        self.idle_active = Some(a.clone());
        box BasicPausable { active: a } as Box<rtio::PausableIdleCallback + Send>
    }
    fn remote_callback(
        &mut self,
        cb: Box<rtio::Callback + Send>
    ) -> Box<rtio::RemoteCallback + Send> {
        let id = self.next_remote;
        self.next_remote += 1;
        self.remotes.push((id, cb));
        box BasicRemote::new(self.messages.clone(), id) as
            Box<rtio::RemoteCallback + Send>
    }
    fn io<'a>(
        &'a mut self
    ) -> Option<&'a mut rtio::IoFactory> {
        Some(&mut self.factory as &mut rtio::IoFactory)
    }
    fn has_active_io(&self) -> bool {
        false
    }
}

enum Message {
    RunRemote(uint),
    RemoveRemote(uint)
}

struct BasicRemote {
    queue: Arc<Exclusive<Vec<Message>>>,
    id: uint,
}

impl BasicRemote {
    fn new(
        queue: Arc<Exclusive<Vec<Message>>>,
        id: uint
    ) -> BasicRemote {
        BasicRemote { queue: queue, id: id }
    }
}

impl rtio::RemoteCallback for BasicRemote {
    fn fire(&mut self) {
        let mut queue = unsafe { self.queue.lock() };
        queue.push(RunRemote(self.id));
        queue.signal();
    }
}

impl Drop for BasicRemote {
    fn drop(&mut self) {
        let mut queue = unsafe { self.queue.lock() };
        queue.push(RemoveRemote(self.id));
        queue.signal();
    }
}

struct BasicPausable {
    active: Arc<AtomicBool>,
}

impl rtio::PausableIdleCallback for BasicPausable {
    fn pause(&mut self) {
        self.active.store(false, SeqCst);
    }
    fn resume(&mut self) {
        self.active.store(true, SeqCst);
    }
}

impl Drop for BasicPausable {
    fn drop(&mut self) {
        self.active.store(false, SeqCst);
    }
}
