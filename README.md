# omst

Reveals whomst thou art with a single character.

## License

Available via the [Anti-Capitalist Software License][ACSL] for individuals, non-profit
organisations, and worker-owned businesses.

[ACSL]: ./LICENSE.md

## Installation

Just build `omst` for your system and install it in `/usr/bin`.

## Usage

`omst` prints one of five characters based upon your effective user permissions:

1. `#` for absolute permissions (i.e. `root`, administrator)
2. `@` for system users
3. `$` for ordinary users
4. `%` for restricted users (e.g. `nobody`, guest)
5. `?` if any error occurs

In all cases, the character is followed by a newline. If an error occurs, it may be printed to
`stderr`; be sure to suppress this error if you plan to use the character in a prompt.

## System support

Currently, unix-family systems (including Linux, Mac OS, and most BSDs) and Windows are supported.
Android and iOS support is currently not available.
