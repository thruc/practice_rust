use std::thread;
//use std::rc::Rc;
//use std::sync::Arc;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;

/* Hello, world!は表示される前にプログラム終了する
fn main() {
    thread::spawn(|| {
        println!("Hello, world!");
    });
}
*/

/* closure may outlive the current function, but it borrows `x`, which is owned by the current function
fn main() {
    let mut handeles = Vec::new();

        for x in 0..10 {
            handeles.push(thread::spawn(|| {
                println!("Hello, world!{}", x);

            }));
        }

        for handele in handeles {
            let _ = handele.join();
        }

}
*/

/*use of moved value: `data`
fn main() {
let mut handeles = Vec::new();
        let mut data = vec![1; 10];

        // move を使用した所有権をスレッドに渡す
        for x in 0..10 {
            handeles.push(thread::spawn(move || {
                data[x] += 1;
            }));
        }

        for handele in handeles {
            let _ = handele.join();
        }
    }
*/
/*cannot be sent between threads safely
fn main() {

    // 共有メモリ

    let mut handeles = Vec::new();
    let mut data = Rc::new(vec![1; 10]);

    // move を使用した所有権をスレッドに渡す
    for x in 0..10 {
        let data_ref = data.clone();
        handeles.push(thread::spawn(move || {
            data_ref[x] += 1;
        }));
    }

    for handele in handeles {
        let _ = handele.join();
    }
    
}
*/
/*cannot borrow data in an `Arc` as mutabl
fn main() {

    let mut handeles = Vec::new();
    let mut data = Arc::new(vec![1; 10]);

    // move を使用した所有権をスレッドに渡す
    for x in 0..10 {
        let data_ref = data.clone();
        handeles.push(thread::spawn(move || {
            data_ref[x] += 1;
        }));
    }

    for handele in handeles {
        let _ = handele.join();
    }
    
}
*/
fn main() {
    //スレッド
    {
        let handle = thread::spawn(|| {
            println!("Hello, world!");
        });

        dbg!(handle.join());
        let mut handeles = Vec::new();

        // move を使用した所有権をスレッドに渡す
        for x in 0..10 {
            handeles.push(thread::spawn(move || {
                println!("Hello, world!{}", x);
            }));
        }

        for handele in handeles {
            let _ = handele.join();
        }
    }
    // 共有メモリ
    {
        let mut handeles = Vec::new();
        let data = Arc::new(Mutex::new(vec![1; 10]));

        // move を使用した所有権をスレッドに渡す
        for x in 0..10 {
            let data_ref = data.clone();
            handeles.push(thread::spawn(move || {
                let mut data = data_ref.lock().unwrap();
                data[x] += 1;
            }));
        }

        for handele in handeles {
            let _ = handele.join();
        }
        dbg!(data);
    }
    // メッセージパッシング
    {
        let (tx, rx) = mpsc::channel();
        let handle = thread::spawn(move || {
            let data = rx.recv().unwrap();
            println!("{}", data);
        });

        let _ = tx.send("hello, word");

        let _ = handle.join();
    }
    {
        let mut handeles = Vec::new();
        let mut data = vec![1; 10];
        let mut send_channels = Vec::new();
        let mut rcv_channels = Vec::new();

        for _ in 0..10 {
            let (send_tx, send_rx) = mpsc::channel();
            let (rcv_tx, rcv_rx) = mpsc::channel();
            send_channels.push(send_tx);
            rcv_channels.push(rcv_rx);

            handeles.push(thread::spawn(move || {
                let mut data = send_rx.recv().unwrap();
                data += 1;
                let _ = rcv_tx.send(data);
            }))
            
        }
        for x in 0..10 {
            let _  = send_channels[x].send(data[x]);
        }

        for x in 0..10 {
            data[x] = rcv_channels[x].recv().unwrap();
        }

        for handele in handeles {
            let _ = handele.join();
        }
        
        dbg!(data);

    }
}
