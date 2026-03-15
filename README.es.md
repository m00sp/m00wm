# m00wm

[![en](https://img.shields.io/badge/English-380000?style=for-the-badge&logoColor=white&logo=DocuSign)](https://github.com/m00sp/m00wm/blob/main/README.md) [![es](https://img.shields.io/badge/Español-EB5406?style=for-the-badge&logoColor=white&logo=DocuSign)](https://github.com/m00sp/m00wm/blob/main/README.es.md) [![br](https://img.shields.io/badge/Português-380000?style=for-the-badge&logoColor=white&logo=DocuSign)](https://github.com/m00sp/m00wm/blob/main/README.pt-BR.md)

Un gestor de ventanas tiling de X11 escrito en Rust, construido desde cero utilizando la excelente crate [penrose](https://github.com/sminez/penrose).

Este es un proyecto educativo que construye un gestor de ventanas completamente funcional de forma incremental. Asegúrate de tener un entorno de escritorio alternativo disponible en caso de que algo falle durante las pruebas.

> **Agradecimientos especiales** a [sminez](https://github.com/sminez) por la excelente crate penrose y la documentación exhaustiva que hizo posible este proyecto.

## Descripción General

m00wm es un gestor de ventanas tiling minimalista que te proporciona simplicidad y control en tu escritorio X11. Construido con Rust, prioriza el rendimiento, la estabilidad y la capacidad de personalización.

**Sigue el desarrollo:**
- Mira el proceso de creación en [YouTube](https://www.youtube.com/playlist?list=PLy2HjaQiG8lOxCKzuWKfmmXov4iEVOGOF)
- Consulta [progress-so-far.md](./progress-so-far.md) para un registro de cambios de las características implementadas

## Características

- **Diseños tiling** – Gestión eficiente de espacios de trabajo con múltiples opciones de diseño
- **Atajos de teclado personalizables** – Atajos de teclado completamente configurables
- **Barra de estado** – Barra de estado integrada con información del sistema
- **Soporte multi-espacio de trabajo** – Organiza ventanas en espacios de trabajo virtuales
- **Gestión de ventanas flotantes** – Soporte para ventanas flotantes cuando sea necesario
- **Espaciado dinámico** – Huecos configurables entre ventanas

## Stack Tecnológica

- **Lenguaje:** [Rust](https://rust-lang.org)
- **Marco de Gestor de Ventanas:** [Penrose](https://github.com/sminez/penrose)
- **Protocolo X11:** x11rb
- **Registro de eventos:** tracing-subscriber

## Requisitos Previos

- [Rust](https://rustup.rs/) (última versión estable)
- Encabezados de desarrollo de X11
- Un gestor de pantalla compatible con sesiones de escritorio personalizadas
- Un entorno de escritorio alternativo para recuperación (recomendado durante el desarrollo)

## Instalación

> **NOTA:** Lee el [Makefile](./Makefile) antes de instalar para entender qué se ejecutará. No hay nada dañino, pero siempre revisa lo que ejecutas con `sudo`.

1. Clona el repositorio:
```bash
git clone https://github.com/m00sp/m00wm.git
cd m00wm
```

2. Revisa y personaliza los atajos de teclado en `src/main.rs` si es necesario (por defecto usa terminal `st` y `dmenu_run`):
```bash
# Reemplaza las referencias de terminal y lanzador por defecto
vim src/main.rs
```

3. Compila e instala:
```bash
make build && sudo make install
```

El gestor de ventanas estará disponible como sesión de escritorio en tu gestor de pantalla. Selecciona "m00wm" o "Plasma m00wm" al iniciar sesión.

## Compilación y Desarrollo

Compila en modo de lanzamiento:
```bash
make build
```

Prueba en una sesión X anidada:
```bash
make test
```

Desinstalar:
```bash
sudo make uninstall
```

## Estructura del Proyecto

```
src/
├── main.rs           # Punto de entrada, atajos de teclado y configuración del gestor
├── lib.rs            # Exportaciones de biblioteca y constantes
├── bar.rs            # Implementación de la barra de estado
├── layouts.rs        # Definiciones de diseños tiling
examples/             # Configuraciones de ejemplo
config/               # Archivos de sesión de escritorio
scripts/              # Utilidades de compilación y prueba
```

## Configuración

Las configuraciones clave se definen en `src/main.rs`:
- **Atajos de teclado** – Modifica el mapa de atajos para personalizar los atajos
- **Diseños** – Agrega o ajusta diseños tiling en `src/layouts.rs`
- **Colores y Fuentes** – Opciones de tema en constantes en la parte superior de `main.rs`
- **Barra de estado** – Personaliza la visualización de la barra de estado en `src/bar.rs`

## Obtener Ayuda

- Consulta [progress-so-far.md](./progress-so-far.md) para el estado de implementación
- Revisa la [documentación de penrose](https://github.com/sminez/penrose)
- Explora ejemplos en el directorio `examples/`

## Agradecimientos

- **sminez** por el marco [penrose](https://github.com/sminez/penrose) de gestor de ventanas y documentación excelente

Translated using GitHub Copilot and GPT-4o.
