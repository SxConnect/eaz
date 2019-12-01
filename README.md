[![Crate](https://img.shields.io/crates/v/check-if-email-exists.svg)](https://crates.io/crates/check-if-email-exists)
[![Docs](https://docs.rs/check-if-email-exists/badge.svg)](https://docs.rs/check-if-email-exists/)
[![](https://img.shields.io/travis/amaurymartiny/check-if-email-exists.svg)](https://travis-ci.org/amaurymartiny/check-if-email-exists)
[![](https://ci.appveyor.com/api/projects/status/github/amaurymartiny/check-if-email-exists?branch=master&svg=true)](https://ci.appveyor.com/project/amaurymartiny/check-if-email-exists-a08kp)
![License](https://img.shields.io/github/license/amaurymartiny/check-if-email-exists.svg)
[![](https://img.shields.io/badge/Buy%20me%20a%20tree-%F0%9F%8C%B3-lightgreen)](https://offset.earth/amaurymartiny)

<br /><br /><br />

<h1 align="center">check-if-email-exists</h1>
<h4 align="center">Check if an email address exists before sending the email.</h4>

<br /><br /><br />

#### 👉 Try it here: https://3nbyey1wsi.execute-api.us-east-1.amazonaws.com/dev/?to_email=YOUR_EMAIL_HERE

And replace the `YOUR_EMAIL_HERE` placeholder with the email you would like to verify.

> Note: The above operation might take up to 15s.

## What Does This Tool Check?

✔️ **Syntax validation.** Is the address syntactically valid?

✔️ **DNS records validation.** Does the domain of the email address have valid MX DNS records?

✔️ **Disposable email address (DEA) validation.** Is the address provided by a known [disposable email address](https://en.wikipedia.org/wiki/Disposable_email_address) provider?

✔️ **SMTP server validation.** Can the mail exchanger of the email address domain be contacted successfully?

✔️ **Mailbox deliverability.** Is mailbox for the email address deliverable?

✔️ **Mailbox disabled.** Has this email address been disabled by the email provider?

✔️ **Full inbox.** Is the inbox of this mailbox full?

✔️ **Catch-all address.** Is this email address a [catch-all](https://debounce.io/blog/help/what-is-a-catch-all-or-accept-all/) address?

Planned features:

-   [ ] **Role account validation.** Is the email address a well-known role account?
-   [ ] **Free email provider check.** Is the email address bound to a known free email provider?
-   [ ] **Syntax validation, provider-specific.** According to the syntactic rules of the target mail provider, is the address syntactically valid.
-   [ ] **Honeypot detection.** Does email address under test hide a [honeypot](<https://en.wikipedia.org/wiki/Honeypot_(computing)>)?

## Why?

Many online services (https://hunter.io, http://verify-email.org, http://email-checker.net) offer this service for a paid fee. Here is an open-source alternative to those tools.

## Download the binary

## Try it now

There are 4 ways to try `check-if-email-exists`.

### 1. Use the hosted version

Try it here: https://3nbyey1wsi.execute-api.us-east-1.amazonaws.com/dev/?to_email=YOUR_EMAIL_HERE

And replace the `YOUR_EMAIL_HERE` placeholder with the email you would like to verify.

> Note: The above operation might take up to 15s.

### 2. Use the Docker Image

### 3. Download the binary

> Note: The binary doesn't connect to the above `amazonaws.com` backend, it checks the mail directly from your computer.

Head to the [releases page](https://github.com/amaurymartiny/check-if-email-exists/releases) and download the binary for your platform.

Make sure you have [`openssl`](https://www.openssl.org/) installed.

```
Check if an email address exists without sending any email.

USAGE:
    check_if_email_exists [FLAGS] [OPTIONS] [TO_EMAIL]

FLAGS:
        --http       Runs a HTTP server
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --from <FROM_EMAIL>    The from email to use in the SMTP connection [default: user@example.org]
        --http-port <PORT>     Sets the port on which the HTTP server should bind. Only used when `--http` flag is on
                               [default: 3000]

ARGS:
    <TO_EMAIL>    The email to check
```

**PRO TIP:** To show debug logs when running the binary, run:

```bash
RUST_LOG=debug check_if_email_exists [FLAGS] [OPTIONS] <TO_EMAIL>
```

### 4. Usage as a Library (Advanced)

In your own Rust project, you can add `check-if-email-exists` in your `Cargo.toml`:

```toml
[dependencies]
check-if-email-exists = "0.5"
```

And use it in your code as follows (async/await syntax):

```rust

use check_if_email_exists::email_exists;

// First arg is the email we want to check, second arg is the FROM email used in the SMTP connection
let checked = email_exists("check.this.email@gmail.com", "user@example.org").await;

println!({:?}, checked); // `checked` is a SingleEmail struct
```

## JSON Output

The output will be a JSON with the below format, the fields should be self-explanatory. For `someone@gmail.com` (note that it is disabled by Gmail), here's the exact output:

```json
{
	"input": "someone@gmail.com",
	"mx": {
		"is_disposable": false,
		"records": [
			"alt3.gmail-smtp-in.l.google.com.",
			"gmail-smtp-in.l.google.com.",
			"alt1.gmail-smtp-in.l.google.com.",
			"alt4.gmail-smtp-in.l.google.com.",
			"alt2.gmail-smtp-in.l.google.com."
		]
	},
	"smtp": {
		"has_full_inbox": false,
		"is_catch_all": false,
		"is_deliverable": false,
		"is_disabled": true
	},
	"syntax": {
		"address": "someone@gmail.com",
		"domain": "gmail.com",
		"username": "someone",
		"valid_format": true
	}
}
```

## FAQ

### The library hangs/takes a long time/doesn't show anything after 1 minute.

Most ISPs block outgoing SMTP requests through ports 25, 587 and 465, to prevent spam. `check-if-email-exists` needs to have these ports open to make a connection to the email's SMTP server, so won't work behind these ISPs, and will instead hang until it times out. There's unfortunately no easy workaround for this problem, see for example [this StackOverflow thread](https://stackoverflow.com/questions/18139102/how-to-get-around-an-isp-block-on-port-25-for-smtp). One solution is to rent a Linux cloud server with a static IP and no blocked ports.

To see in details what the binary is doing behind the scenes, run it in [verbose mode](#verbose-mode) to see the logs.

## Legacy Bash Script

The 1st version of this tool was a simple bash script which made a telnet call. If you would like to use that simpler version, have a look at the [`legacy`](https://github.com/amaurymartiny/check-if-email-exists/tree/legacy) branch. The reasons for porting the bash script to the current codebase are explained [in this issue](https://github.com/amaurymartiny/check-if-email-exists/issues/4).

## Build From Source

First, [install Rust](https://www.rust-lang.org/tools/install); you'll need Rust 1.37.0 or later. Then, clone the source code locally:

```bash
# Download the code
$ git clone https://github.com/amaurymartiny/check-if-email-exists
$ cd check-if-email-exists

# Build in release mode
$ cargo build --release

# Run the binary
$ ./target/release/check-if-email-exists --help
```

## License

GPL-3.0. See the [LICENSE](./LICENSE) file for more info.
