## What is this for?

Pseudo-programmng language for library maintanence and initialization.

## TODO

* Add more features and keywords to lib_lang
* Add more detailed compiler errors

## Contributing

* New features are only to be added using new branch called `feature/THIS_IS_A_NEW_FEATURE`
* `feature` brances are only to be merged into `develop`
* `develop` is never to be edited directly, except in case of syntax errors or other similar cases.
* `develop` is only to be merged into `master` when thorough testing has been made.

## Installation

Run in project root

```sh
# run condition if on macOS
cargo run

if [ -d /usr/bin/ ]; then
    sudo mkdir /etc/bin/
fi
# run condition if on macOS

sudo chmod +x target/debug/lib_lang
sudo cp target/debug/lib_lang /usr/local/bin/
```

## How to run

### File
```sh
lib_lang run PATH
# or
lib_lang build PATH
```

### Source code
```sh
# run in project root
cargo run
```

## lib_lang example file

```lib_lang
lib init;

lib add user {
    name 'Adrian'
};

lib add book {
    index 1,
    title 'Hobbit',
    author 'Tolkien'
};

lib print user {
    index 1
};
```

## Libraries used

* **pest** parser and lexer
* **clap** CLI parser