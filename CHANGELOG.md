# 📝 CHANGELOG

## [0.1.1] - 2026-05-12
### ✨ Añadido
- **Compilación de Producción**: Generación del binario optimizado en modo `--release`.
- **Integración con MangoWC**: Configuración del autostart del compositor para usar RustBar en lugar de Waybar.

## [0.1.0] - 2026-05-12

### ✨ Añadido
- **Arquitectura Modular**: Implementación del patrón **Servicios + Widgets** para desacoplar la lógica de fondo de la UI.
- **Centro de Notificaciones**: Servidor DBus (`org.freedesktop.Notifications`) funcional con historial y bandeja interactiva.
- **Widget de Reloj**: Sistema de tiempo sincronizado mediante servicios.
- **Sistema de Slots**: Layout de Slint rediseñado con contenedores `left`, `center` y `right`.
- **Superficie Extendida**: Configuración de superficie de 440px para permitir popups sin recortes de Wayland.
- **Documentación Técnica**: Creación de `GEMINI.md`, `README.md` y gestión de pendientes.

### 🔧 Cambiado
- Refactorización total de `main.rs` de un archivo monolítico a un orquestador de widgets.
- Migración de `PopupWindow` interno de Slint a una superficie de capa extendida.

### 🗑️ Eliminado
- Gestión manual de canales de UI en `main.rs` (ahora delegada a los widgets).
