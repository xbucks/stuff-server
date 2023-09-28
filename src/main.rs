use std::io::Read;
use std::io::Write;
use std::net::TcpStream;
use std::net::TcpListener;

fn print(bytes: &[u8]) {
    match std::str::from_utf8(bytes) {
        Ok(string) => { println!("PRINT {}", string); }
        Err(_) => { println!("PRINT ERROR") }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut receive_buffer = [0; 4098];

    println!("** STREAM START **");
    loop {
        match stream.read(&mut receive_buffer) {
            Ok(received_size) => {
                if received_size == 0 {
                    return
                }

                let received_data = &receive_buffer[0..received_size];
                match stream.write(received_data) {
                    Ok(send_size) => {
                        if send_size != received_size {
                            println!("** STREAM RESEND ERROR **");
                            return
                        }

                        println!("** STREAM PING PONG **");
                        print(received_data);
                    }
                    Err(_) => {
                        println!("** STREAM STOPPED (WRITE) **");
                        return
                    }
                }
            }
            Err(_) => {
                println!("** STREAM STOPPED (READ) **");
                return
            }
        }
    }
}

fn build_tray() -> Result<(), systray::Error> {
    let mut app;
    match systray::Application::new() {
        Ok(w) => app = w,
        Err(_) => panic!("Can't create window!"),
    }

    match app.set_icon_from_file("./resources/appicon_512x512.png") {
        Ok(..) => {},
        Err(err) => {
            println!("{}", err);
        }
    };
    app.set_tooltip(&"Whatever".to_string())?;

    app.add_menu_item("Print a thing", |_| {
        println!("Printing a thing!");
        Ok::<_, systray::Error>(())
    })?;

    app.add_menu_item("Add Menu Item", |window| {
        window.add_menu_item("Interior item", |_| {
            println!("what");
            Ok::<_, systray::Error>(())
        })?;
        window.add_menu_separator()?;
        Ok::<_, systray::Error>(())
    })?;

    app.add_menu_separator()?;

    app.add_menu_item("Quit", |window| {
        window.quit();
        Ok::<_, systray::Error>(())
    })?;

    println!("Waiting on message!");
    app.wait_for_message()?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    match build_tray() {
        Ok(..) => {

        },
        Err(..) => {

        }
    };

    println!("-- SERVER START --");
    let listener = TcpListener::bind("127.0.0.1:30000")?;
    for stream in listener.incoming() { handle_client(stream?); }
    println!("-- SERVER STOPPED --");

    Ok(())
}