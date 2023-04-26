---
title: "How Nautilus Works"
description: "Get started building with the Nautilus framework"
link: "https://github.com/nautilus-project/nautilus"
release: "0.0.1"
previous: "What is Nautilus?"
previousLink: "/docs/what-is-nautilus"
next: "Installation"
nextLink: "/docs/installation"
---


## Overview
---

* **cli**: The CLI for building & deploying Nautilus programs.
* **js**: The JavaScript client library.
* **py**: The Python client library.
* **solana**: The Rust crates.
    * **derive**: The macros that - leveraging the *nautilus-syn* crate - generate the code required.
    * **idl**: A crate of structs powered by *serde-json* that build an IDL JSON file *and it's relevant TypeScript and Python types*.
    * **src**: All traits and objects leveraged by the Nautilus framework. Also exposes *solana-program*, *spl-token*, and *mpl-token-metadata* dependencies.
    * **syn**: Code generation and parsing powered by Rust's *syn* and *quote* crates (*nautilus-syn*).
* **test-programs**: Full programs demonstrating Nautilus functionality.

## Process
---


## Other Resources
---

  * [GitHub Repo](https://github.com/nautilus-project/nautilus)