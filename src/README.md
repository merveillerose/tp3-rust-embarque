# TP3&4 – Rust Embarqué : Embassy et les périphériques

Firmware embarqué pour la carte d'extension ENSEA sur Nucleo-64 L476RG, développé en Rust avec la bibliothèque [Embassy](https://embassy.dev/).

---

## Matériel cible

| Carte | MCU |
|-------|-----|
| Nucleo-64 L476RG + shield ENSEA | STM32L476RG (Cortex-M4, 80 MHz) |

---

## Structure du projet

```
src/
├── bsp_ensea.rs       # Board Support Package – association pins / périphériques
├── bargraph.rs        # Driver bargraph 8 LEDs
├── gamepad.rs         # Driver croix de boutons
├── encoder.rs         # Driver encodeur rotatif (QEI sur TIM2)
├── stepper.rs         # Driver moteur pas à pas (PWM sur TIM3)
├── oled.rs            # Driver écran OLED SSD1306 (I2C)
├── lib.rs
├── main.rs            # Application principale (tâches Embassy)
└── bin/
    ├── bargraph_example.rs
    ├── gamepad_example.rs
    ├── encoder_example.rs
    └── stepper_example.rs
```

---

## Périphériques et broches

| Périphérique | Broches |
|---|---|
| Bargraph 8 LEDs | PB5, PB14, PB4, PB15, PB1, PA8, PB2, PC7 |
| Gamepad (5 boutons) | PC8, PB11, PC9, PC6, PC5 |
| Encodeur rotatif | PA0, PA1 (QEI), PA15 (bouton) |
| Moteur pas à pas | PA7 (DIR), PA6 (STEP/PWM), PA12 (ENN), PA11, PB12 (MS1/2) |
| Écran OLED SSD1306 | PB6 (SCL), PB7 (SDA) – I2C1 |
| GPS | PB13 (enable) |
| USART1 | PA9 (TX), PA10 (RX) |
| USART2 | PA2 (TX), PA3 (RX) |
| SPI2 | PB10 (SCK), PC3 (MOSI), PC2 (MISO), PC0 (CS) |

---

## Architecture logicielle

### Partie 1 – Board Support Package (`bsp_ensea.rs`)

Le BSP centralise toutes les associations entre périphériques et broches. La structure `Board` regroupe les sous-structures `BargraphPins`, `GamepadPins`, `EncoderPins`, `SteppersPins`, `I2C1Pins`, etc. Cela évite de relire la documentation matérielle à chaque utilisation.

### Partie 2 – Drivers

Chaque driver est indépendant et ne dépend que de son groupe de broches fourni par le BSP.

- **Bargraph** : `set_range(min, max)` + `set_value(v)` allume un nombre de LEDs proportionnel à la valeur.
- **Gamepad** : `poll() -> GamepadState` lit les 5 boutons ; `is_pressed(Button)` pour un test unitaire.
- **Encodeur** : basé sur `Qei` (embassy_stm32). `position()` retourne une valeur centrée sur 0. `set_position()` et `reset()` agissent directement sur le registre `CNT` via le PAC.
- **Stepper** : génère les impulsions STEP via un timer PWM (TIM3). `set_speed(hz, direction)`, `enable()`, `disable()`, `set_microstepping(mode)`.
- **OLED** : driver SSD1306 via I2C bloquant. Affiche la position de l'encodeur, la vitesse et direction du moteur, et l'état du gamepad.

### Partie 3 – Tâches asynchrones (`main.rs`)

L'application est découpée en tâches Embassy indépendantes communiquant via des variables atomiques et des signaux :

| Tâche | Rôle |
|---|---|
| `encoder_task` | Lit la position (toutes les 100 ms), met à jour `BARGRAPH_VALUE` et `STEPPER_SPEED/DIR` |
| `bargraph_task` | Attend le signal `BARGRAPH_SIGNAL`, met à jour les LEDs |
| `gamepad_task` | Lit les boutons (toutes les 50 ms), stocke l'état dans `GAMEPAD_BITS` |
| `stepper_update_task` | Attend `STEPPER_SIGNAL`, applique vitesse et direction au moteur |
| `emergency_stop_task` | Surveille le bouton de l'encodeur par interruption EXTI ; arrêt immédiat du moteur |
| `oled_task` | Rafraîchit l'écran toutes les 200 ms avec l'état de tous les périphériques |

### Partie 4 – Synchronisation et race conditions

L'accès concurrent au compteur de l'encodeur (registre `TIM2.CNT`) entre `encoder_task` (lecture) et `emergency_stop_task` (remise à zéro) est protégé par un mutex :

```rust
static ENCODER_MUTEX: Mutex<CriticalSectionRawMutex, ()> = Mutex::new(());
```

Les deux tâches acquièrent ce verrou avant d'accéder au registre, garantissant qu'une remise à zéro d'urgence ne peut pas survenir en pleine lecture.

---

## Compilation et flash

```bash
# Compiler le firmware principal
cargo build --release

# Flasher via probe-rs
cargo run --release

# Lancer un exemple spécifique
cargo run --bin bargraph --release
cargo run --bin gamepad  --release
cargo run --bin encoder  --release
cargo run --bin stepper  --release
```

Les logs `defmt` sont disponibles via RTT (probe-rs affiche automatiquement la sortie série).

---

## Dépendances principales

| Crate | Rôle |
|---|---|
| `embassy-stm32` | HAL async pour STM32 |
| `embassy-executor` | Exécuteur de tâches async |
| `embassy-sync` | Primitives de synchronisation (Signal, Mutex) |
| `embassy-time` | Timers async |
| `ssd1306` | Driver écran OLED |
| `embedded-graphics` | Rendu texte/graphiques |
| `defmt` + `defmt-rtt` | Logs embarqués via RTT |
