<div align="center">
  <a href="https://github.com/fission-codes/fission-cli" target="_blank">
    <img src="./assets/logo.png" alt="Fission-Cli Logo" width="100"></img>
  </a>

  <h1 align="center">Fission-Cli</h1>

  <p>
    <!-- <a href="https://github.com/fission-codes/rust-template/actions?query=">
      <img src="https://github.com/fission-codes/rust-template/actions/workflows/build.yml/badge.svg" alt="Build Status">
    </a> -->
    <a href="./LICENSE">
      <img src="https://img.shields.io/badge/License-Apache%202.0-blue.svg" alt="License-Apache">
    </a>
    <a href="https://fission.codes/discord">
      <img src="https://img.shields.io/static/v1?label=Discord&message=join%20us!&color=mediumslateblue" alt="Discord">
    </a>
    <a href="https://talk.fission.codes">
      <img src="https://img.shields.io/discourse/https/talk.fission.codes/topics" alt="Discourse">
    </a>
  </p>
</div>

##

### Note: **This is Archived**

This repository used to be a rewrite of an old Cli that can be found [here](https://github.com/fission-codes/fission/tree/main/fission-cli). The progress we are making toward finishing the rewrite can be seen on [this tracking issue](https://github.com/fission-codes/fission-cli/issues/1).
However, we're also rewriting the server implementation now, which lives at [`fission-codes/fission-server`](https://github.com/fission-codes/fission-server) and the monorepo contains a new CLI implementation in its [`fission-cli/` directory](https://github.com/fission-codes/fission-server/tree/main/fission-cli).

### What does it do?
This CLI allows a user to publish front-end webapps to IPFS and view it an easy way.

#### How Do I use it?
Take a look at our [getting started](https://guide.fission.codes/developers/getting-started) page for more information.


### Contributing
Check out our [contibuting guide](./CONTRIBUTING.md) for more info surrounding how to contribute to this project.

### Working from Source

#### Dependancies
The only dependancy is [Rust](https://www.rust-lang.org/).

#### Build from Source
`cargo run` or `cargo build` if you want to build without running it

#### Run Tests
`cargo test` or `cargo test -- --nocapture` if you want to see the console outputs of the tests
