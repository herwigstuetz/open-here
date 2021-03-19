# open-here

`open-here` is a CLI tool to open URLs and files, like `xdg-open`, over the network.
This is useful, for example, if you are working via `ssh` remotely or in a local, headless development VM, e.g. WSL or Vagrant, and want to open a file or link with your local programs.

# Installing

To install `open-here` from source, you will need `cargo` and `rustc` (e.g. via [rustup](https://rustup.rs/)). It has been tested with version 1.49.0, but earlier versions might work as well.

Run:
```sh
$ git clone https://github.com/herwigstuetz/open-here
$ cd open-here
$ cargo install --path .
```
Make sure to add `~/.cargo/bin` to your `PATH`.

# Running

On your local machine, i.e. where you want to have the files opened, start the server with
```sh
$ open-here server 127.0.0.1:9123
# Open an ssh session and establish a tunnel from the host running the server to the client
$ ssh remote-host -R 9123:127.0.0.1:9123
```

On the `remote-host`, either in the `ssh` session established above, or another session, run `open-here open` with your link or file you want to open:
```sh
# Open https://crates.io/ in the default browser
$ open-here open https://crates.io/
# Open image.png in the default image viewer
$ open-here open ~/image.png
```
The environment variable `OPEN_HOST` can be used to configure the address and port of the `open-here server`. By default, if `OPEN_HOST` is not specified, `open-here open` tries to connect to `127.0.0.1:9123`.


# Integrations

To use `open-here` from within Emacs with `browse-url`, add this to your `init.el`:
```elisp
(when (executable-find "open-here")
  (setq browse-url-browser-function 'browse-url-generic
        browse-url-generic-program "open-here"
        browse-url-generic-args '("open")))
```

# WARNING

Running `open-here server` on a public interface opens that machine up to arbitrary opening/execution of files!
It is recommended to start `open-here server` on a loopback interface and forward a port from the remote machine.

# Known limitations

- Changes to files opened with `open-here` are not automatically synchronized. If an opened file changes on the client-side, it needs to be re-opened in order for the server to see those changes. Vice versa, changes to an opened file on the server side are not written back to the file on the client-side. There is currently no possibility to do this.
- There is no authentication for connection to an `open-here` server.
