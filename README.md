# paket
Blazing fast, memory safe &amp; modern Linux package manager written in Rust.

## How does it work?
Basic principle of how paket works is:
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