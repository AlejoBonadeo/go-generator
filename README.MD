# Go Module Generator

CLI tool to simplify the generation of Go project files.

---

## **Requirements**

- **Rust and Cargo**: You need Rust and Cargo installed on your system. Follow the [Getting Started guide](https://doc.rust-lang.org/book/ch01-01-installation.html) from the official Rust Book.

---

## **Installation**

1. **Clone the Repository**:

   ```bash
   git clone https://github.com/yourusername/go-mod-generator.git
   cd go-mod-generator
   ```

2. **Install the CLI**:
   Run the following command to build and install the CLI locally:
   ```bash
   cargo install --path .
   ```

---

## **Usage**

Navigate to the root directory of your Go project, then run:

- On **Linux/Mac**:
  ```bash
  go-generator
  ```
- On **Windows**:
  ```bash
  go-generator.exe
  ```

### **What It Does**

This CLI tool will:

- **Generate new files**:
  - `usecases/name.go`
  - `usecases/name_tests.go`
  - `rest/name.go`
- **Create or update existing files**:
  - `entrypoints/module.go`
  - `usecases/errors.go`
  - `rest/codes.go`
- **Update existing files** (assumes they exist):
  - `wire.go`
  - `containers.go`
  - `functions.go`

### **Input Requirements**

- **Module** and **Endpoint** names must be provided in **lowerCamelCase**.

---

## **License**

This project is licensed under the [MIT License](LICENSE).

---

### **TODOs**

- Add parameters for methods(currently set to post by default).
- Support repository and provider generation.
- Support conditional generation
- Run wire gen

---

This `README.md` provides clear installation and usage instructions, making it easier for users to set up and use your CLI tool effectively.
