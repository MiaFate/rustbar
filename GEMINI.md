# 🤖 GEMINI.md - Contexto Técnico para la IA

Este archivo contiene el contexto arquitectónico y las directrices técnicas para mantener y expandir RustBar de forma coherente.

## 🏗️ Patrón Arquitectónico: Servicios + Widgets

RustBar utiliza una arquitectura desacoplada para garantizar escalabilidad y estabilidad.

### 1. Servicios (`src/services/`)
Los servicios son la **Fuente de Verdad**. No tienen interfaz y corren de fondo.
- **Trait `Service`**: Define los métodos `name()` y `update()`.
- **Comunicación**: Los servicios deben usar `Arc<Mutex<T>>` para exponer sus datos de forma segura entre hilos.
- **Responsabilidad**: protocolos (DBus), timers maestros, polling de sistema.

### 2. Widgets (`src/widgets/`)
Los widgets son el **Puente**. Conectan servicios con Slint.
- **Trait `Widget`**: Define `name()`, `init()` (para callbacks de Slint) y `update()` (para volcar datos a Slint).
- **Comunicación**: Reciben servicios en su constructor (`new(service)`).
- **Responsabilidad**: Formateo de datos para la UI, captura de eventos de clic, actualización de propiedades Slint.

### 3. UI Modular (`ui/rustbar.slint` & `ui/widgets/`)
- **MainWindow**: Utiliza un sistema de **Slots** (`left-slot`, `center-slot`, `right-slot`).
- **Superficie Extendida**: La ventana principal tiene una altura de **440px** con una `exclusive_zone` de **40px**. Esto permite que los `PopupWindow` (bandeja de notificaciones) se dibujen por debajo de la barra sin ser recortados por el compositor.
- **Componentes**: Todos los componentes visuales deben vivir en `ui/widgets/` e importarse en el archivo principal.

## 🛠️ Guía para añadir nuevas funcionalidades

Para añadir un nuevo componente (ej. "Widget de Batería"):
1. **Servicio**: Crea `src/services/battery.rs`. Implementa la lógica de lectura de `/sys/class/power_supply/`.
2. **Widget**: Crea `src/widgets/battery.rs`. Suscríbete al `BatteryService` y actualiza Slint.
3. **UI**: Crea `ui/widgets/battery.slint` con el diseño visual.
4. **Registro**:
   - Añade el servicio y widget en `src/main.rs`.
   - Importa y coloca el componente en el slot deseado de `ui/rustbar.slint`.

## ⚠️ Notas de Estabilidad (Wayland/Slint)
- **Layer Shell**: Se usa `layer-shika`. Cuidado con las dimensiones de las superficies.
- **DBus**: El servidor de notificaciones impersona a `mako`. Asegurarse de que no haya otros demonios corriendo antes de iniciar.
- **Multi-monitor**: Siempre iterar sobre `st.bar_instances` para actualizar todas las pantallas.

## 📜 Política de Documentación (Obligatoria)
Para garantizar la persistencia del conocimiento, **siempre** se deben actualizar los siguientes archivos tras cambios significativos:
- **README.md**: Mantener la descripción general, características y guía de inicio al día.
- **CHANGELOG.md**: Registrar todas las nuevas funciones, cambios en la arquitectura o correcciones críticas.
- **PENDINGS.md**: Actualizar el roadmap, tachar tareas completadas y añadir nuevos desafíos detectados.
- **GEMINI.md**: Reflejar cualquier cambio en la arquitectura base, nuevos patrones de diseño o notas de estabilidad.

---
*Documento generado por Antigravity para mantener la excelencia técnica en RustBar.*
