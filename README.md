# 🦀 RustBar

**_A premium, modular Wayland bar built with Rust and Slint._**

RustBar es una barra de estado para Wayland diseñada con una estética "Glassmorphism" y una arquitectura moderna de **Servicios y Widgets** (inspirada en Noctalia). Está construida para ser rápida, ligera y extremadamente fácil de extender.

## ✨ Características

- 🏗️ **Arquitectura Modular**: Separación total entre la lógica de fondo (Servicios) y la interfaz de usuario (Widgets).
- 🔔 **Centro de Notificaciones**: Servidor DBus integrado que impersona a `mako`. Soporta historial, descarte individual y limpieza total.
- 🖥️ **Soporte Multi-monitor**: Sincronización automática de datos y estados en todas las pantallas detectadas.
- 🎨 **Estética Premium**: Diseño minimalista con gradientes sutiles, efectos de cristal y tipografía cuidada.
- ⌚ **Sistema de Slots**: Layout flexible con áreas dedicadas para widgets a la izquierda, centro y derecha.

## 🏛️ Arquitectura

El proyecto sigue un patrón de diseño desacoplado:
- **Servicios (`src/services/`)**: Gestionan los datos y protocolos de fondo (DBus, timers, etc.).
- **Widgets (`src/widgets/`)**: Actúan como puente, consumiendo servicios y actualizando la UI de Slint.
- **UI (`ui/widgets/`)**: Componentes visuales declarativos y reutilizables.

## 🚀 Cómo empezar

### Requisitos
- Rust (Cargo)
- Un compositor Wayland (Hyprland, Niri, Sway, MangoWC, etc.)
- `direnv` (opcional, para gestionar el entorno Nix)

### Ejecución
```bash
cargo run
```

### Probar Notificaciones
```bash
notify-send "RustBar" "¡Todo funciona perfectamente!"
```

---
Desarrollado con ❤️ por **mia** y **Antigravity**.
