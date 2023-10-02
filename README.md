# paket
Blazing fast, memory safe &amp; modern Linux package manager written in Rust.

---

![Alt text](/paket-architecture.png "Paket Architecture")

## Package Types
type | Description
---|---
application | an Application in binary compiled form *(e.g. vlc)*
script | an Application in script form *(e.g.: pardus-image-writer, Python or Javascript script applications)*
library | a Shared Library to applications depend on it. *(e.g. libgtk-4-1)*
development_library | Development files of a library which programs depend on. Used in application development. *(e.g. libgtk-4-dev)*
application_source_code | Source code of an application. It can be used to compile the application from source code embedded in the package. *(e.g. gzip-src)*
library_source_code | Source code of a library. It can be used to compile the library from source code embedded in the package. *(e.g. libXYZ-1-src)*
configuration | Files to be directly copied to the system. *(e.g. font-abc, icon-xyz, dotfiles-xyz)*

## Example Paket.toml:
```toml
[package]
name = "hello-world"
type = "application"
version = "0.1.0"
maintainers = ["Emin Fedar <eminfedar@gmail.com>"]

keywords = ["action", "shooter", "web", "2D"]
categories = ["Game", "Education"] # freedesktop categories: https://specifications.freedesktop.org/menu-spec/menu-spec-1.0.html#category-registry

homepage = "pardus.org.tr"
source_repository_url = "github.com/repo-here/if-exists"

license = "MIT" # or for custom licenses: "myapp.com/LICENSE"
architectures = ["amd64", "i386", "riscv64", "arm64"] # or any, all

description = """
Multiline or single line description of the package.
"""

[dependencies]
application = ["python3.11", "python3-gi"]
library = ["libgtk-3-0", "libglib2.0.0", "libpango-1.0-0"]
# development = ["libgtk-3-0-dev"] # This field is only needed if the paket type is `application_source_code` or `library_source_code`
```
