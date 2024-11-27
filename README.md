# Rusty Journal

_A command line to-do app written in Rust_

## Descripción

Rusty Journal es una aplicación de línea de comandos para gestionar tareas, escrita en Rust. Permite añadir, listar y marcar tareas como completadas usando un archivo de diario en formato JSON.

## Instalación

Para instalar Rusty Journal, sigue estos pasos:

1. Clona el repositorio:

    ```sh
    git clone https://github.com/tuusuario/rusty-journal.git
    cd rusty-journal
    ```

2. Compila la aplicación:

    ```sh
    cargo build --release
    ```

3. Ejecuta la aplicación:

    ```sh
    ./target/release/rusty-journal
    ```

## Uso

Rusty Journal soporta las siguientes acciones:

### Añadir una tarea

Para añadir una nueva tarea, usa el comando `add` seguido de la descripción de la tarea:

```sh
rusty-journal add "Describir la nueva funcionalidad en el README"
```

### Listar tareas

Para listar todas las tareas, usa el comando list:

```sh
rusty-journal list
```

### Completar una tarea

Para marcar una tarea como completada, usa el comando done seguido del número de la tarea:

```sh
rusty-journal done 1
```

