extern crate clipboard;
extern crate reqwest;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use std::time::Duration;
use std::collections::HashMap;
use reqwest::Client;
use clokwerk::{Scheduler, TimeUnits};
use std::thread;


fn main()  {
    let mut scheduler = Scheduler::new();

    let mut clipboard_context: ClipboardContext = ClipboardProvider::new().unwrap();
    let mut last_content: String = clipboard_context.get_contents().unwrap_or_default();
    let mut counter = 0;

    let clipboard_url = "https://jessestolwijk.builtwithdark.com/clipboard";
    let device_id = "2a8672cd-a2ce-4afa-8a89-bfa2e64ad4b8";

    scheduler.every(1.seconds()).run(move || {
        if counter % 5 == 0 {
            let reponse = reqwest::get(&format!("{}{}{}", clipboard_url, "?device_id=", device_id));
            
            match reponse {
                Ok(mut r) => { 
                    last_content = r.text().unwrap_or_default();
                    clipboard_context.set_contents(last_content.clone()); 
                    () 
                } 
                Err(_) => println!("Unable to retrieve response")
            }
                   
        }
        
        counter+=1;

        let clipboard_content = clipboard_context.get_contents().unwrap_or_default();
        if clipboard_content != last_content {
            println!("New data detected! {:?}", clipboard_content);
            last_content = clipboard_content;

            share_clipboard(&clipboard_url, &last_content, &device_id);
        }   
    });

    loop {
        scheduler.run_pending();
        thread::sleep(Duration::from_millis(100));
    }
}



fn share_clipboard(clipboard_url: &str, clipboard: &str, device_id: &str) {
    let mut map = HashMap::new();
    map.insert("device_id", device_id);
    map.insert("content", clipboard);

    Client::new().post(clipboard_url)
        .json(&map)
        .send();

    // TODO handle errors
}