# imapalyze

How the hell do i have so many unread emails? Who sent me these all these mails?

## Setup

Create a `.env` file or pass in these environment variables

| Variable name     | Description                         |
| ----------------- | ----------------------------------- |
| EMAIL_IMAP_DOMAIN | IMAP Server (E.g. `imap.gmail.com`) |
| EMAIL_IMAP_PORT   | IMAP Server port E.g.(`993`)        |
| EMAIL_LOGIN       | (Your) Username                     |
| EMAIL_PASSWORD    | (Your) Password                     |

### Run

Build it first 
```sh
cargo build --release
```
Then run it

```sh
target/release/imapalyze
```

### Additional Notes

- For GMail, you will have to [setup an app password](https://support.google.com/accounts/answer/185833?hl=en), as other methods are not in scope of this work.
- This IMAP connection is made with TLS so please ensure to pick the server port that allows for this.
