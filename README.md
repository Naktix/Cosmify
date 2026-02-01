# Installation

Cosmify is a mpris‑based media-control‑applet for the **COSMIC Desktop Environment**.
It is possible to install it manually or via flatpak (in the near future).

## Manual installation

```bash
sudo apt install cargo
```

### 1. Cloning the repository

```bash
git clone https://github.com/naktix/cosmify.git
cd cosmify
```

### 2. Build and install

```bash
./install_cosmify.sh
```

The script:
- compiles Cosmify in release-mode
- installs the binary at `/usr/bin/cosmify`
- installs the `.desktop`‑file at `/usr/share/applications/`
- installs the appstream‑metainfo at `/usr/share/metainfo/`

### 3. Restarting COSMIC

For COSMIC to detect the applet you need to:

- restart COSMIC **or**
- logout and login again

### 4. Activate the Applet

Open:

> **COSMIC Settings -> Desktop -> Panel / Dock -> Applets**

Scroll down to the external applets and choose **Cosmify**.

## How to uninstall Cosmify

If you ever wish to uninstall Cosmify just run:

```bash
./uninstall_cosmify.sh
```

The script removes:
- `/usr/bin/cosmify`
- `/usr/share/applications/dev.naktix.Cosmify.desktop`
- `/usr/share/metainfo/dev.naktix.Cosmify.metainfo.xml`

restart COSMIC afterwards.
