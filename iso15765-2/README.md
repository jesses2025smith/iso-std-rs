[![Latest version](https://img.shields.io/crates/v/iso15765-2.svg)](https://crates.io/crates/iso15765-2)
[![Documentation](https://docs.rs/bleasy/badge.svg)](https://docs.rs/iso15765-2)
![LGPL](https://img.shields.io/badge/license-LGPL-green.svg)
![MIT](https://img.shields.io/badge/license-MIT-yellow.svg)
![Downloads](https://img.shields.io/crates/d/iso15765-2)

## Overview

**iso15765-2** is dedicated to implementing the generic ISO-TP protocol. ISO-TP (ISO 15765-2) is a transport protocol used in automotive communication.

## Features

- **ISO-TP Implementation**: Provides a complete implementation of the ISO-TP protocol in Rust.
- **Transport Layer Support**: Efficient handling of messages in the transport layer.
- **Asynchronous Support**: Designed to integrate seamlessly with asynchronous Rust applications.

### Prerequisites

- Rust 1.70 or higher
- Cargo (included with Rust)

## Goal List
- ISO-TP CAN
- ISO-TP LIN
- ISO-TP FlexRay
- ...

### Adding to Your Project

To use **iso15765-2** in your Rust project, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
iso15765-2 = { version="lastest-version" }
```

## Contributing

We're always looking for users who have thoughts on how to make `iso15765-2` better, or users with
interesting use cases.  Of course, we're also happy to accept code contributions for outstanding
feature requests!
