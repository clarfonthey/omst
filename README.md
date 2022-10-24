# omst

Reveals whomst thou art with a single character.

## License

Available via the [Anti-Capitalist Software License][ACSL] for individuals, non-profit
organisations, and worker-owned businesses.

[ACSL]: ./LICENSE.md

## Installation

Just build `omst` and `omst-be` for your system and install them in `/usr/bin`.

## Usage

`omst` prints one of five characters based upon your effective user permissions:

1. `#` for absolute permissions (i.e. `root`, administrator)
2. `@` for system users
3. `$` for ordinary users
4. `%` for restricted users (e.g. `nobody`, guest)
5. `?` if any error occurs

In all cases, the character is followed by a newline. If an error occurs, the exit status will be
nonzero; to see full errors, run `omst-be` instead.

## System support

Currently, unix-family systems (via libc & shadow) and Windows (via WinAPI) are supported. Android
support is currently unavailable.

Mac OS and iOS are supported on a "coincidental" basis, meaning that if it happens to work under
the existing code, nice! Otherwise, no substantial code will be added for these targets, since Apple
does not make it easy to test software on their platforms without dedicated hardware.

## Implementation specifics

Under unix-family systems, the permissions are mapped based upon the effective user ID
(`libc::getuid`) and the `UID_MIN` and `UID_MAX` fields of `/etc/login.defs`:

1. `Absolute`: UID 0 (usually, but not always the `root` user)
2. `System`: Below `UID_MIN`
3. `User`: Between `UID_MIN` and `UID_MAX` (inclusive)
4. `Guest`: Above `UID_MAX`

Under Windows, the permissions are mapped based upon the `priv` field of the `USER_INFO`
struct:

1. `Absolute`: `USER_PRIV_ADMIN`
2. `System`: Unused (Windows doesn't have system users)
2. `User`: `USER_PRIV_USER`
3. `Guest`: `USER_PRIV_GUEST`
