use std::env;
use std::thread;
use crate::ProtectionLayer;
use rvepp_ml::MachineLearningModel;
use rvepp_ml::ml_elf_model::ElfModel;
extern crate inotify;

use inotify::{
    EventMask,
    WatchMask,
    Inotify,
};

pub struct Rtfm {

}

impl ProtectionLayer for Rtfm {
    fn initialize(&mut self) {
        let model = ElfModel { };

        model.init();

        
        thread::spawn(|| {
            let mut pl_inotify = Inotify::init()
                .expect("Failed to initialize inotify");

            let current_dir = env::current_dir()
                .expect("Failed to determine current directory");

            pl_inotify
                .watches()
                .add(
                    current_dir,
                    WatchMask::MODIFY | WatchMask::CREATE | WatchMask::DELETE,
                )
                .expect("Failed to add inotify watch");

            let mut buffer = [0u8; 4096];

            loop {
                let events = pl_inotify
                    .read_events_blocking(&mut buffer)
                    .expect("Failed to read inotify events");

                for event in events {
                    if event.mask.contains(EventMask::CREATE) {
                        if event.mask.contains(EventMask::ISDIR) {
                            println!("Directory created: {:?}", event.name);
                        } else {
                            println!("File created: {:?}", event.name);
                        }
                    } else if event.mask.contains(EventMask::DELETE) {
                        if event.mask.contains(EventMask::ISDIR) {
                            println!("Directory deleted: {:?}", event.name);
                        } else {
                            println!("File deleted: {:?}", event.name);
                        }
                    } else if event.mask.contains(EventMask::MODIFY) {
                        if event.mask.contains(EventMask::ISDIR) {
                            println!("Directory modified: {:?}", event.name);
                        } else {
                            println!("File modified: {:?}", event.name);
                        }
                    }
                }
            }
        });
    }

    fn shutdown(&mut self) {
        // Inotify
    }

    fn run(&mut self) {

    }
}