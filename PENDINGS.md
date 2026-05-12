# ⏳ PENDINGS

Lista de funcionalidades y mejoras planificadas para RustBar.

## 🚀 Prioridad Alta
- [x] **Arquitectura Base**: Implementación de Servicios + Widgets (Modularización).
- [x] **Centro de Notificaciones**: DBus + UI integrada en Slint.
- [ ] **Widget de Batería**: Lectura de `/sys/class/power_supply/` y alertas de batería baja.
- [ ] **Control de Volumen**: Integración con PulseAudio/Pipewire (libpulse o similar).
- [ ] **Workspaces**: Indicador de espacios de trabajo para Niri/Hyprland mediante IPC.

## 🛠️ Mejoras de Sistema
- [x] **Compilación de Producción**: Binario optimizado en modo release.
- [x] **Integración con Compositor**: Autostart en MangoWC.
- [ ] **Archivo de Configuración**: Soportar `config.toml` o `config.json` para definir qué widgets cargar y en qué slots.
- [ ] **Estilo Dinámico**: Permitir cambiar colores o fuentes desde el archivo de configuración sin recompilar.
- [ ] **Persistencia de Notificaciones**: Guardar el historial en `~/.cache/rustbar/notifications.json`.

## 🎨 UI/UX
- [ ] **Animaciones de Entrada**: Desplazamiento suave para la bandeja de notificaciones.
- [ ] **Tooltips**: Mostrar información extra al pasar el ratón sobre los widgets.
- [ ] **Media Player Widget**: Control de música (MPRIS) con carátulas de álbum.

## 🧪 Investigación
- [ ] **Plugins Externos**: Investigar carga dinámica de librerías (.so) o WASM para widgets de terceros.
