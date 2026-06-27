
## v0.3.0 - 2026-06-27







### :rocket: New features

- **(http)** Implement memory-safe synchronous HTTP client






### :bug: Bug fixes

- **(http)** Centralize HTTP client configuration and update error handling

- **(fixes)** Resolve HTTP payload corruption and Makefile uninstall bugs



















## v0.2.7 - 2026-05-02







### :rocket: New features

- **(process)** Implement asynchronous process execution module























## v0.2.6 - 2026-03-22







### :rocket: New features

- **(cmd)** Implement memory-safe shell command execution with RAII






















### :hammer: Build

- **(install)** Add ns_cmd to libs to install

- **(root)** Removed the hardcoded root commands



## v0.2.5 - 2026-03-21







### :rocket: New features

- **(ns_io)** Implement type-safe string interpolation for ns_print






### :bug: Bug fixes

- **(ns_read)** Bug fix



















## v0.2.4 - 2026-03-20







### :rocket: New features

- **(ns_string)** Add `ns_read` support for `ns_string`


















### :art: Styling

- **(typo)** Fixed a typo







## v0.2.3 - 2026-03-20







### :rocket: New features

- **(underline)** Add Underline macro to `ns_color`






### :bug: Bug fixes

- **(size_t)** Add proper support for size_t in ns_print



















## v0.2.2 - 2026-03-19







### :rocket: New features

- **(bool)** Add Boolean read and print






### :bug: Bug fixes

- **(error)** Better error handling



















## v0.2.1 - 2026-03-18







### :rocket: New features

- **(ns_data)** Add safe hashmap and update string docs






### :bug: Bug fixes

- **(string)** String printing bug



















## v0.2.0 - 2026-03-18







### :rocket: New features

- **(ns_data)** Introduce safe dynamic arrays (ns_vec)

- **(core)** Add ns_error compatibility with ns_string

- **(add colors)** Added terminal colors

- **(add ns_error_t)** Custom error type

- **(string)** Add memory-safe ns_string with Small String Optimization

- **(add cargo workspace)** Workspaces added for better structure

- **(separate print and println)** Separate print, println

- **(Add user input)** User input added

- **(Added string print)** Add string print

- **(updated examples)** 02_print_float_double updated

- **(add float and double)** Print float and double

- **(add CHANGELOG)** Added CHANGELOG

- **(add int print)** Added integer print

- **(Initial Commit)** Initial Commit






### :bug: Bug fixes

- **(repo name)** Put correct repo name in cliff.toml

- **(cliff.toml)** Bug in cliff.toml

- **(typo)** Typo in actions caused failure






### :zap: Performance

- **(io)** Optimize string printing with direct ns_string memory access

- **(Makefile update)** No need to recompile the whole lib






### :recycle: Refactoring

- **(vec.rs)** Seperate file for vec






### :art: Styling

- **(bump)** Bumped version from 0.1.6 to 0.1.7

- **(cliff.toml)** Update cliff.toml with CI section






### :hammer: Build

- **(install)** Make install now installs to system

- **(bump nextstd version)** Bumped from 0.1.7 to 0.1.8

- **(changed Makefile)** Update Makefile



### :tada: New Contributors
- @github-actions[bot] made their first contribution
- @Vaishnav-Sabari-Girish made their first contribution
