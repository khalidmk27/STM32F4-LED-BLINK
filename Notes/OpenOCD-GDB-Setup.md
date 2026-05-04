# STM32 Development Setup (WSL + OpenOCD + GDB)

This is a **clean, beginner-safe, repeatable checklist** for the setup:

* WSL → build + GDB
* Windows → OpenOCD
* STM32 → target
---

# 🧱 0. One-time prerequisites (already done)

Inside WSL:

```bash
sudo apt install gcc-arm-none-eabi gdb-multiarch cmake ninja-build
```

---

# ⚙️ 1. Build firmware using CMake

From your project root:

```bash
cd /mnt/d/STM32-Projects/LED-BLINK
```

---

## 🔧 Configure (ONLY needed once or after clean)

```bash
cmake -S . -B build \
  -DCMAKE_TOOLCHAIN_FILE=cmake/gcc-arm-none-eabi.cmake \
  -DCMAKE_BUILD_TYPE=Debug \
  -G Ninja
```

---

## 🔨 Build (do this every time you change code)

```bash
cmake --build build
```

---

## ✅ Verify output

```bash
ls build
```

You MUST see:

```text
LED-BLINK.elf
```

---

# 🔌 2. Start OpenOCD (Windows)

Open **Command Prompt / PowerShell (Windows)**

Run:

```bash
openocd -f interface/stlink.cfg -f target/stm32f4x.cfg -c "bindto 0.0.0.0"
```

---

## ✅ Confirm this line appears

```text
Listening on port 3333 for gdb connections
```

If not → debugging will NEVER work.

---

# 🌐 3. Get Windows IP (WSL → Windows bridge)

Inside WSL:

```bash
ip route | grep default
```

Example:

```text
default via 172.21.208.1 dev eth0
```

👉 Use that IP → `172.21.208.1`

---

## 🔎 Test connection (IMPORTANT)

```bash
nc -zv 172.21.208.1 3333
```

Must say:

```text
succeeded
```

---

# 🐞 4. Start GDB (WSL)

```bash
gdb-multiarch build/LED-BLINK.elf
```

You will see:

```text
(gdb)
```

---

# 🔗 5. Connect GDB → OpenOCD

Inside GDB:

```gdb
set architecture armv7e-m
target extended-remote 172.21.208.1:3333
```

---

## ✅ Success looks like:

```text
Remote debugging using 172.21.208.1:3333
```

---

# ⚡ 6. Flash firmware

Inside GDB:

```gdb
monitor reset halt
load
```

---

# ▶️ 7. Run program

```gdb
continue
```

---

# 🧪 8. Debug basics

### Break at main:

```gdb
break main
continue
```

---

### Step execution:

```gdb
next
step
```

---

### Inspect variables:

```gdb
print variable_name
```

---

### View call stack:

```gdb
bt
```

---

# 🛑 9. Quit debugging

```gdb
quit
```

---

# ⚠️ Common failure checklist (fast diagnosis)

### ❌ `Connection refused`

* OpenOCD not running
* Forgot `bindto 0.0.0.0`
* Firewall blocking

---

### ❌ `.elf not found`

* Forgot:

```bash
cmake --build build
```

---

### ❌ GDB crash

Fix:

```gdb
set architecture armv7e-m
```

---

### ❌ Breakpoints not hit

* Ensure Debug build:

```cmake
-O0 -g3
```

---

### ❌ HardFault / crash in HAL

* Likely bug in your code (like your NULL pointer case)

---

# 🧠 Final mental model

```text
CMake → builds .elf
        ↓
GDB (WSL)
        ↓ TCP (172.x.x.x:3333)
OpenOCD (Windows)
        ↓ SWD
STM32
```

---

# ⚡ Minimal daily workflow

You only repeat this:

```bash
# 1. Build
cmake --build build

# 2. Start OpenOCD (Windows)

# 3. Debug
gdb-multiarch build/LED-BLINK.elf
```

Inside GDB:

```gdb
set architecture armv7-m
target extended-remote <WINDOWS_IP>:3333
load
continue
```

---

