extern crate native_tls;


fn fetch_inbox_top() -> imap::error::Result<Option<String>> {
    let domain = "smtp.gmail.com";
    let tls = native_tls::TlsConnector::builder().build().unwrap();


    let client = imap::connect((domain, 993), domain, &tls).unwrap();

    let mut imap_session = client
        .login("email", "password")
        .map_err(|e| e.0)?;


    imap_session.select("INBOX")?;

    let messages = imap_session.fetch("1", "RFC822")?;
    let message = if let Some(m) = messages.iter().next() {
        m
    } else {
        return Ok(None);
    };

    let body = message.body().expect("message did not have a body!");
    let body = std::str::from_utf8(body)
        .expect("message was not valid utf-8")
        .to_string();

    println!("{}", body);
    
    imap_session.logout()?;

    Ok(Some(body))
}


fn main() {
    if let Err(e) = fetch_inbox_top() {
        println!("Erro: {:?}", e);
    }
}
