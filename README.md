# paket
Blazing fast, memory safe &amp; modern Linux package manager written in Rust.

---

![Alt text](/paket-architecture.png "Paket Architecture")

**Example Paket.toml:**
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