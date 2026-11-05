//! Text written for Rusty MCLA's own controls and messages.
//!
//! These translations are not Rockstar strings. Original game text must continue to use
//! `StringTable` and the player's original language resources instead of this catalog.
//! The labels below describe the port's added settings, not the persisted enum words.

use mcla_config::Language;

macro_rules! catalog {
    ($( $key:ident, $origin:literal, $en:literal, $es:literal $(, keys [$($alias:literal),+])?; )+) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum Text {
            $(#[doc = $origin] $key,)+
        }

        pub fn text(language: Language, key: Text) -> &'static str {
            match (language, key) {
                $((Language::English, Text::$key) => $en,
                  (Language::Spanish, Text::$key) => $es,)+
            }
        }

        /// A visible keycap label only. Input/save names and unknown tokens remain untouched.
        pub fn key_label(language: Language, name: &str) -> &str {
            if language == Language::English {
                return name;
            }
            match name {
                $($($($alias)|+ => text(language, Text::$key),)?)+
                _ => name,
            }
        }

        /// Audit metadata: key, origin, English text and the port's Spanish translation.
        pub const ENTRIES: &[(Text, &str, &str, &str)] = &[$((Text::$key, $origin, $en, $es)),+];
    };
}

catalog! {
    Display, "Port's added OPTIONS page (D19).", "DISPLAY", "PANTALLA";
    Performance, "Port's added OPTIONS page (D19).", "PERFORMANCE", "RENDIMIENTO";
    RearViewMirror, "Port's visible toggle for the original hidden debug mirror (game.rs Options, D20); not an original menu string.", "REAR-VIEW MIRROR", "RETROVISOR";
    WindowMode, "PC window setting on the port's DISPLAY page.", "WINDOW MODE", "MODO DE VENTANA";
    Resolution, "PC display setting on the port's DISPLAY page.", "RESOLUTION", "RESOLUCIÓN";
    Monitor, "PC display setting on the port's DISPLAY page.", "MONITOR", "MONITOR";
    Vsync, "PC display setting on the port's DISPLAY page.", "VSYNC", "SINCRONIZACIÓN VERTICAL";
    FpsCap, "Port's frame limiter setting.", "FPS CAP", "LÍMITE DE FPS";
    FpsCounter, "Port's FPS overlay setting (D25).", "FPS COUNTER", "CONTADOR DE FPS";
    RenderScale, "Port's render-scale setting.", "RENDER SCALE", "ESCALA DE RENDERIZADO";
    TextureFiltering, "Port's added override of original texture filtering.", "TEXTURE FILTERING", "FILTRADO DE TEXTURAS";
    AntiAliasing, "Port's added override of original anti-aliasing.", "ANTI-ALIASING", "SUAVIZADO DE BORDES";
    Bloom, "Port's added switch for the original rendering effect.", "BLOOM", "RESPLANDOR";
    DepthOfField, "Port's added switch for the original rendering effect.", "DEPTH OF FIELD", "PROFUNDIDAD DE CAMPO";
    MotionBlur, "Port's added switch for the original rendering effect.", "MOTION BLUR", "DESENFOQUE DE MOVIMIENTO";
    ShadowResolution, "Port's shadow-map size multiplier setting.", "SHADOW RESOLUTION", "RESOLUCIÓN DE SOMBRAS";
    DrawDistance, "Port's draw-distance multiplier setting.", "DRAW DISTANCE", "DISTANCIA DE DIBUJADO";
    DetailLevel, "Port's detail multiplier setting (3X includes high-detail traffic).", "DETAIL LEVEL", "NIVEL DE DETALLE";
    Intro, "Port's start-up/intro mode setting (D24).", "INTRO", "INTRODUCCIÓN";
    StickDeadZone, "Port's additional gamepad dead-zone setting (D39).", "STICK DEAD ZONE", "ZONA MUERTA DEL STICK";
    PortOn, "Enabled value for added PC settings; not the original G_On string.", "ON", "ACTIVADO";
    PortOff, "Disabled value for added PC settings; not the original G_Off string.", "OFF", "DESACTIVADO";
    Original, "Port setting value meaning original rendering behavior.", "ORIGINAL", "ORIGINAL";
    Windowed, "Display-only name of the windowed config enum value.", "WINDOWED", "VENTANA";
    Borderless, "Display-only name of the borderless config enum value.", "BORDERLESS", "SIN BORDES";
    Fullscreen, "Display-only name of the fullscreen config enum value.", "FULLSCREEN", "PANTALLA COMPLETA";
    FpsFrameTime, "Port FPS overlay mode; FPS remains a technical unit.", "FPS + FRAME TIME", "FPS + TIEMPO POR FOTOGRAMA";
    IntroNormal, "Port intro setting's normal value.", "NORMAL", "NORMAL";
    IntroFast, "Port intro setting's fast value.", "FAST", "RÁPIDA";
    IntroNextStart, "Port intro setting's deferred-change notice.", "(NEXT START)", "(PRÓXIMO INICIO)";
    ScreenshotFailed, "Port's F12 screenshot notification.", "SCREENSHOT FAILED", "ERROR AL GUARDAR LA CAPTURA";
    ScreenshotSaved, "Port's F12 screenshot notification.", "SCREENSHOT SAVED", "CAPTURA GUARDADA";
    KeyboardControls, "Port's keyboard binding editor (mc4.rs keyboard editor).", "KEYBOARD CONTROLS", "CONTROLES DEL TECLADO";
    Key, "Port's keyboard binding editor column label.", "KEY", "TECLA";
    SecondKey, "Port's keyboard binding editor secondary slot suffix.", " (2ND KEY)", " (TECLA 2)";
    PressKeyCancel, "Port's keyboard binding capture prompt; ESC remains the key identifier.", "PRESS A KEY - ESC: CANCEL", "PULSA UNA TECLA - ESC: CANCELAR";
    KeyboardSteerLeft, "Port's keyboard-only digital split of the original steering axis; not an original action string.", "STEER LEFT", "GIRAR A LA IZQUIERDA";
    KeyboardSteerRight, "Port's keyboard-only digital split of the original steering axis; not an original action string.", "STEER RIGHT", "GIRAR A LA DERECHA";
    KeyboardNextAbility, "Port's keyboard shortcut for cycling abilities; not an original action string.", "NEXT ABILITY", "SIGUIENTE HABILIDAD";
    KeyboardLeanBack, "Port's keyboard-only digital split of the original lean axis; not an original action string.", "LEAN BACK", "INCLINARSE HACIA ATRÁS";
    KeyboardLeanForward, "Port's keyboard-only digital split of the original lean axis; not an original action string.", "LEAN FORWARD", "INCLINARSE HACIA ADELANTE";
    KeyboardResetVehicle, "Port's PC vehicle reset shortcut; not an original action string.", "RESET VEHICLE", "RESTABLECER VEHÍCULO";
    KeyUp, "Port keyboard keycap label; stored key name remains UP.", "UP", "ARRIBA", keys ["UP"];
    KeyDown, "Port keyboard keycap label; stored key name remains DOWN.", "DOWN", "ABAJO", keys ["DOWN"];
    KeyLeft, "Port keyboard keycap label; stored key name remains LEFT.", "LEFT", "IZQUIERDA", keys ["LEFT"];
    KeyRight, "Port keyboard keycap label; stored key name remains RIGHT.", "RIGHT", "DERECHA", keys ["RIGHT"];
    KeySpace, "Port keyboard keycap label; stored key name remains SPACE.", "SPACE", "ESPACIO", keys ["SPACE"];
    KeyTab, "Port keyboard keycap label; TAB is retained as a technical keycap.", "TAB", "TAB", keys ["TAB"];
    KeyCapsLock, "Port keyboard keycap label; stored key name remains CAPS LOCK.", "CAPS LOCK", "BLOQ MAYÚS", keys ["CAPS LOCK"];
    KeyLeftShift, "Port keyboard keycap label; stored key name remains LEFT SHIFT.", "LEFT SHIFT", "MAYÚS IZQ", keys ["LEFT SHIFT"];
    KeyRightShift, "Port keyboard keycap label; stored key name remains RIGHT SHIFT.", "RIGHT SHIFT", "MAYÚS DER", keys ["RIGHT SHIFT"];
    KeyLeftCtrl, "Port keyboard keycap label; retain the CTRL key token.", "LEFT CTRL", "CTRL IZQ", keys ["LEFT CTRL"];
    KeyRightCtrl, "Port keyboard keycap label; retain the CTRL key token.", "RIGHT CTRL", "CTRL DER", keys ["RIGHT CTRL"];
    KeyLeftAlt, "Port keyboard keycap label; retain the ALT key token.", "LEFT ALT", "ALT IZQ", keys ["LEFT ALT"];
    KeyRightAlt, "Port keyboard keycap label; retain the ALT key token.", "RIGHT ALT", "ALT DER", keys ["RIGHT ALT"];
    KeyInsert, "Port keyboard keycap label; INSERT is retained as a technical keycap.", "INSERT", "INSERT", keys ["INSERT"];
    KeyHome, "Port keyboard keycap label; stored key name remains HOME.", "HOME", "INICIO", keys ["HOME"];
    KeyEnd, "Port keyboard keycap label; stored key name remains END.", "END", "FIN", keys ["END"];
    KeyPageUp, "Port keyboard keycap label; stored key name remains PAGE UP.", "PAGE UP", "RE PÁG", keys ["PAGE UP", "PGUP"];
    KeyPageDown, "Port keyboard keycap label; stored key name remains PAGE DOWN.", "PAGE DOWN", "AV PÁG", keys ["PAGE DOWN", "PGDN"];
    KeyMinus, "Port keyboard keycap label; stored key name remains MINUS.", "MINUS", "MENOS", keys ["MINUS"];
    KeyEquals, "Port keyboard keycap label; stored key name remains EQUALS.", "EQUALS", "IGUAL", keys ["EQUALS"];
    KeyLeftBracket, "Port keyboard keycap label; stored key name remains LEFT BRACKET.", "LEFT BRACKET", "CORCHETE IZQ", keys ["LEFT BRACKET"];
    KeyRightBracket, "Port keyboard keycap label; stored key name remains RIGHT BRACKET.", "RIGHT BRACKET", "CORCHETE DER", keys ["RIGHT BRACKET"];
    KeyBackslash, "Port keyboard keycap label; stored key name remains BACKSLASH.", "BACKSLASH", "BARRA INVERSA", keys ["BACKSLASH"];
    KeySemicolon, "Port keyboard keycap label; stored key name remains SEMICOLON.", "SEMICOLON", "PUNTO Y COMA", keys ["SEMICOLON"];
    KeyQuote, "Port keyboard keycap label; stored key name remains QUOTE.", "QUOTE", "COMILLA", keys ["QUOTE"];
    KeyComma, "Port keyboard keycap label; stored key name remains COMMA.", "COMMA", "COMA", keys ["COMMA"];
    KeyPeriod, "Port keyboard keycap label; stored key name remains PERIOD.", "PERIOD", "PUNTO", keys ["PERIOD"];
    KeySlash, "Port keyboard keycap label; stored key name remains SLASH.", "SLASH", "BARRA", keys ["SLASH"];
    KeyBackquote, "Port keyboard keycap label; stored key name remains BACKQUOTE.", "BACKQUOTE", "ACENTO GRAVE", keys ["BACKQUOTE"];
    KeyIntlBackslash, "Port keyboard keycap label; stored key name remains INTL BACKSLASH.", "INTL BACKSLASH", "BARRA INVERSA INTL", keys ["INTL BACKSLASH"];
    KeyNumPlus, "Port keyboard keycap label; retain the NUM key prefix.", "NUM PLUS", "NUM MÁS", keys ["NUM PLUS"];
    KeyNumMinus, "Port keyboard keycap label; retain the NUM key prefix.", "NUM MINUS", "NUM MENOS", keys ["NUM MINUS"];
    KeyNumMultiply, "Port keyboard keycap label; retain the NUM key prefix.", "NUM MULTIPLY", "NUM MULTIPLICAR", keys ["NUM MULTIPLY"];
    KeyNumDivide, "Port keyboard keycap label; retain the NUM key prefix.", "NUM DIVIDE", "NUM DIVIDIR", keys ["NUM DIVIDE"];
    KeyNumDecimal, "Port keyboard keycap label; NUM DECIMAL is also Spanish technical text.", "NUM DECIMAL", "NUM DECIMAL", keys ["NUM DECIMAL"];
    KeyEscape, "Port keyboard keycap label; ESC is the technical abbreviation for stored ESCAPE.", "ESCAPE", "ESC", keys ["ESCAPE"];
    KeyEnter, "Port keyboard keycap label; retain the ENTER key token.", "ENTER", "ENTER", keys ["ENTER"];
    KeyNumEnter, "Port keyboard keycap label; retain the ENTER and NUM key tokens.", "NUM ENTER", "ENTER NUM", keys ["NUM ENTER"];
    KeyBackspace, "Port keyboard keycap label; stored key name remains BACKSPACE; BKSP is a prompt alias.", "BACKSPACE", "RETROCESO", keys ["BACKSPACE", "BKSP"];
    KeyDelete, "Port keyboard keycap label; stored key name remains DELETE.", "DELETE", "SUPR", keys ["DELETE"];
    KeyMouse, "Port keyboard/mouse prompt label; not an original controller token.", "MOUSE", "RATÓN", keys ["MOUSE"];
    KeyArrows, "Port keyboard prompt label for the four direction keys.", "ARROWS", "FLECHAS", keys ["ARROWS"];
    KeyLeftRight, "Port keyboard prompt label; source LEFT/RIGHT token is unchanged.", "LEFT/RIGHT", "IZQ./DER.", keys ["LEFT/RIGHT"];
    KeyPageUpDown, "Port keyboard map zoom prompt; source PGUP/PGDN token is unchanged.", "PGUP/PGDN", "REPÁG/AVPÁG", keys ["PGUP/PGDN"];
    Gamepad, "Port's input-device chooser; original controller style names use their own official keys.", "GAMEPAD", "MANDO";
    Keyboard, "Port's input-device chooser.", "KEYBOARD", "TECLADO";
    RacingWheel, "Port's input-device chooser; not the original controller-style string.", "RACING WHEEL", "VOLANTE";
    ModMissing, "Port's mod garage status; not an original vehicle unlock message.", "MOD MISSING", "MOD NO INSTALADO";
    Skip, "Port's fallback video overlay; original boot Flash has no added skip line.", "SKIP", "OMITIR";
    CollectablesNeeded, "Port's added collectable lock fallback; an original resource key, when present, takes precedence.", "{count} COLLECTABLES NEEDED", "SE NECESITAN {count} COLECCIONABLES";
    PressAnyKey, "Port's added keyboard start prompt; not the original controller prompt.", "PRESS ANY KEY", "PULSA CUALQUIER TECLA";
    QuitGame, "Port's PC quit command; not an original console option.", "QUIT GAME", "SALIR DEL JUEGO";
    LeftRightHint, "Port's keyboard hint; preserve the selected action field.", "LEFT / RIGHT  {action}", "IZQ. / DER.  {action}";
    EscapeBackHint, "Port's keyboard hint; ESC remains the key identifier.", "ESC  BACK", "ESC  VOLVER";
    VinylKeyboardHint, "Port's keyboard controls for the vinyl editor.", "LEFT / RIGHT  CHANGE    SHIFT  FINE    ESC  DONE", "IZQ. / DER.  CAMBIAR    MAYÚS  AJUSTE FINO    ESC  TERMINAR";
    PerformanceKeyboardHint, "Port's keyboard controls for performance purchases.", "ENTER  BUY THE NEXT STAGE      ESC  BACK", "ENTER  COMPRAR LA SIGUIENTE FASE      ESC  VOLVER";
    RaceEditorKeyboardHint, "Port's PC race editor keyboard controls.", "ARROWS  MOVE      ENTER  PLACE (START FIRST)      BACKSPACE  UNDO      R  RACE IT      E  LEAVE", "FLECHAS  MOVER      ENTER  COLOCAR (PRIMERO LA SALIDA)      RETROCESO  DESHACER      R  COMPETIR      E  SALIR";
    MapKeyboardHint, "Port's PC map keyboard controls and legend.", "ARROWS  MOVE      Z / X  ZOOM      ORANGE  RACE EVENTS      BLUE  CAREER EVENTS      E  RACE EDITOR      M / ESC  CLOSE", "FLECHAS  MOVER      Z / X  ZOOM      NARANJA  CARRERAS      AZUL  EVENTOS DEL MODO HISTORIA      E  EDITOR DE CARRERAS      M / ESC  CERRAR";
    PhotoKeyboardHint, "Port's photo-mode keyboard/mouse controls.", "WASD  MOVE      Q / E  DOWN / UP      RIGHT MOUSE  LOOK      ENTER  TAKE PHOTO      ESC  BACK", "WASD  MOVER      Q / E  BAJAR / SUBIR      BOTÓN DERECHO  MIRAR      ENTER  HACER FOTO      ESC  VOLVER";
    GarageKeyboardHint, "Port's keyboard hint for entering the garage.", "G  ENTER GARAGE", "G  ENTRAR EN EL GARAJE";
    BuildStreetRaceFailed, "Port's procedural race builder error.", "Could not build a race here, drive to a street first", "No se ha podido crear una carrera aquí. Conduce hasta una calle.";
    VehicleReady, "Port's player vehicle replacement status; preserve the vehicle name.", "{vehicle} ready", "{vehicle} listo";
    VehicleLoadFailed, "Port's player vehicle replacement error.", "Failed to load that car", "No se ha podido cargar ese vehículo";
    HydraulicsKeyboardOn, "Port's hydraulics keyboard status/hint; preserve keycaps.", "HYDRAULICS ON   {keys}", "HIDRÁULICA ACTIVADA   {keys}";
    HydraulicsKeyboardOff, "Port's hydraulics keyboard status.", "HYDRAULICS OFF", "HIDRÁULICA DESACTIVADA";
    CycleAbility, "Port's keyboard ability-cycle status; preserve the original ability name.", "ABILITY: {ability}", "HABILIDAD: {ability}";
    GpuDriverTitle, "Port's graphics initialization error dialog.", "Rusty MCLA - the graphics driver could not start the game", "Rusty MCLA - el controlador gráfico no pudo iniciar el juego";
    GpuDriverMessage, "Port's graphics initialization error dialog.", "The graphics driver could not start the game.", "El controlador gráfico no pudo iniciar el juego.";
    GpuDriverHelp, "Port's graphics initialization error dialog.", "Updating the graphics driver may help. If it keeps happening, please report it on GitHub with the logs.", "Actualizar el controlador gráfico puede ayudar. Si el problema persiste, comunícalo en GitHub y adjunta los registros.";
    GpuUnsupportedTitle, "Port's unsupported graphics error dialog.", "Rusty MCLA - graphics card not supported", "Rusty MCLA - tarjeta gráfica no compatible";
    GpuUnsupportedMessage, "Port's unsupported graphics error dialog.", "Rusty MCLA cannot start on this graphics card.", "Rusty MCLA no puede iniciarse con esta tarjeta gráfica.";
    GpuRequirements, "Port's unsupported graphics error dialog.", "The game needs a graphics card whose driver supports Vulkan or DirectX 12. This one has neither, so the game cannot create its graphics device on it.", "El juego necesita una tarjeta gráfica cuyo controlador sea compatible con Vulkan o DirectX 12. Esta no es compatible con ninguno de los dos, por lo que el juego no puede crear el dispositivo gráfico.";
    GpuAlternative, "Port's unsupported graphics error dialog.", "If the PC has another graphics card, make sure the game runs on that one and its driver is up to date.", "Si el PC tiene otra tarjeta gráfica, comprueba que el juego la utilice y que su controlador esté actualizado.";
    GpuGraphicsLabel, "Port error dialog field label; GPU names/backends are not translated.", "Graphics", "Gráficos";
    GpuNone, "Port error dialog when no suitable adapter was found.", "none found that can draw to the window", "no se ha encontrado ninguna que pueda mostrar el juego en la ventana";
    GpuOpenGlOnly, "Port error dialog; OpenGL is a technical API name.", "OpenGL only", "solo OpenGL";
    GpuDetailsLabel, "Port error dialog; error details themselves are not translated.", "Details", "Detalles";
    GpuLogLocation, "Port error dialog; preserve {path} and the launcher's existing Export Logs button name.", "The game's log is in {path}. The launcher's Export Logs collects it.", "El registro del juego está en {path}. La opción Export Logs del lanzador lo recopila.";
}



/// The startup language. This does not read or change the language in a game save.
pub fn current(key: Text) -> &'static str {
    text(crate::hooks::language(), key)
}
