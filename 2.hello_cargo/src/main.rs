fn main() {
    println!("Hello, world!");
}

// cree la carpeta del repo, pero cargo no deja crear un proyecto con un digito como primer char en
// el nombre. Cree la carpeta, me metí en la misma y corrí: $ cargo init --name hello_cargo
// cargo.toml es como el "package.json". "Crates" son los paquetes de código.
// Para contruir un proyecto de Cargo, hay que ejecutar: `$ cargo build`. Esto genera un ejecutable
// dentro de una carpeta `/target/debug/` porque por defecto es una compilación de depuración
// lo ejecutamos haciendo: `$ ./target/debug/hello_cargo`. O podemos usar: `$ cargo run`
// Por otro lado podemos usar: `$ cargo check` para no generar ningún ejecutable y verificar que
// el proyecto compila. Por último se puede usar: `$ cargo build --release`. Esto genera un
// ejecutable para producción, con sus optimizaciones, pero el tiempo de build es mayor. El
// ejecutable generado se guarda en `/target/release/`