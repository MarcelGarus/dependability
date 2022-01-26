use retry::retry;

// Our connection isn't great, so connect() will always fail
fn connect(_host: &'static str, _port: u16) -> Option<&[u8]> {
    None
}

// For efficiency or security, used requests are marked as "sent"
struct Request {
    sent: bool,
    data: [u8; 1024],
}

#[retry(2)]
fn make_request(r: &mut Request) -> Result<(), &'static str> {
    // Obviously, you can't reuse a request
    if r.sent {
        Err("Can only send a request once")
    } else {
        // We sent the request, so it can't be used again
        // ...which makes the #[retry] completely useless.
        r.sent = true;
        if let Some(data) = connect("realserver.com", 1234) {
            r.data.copy_from_slice(data);
        } else {
            return Err("Failed to connect");
        }
        Ok(())
    }
}

fn main() {
    let mut req = Request {
        sent: false,
        data: [0; 1024],
    };
    let _ = dbg!(make_request(&mut req));
}
