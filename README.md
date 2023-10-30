# paket
Blazing fast, memory safe &amp; modern Linux package manager written in Rust.

## Roadmap

### Version: 0.1
- [x] Paket.toml file parsing. (#1, #2)
- [x] CLI handling (`paket <...>`) (#3)
- [ ] Creating .paket files with `paket build` (#6, #7, #8)
- [ ] Install a .paket file with `paket install <filename>` (#9)
- [ ] Upgrade, Downgrade an existing paket
- [ ] Remove a paket
### Version: 0.2
- [ ] `paket-server` serves as a repository file server
- [ ] QUIC + HTTP/3 implementation for both client(`paket-cli`) and server(`paket-server`)
- [ ] Download and install .paket files from repository
- [ ] Search pakets with `paket search [keywords]`
### Version 0.3:
- [ ] Development of paket-maker-gui, making the creating .paket process easy for package maintainers.

## How does it work?
Basically:
![Alt text](/paket-architecture.webp "Paket Architecture")

## Package Types
type | Description
---|---
[application](/libpaket/example_pakets/application_paket_with_assets/Paket.toml) | an Application in binary compiled form *(e.g. vlc)*
[script](/libpaket/example_pakets/script_paket_with_assets/Paket.toml) | an Application in script form *(e.g.: pardus-image-writer, Python or Javascript script applications)*
[library](#TODO) | a Shared Library to applications depend on it. *(e.g. libgtk-4-1)*
[development_library](#TODO) | Development files of a library which programs depend on. Used in application development. *(e.g. libgtk-4-dev)*
[application_source_code](#TODO) | Source code of an application. It can be used to compile the application from source code embedded in the package. *(e.g. gzip-src)*
[library_source_code](#TODO) | Source code of a library. It can be used to compile the library from source code embedded in the package. *(e.g. libXYZ-1-src)*
[configuration](/libpaket/example_pakets/configuration_paket/Paket.toml) | Files to be directly copied to the system. *(e.g. font-abc, icon-xyz, dotfiles-xyz)*
