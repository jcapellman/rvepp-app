use std::env;
use crate::ProtectionLayer;

extern crate inotify;

use inotify::{
    EventMask,
    WatchMask,
    Inotify,
};

pub struct RTFM {
    inotify: Inotify
}

impl RTFM {}

impl ProtectionLayer for RTFM {

    fn initialize(&self) {
        inotify = Inotify::init()
            .expect("Failed to initialize inotify");

        let current_dir = env::current_dir()
            .expect("Failed to determine current directory");

        inotify
            .watches()
            .add(
                current_dir,
                WatchMask::MODIFY | WatchMask::CREATE | WatchMask::DELETE,
            )
            .expect("Failed to add inotify watch");
    }

    fn shutdown(&self) {
        // Inotify
    }

    fn run(&self) {
        let mut buffer = [0u8; 4096];

        loop {
            let events = inotify
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
    }
}