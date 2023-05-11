# mtm
mail.tm library in rust


## in cargo.toml file
```
mtm = { git = "https://github.com/dutchaen/mtm" }
```

## example code with random account
```
use mtm::Account;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let account = Account::create_random()?;
    
    let messages = account.get_messages()?;
    let query = String::from("email@example.com");
    
    for msg in messages.iter() {
        if msg.from.address == query {
            println!("Intro to Message from {}: {}", query, msg.intro);
        }
    }
    
    return Ok(()); // <- account gets automatically deleted from mail.tm servers when dropped
}
```

## example code using channels
```
use mtm::Account;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let account = Account::create_random()?;
    let query = String::from("email@example.com");
    
    let rx = account.rx_messages();
    while let Ok((msg, err)) = rx.recv() { // <-  wait for message to be passed into the channel

        match msg {
            Some(msg) => {
                if msg.from.address == query {
                    println!("Intro to Message from {}: {}", query, msg.intro);
                    break;
                }
            },
            None => panic!("Error has occured receiving messages: {:?}", err)
        };
    }
    
    return Ok(()); // <- account gets automatically deleted from mail.tm servers when dropped
}
```
