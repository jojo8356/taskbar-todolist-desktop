APP_NAME := taskbar-todolist-desktop
APP_TITLE := Taskbar Todolist
VERSION := $(shell sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -n 1)
PREFIX ?= $(HOME)/.local
BINDIR := $(PREFIX)/bin
DATADIR := $(PREFIX)/share
APPLICATIONS_DIR := $(DATADIR)/applications
LIBEXECDIR := $(DATADIR)/$(APP_NAME)
DESKTOP_FILE := $(APPLICATIONS_DIR)/$(APP_NAME).desktop
LAUNCHER := $(BINDIR)/$(APP_NAME)
INSTALLED_BINARY := $(LIBEXECDIR)/$(APP_NAME)
DISTDIR := dist
APPDIR_ROOT := $(DISTDIR)/appimage
APPIMAGE_DIR := $(APPDIR_ROOT)/$(APP_NAME).AppDir
DEB_ROOT := $(DISTDIR)/deb/$(APP_NAME)_$(VERSION)_amd64
DEB_FILE := $(DISTDIR)/$(APP_NAME)_$(VERSION)_amd64.deb
APPIMAGE_FILE := $(DISTDIR)/$(APP_NAME)-$(VERSION)-x86_64.AppImage
APPIMAGETOOL ?= appimagetool

.PHONY: build install uninstall desktop-refresh package package-deb package-appimage clean-dist

build:
	cargo build --release

install: build
	install -d "$(BINDIR)" "$(LIBEXECDIR)" "$(APPLICATIONS_DIR)"
	install -m 755 "target/release/$(APP_NAME)" "$(INSTALLED_BINARY)"
	printf '%s\n' '#!/usr/bin/env sh' \
		'set -eu' \
		'DATA_DIR="$${XDG_DATA_HOME:-$$HOME/.local/share}/$(APP_NAME)"' \
		'mkdir -p "$$DATA_DIR"' \
		'cd "$$DATA_DIR"' \
		'exec "$(INSTALLED_BINARY)" "$$@"' > "$(LAUNCHER)"
	chmod 755 "$(LAUNCHER)"
	printf '%s\n' \
		'[Desktop Entry]' \
		'Type=Application' \
		'Name=$(APP_TITLE)' \
		'Comment=Todo list compacte dans la zone de notification' \
		'Exec=$(LAUNCHER)' \
		'Icon=task-due' \
		'Terminal=false' \
		'Categories=Utility;' \
		'StartupNotify=false' > "$(DESKTOP_FILE)"
	chmod 644 "$(DESKTOP_FILE)"
	$(MAKE) desktop-refresh
	@printf '%s\n' 'Installé: $(DESKTOP_FILE)'

desktop-refresh:
	@if command -v update-desktop-database >/dev/null 2>&1; then \
		update-desktop-database "$(APPLICATIONS_DIR)" >/dev/null 2>&1 || true; \
	fi

uninstall:
	rm -f "$(DESKTOP_FILE)" "$(LAUNCHER)" "$(INSTALLED_BINARY)"
	-rmdir "$(LIBEXECDIR)" 2>/dev/null
	$(MAKE) desktop-refresh
	@printf '%s\n' 'Désinstallé: $(APP_NAME)'

package: package-deb package-appimage

clean-dist:
	rm -rf "$(DISTDIR)"

package-deb: build
	rm -rf "$(DEB_ROOT)"
	install -d "$(DEB_ROOT)/DEBIAN" "$(DEB_ROOT)/usr/bin" "$(DEB_ROOT)/usr/share/$(APP_NAME)" "$(DEB_ROOT)/usr/share/applications" "$(DEB_ROOT)/usr/share/icons/hicolor/scalable/apps"
	install -m 755 "target/release/$(APP_NAME)" "$(DEB_ROOT)/usr/share/$(APP_NAME)/$(APP_NAME)"
	install -m 644 "packaging/$(APP_NAME).svg" "$(DEB_ROOT)/usr/share/icons/hicolor/scalable/apps/$(APP_NAME).svg"
	sed 's|@EXEC@|/usr/bin/$(APP_NAME)|g; s|@ICON@|$(APP_NAME)|g' "packaging/$(APP_NAME).desktop.in" > "$(DEB_ROOT)/usr/share/applications/$(APP_NAME).desktop"
	printf '%s\n' '#!/usr/bin/env sh' \
		'set -eu' \
		'DATA_DIR="$${XDG_DATA_HOME:-$$HOME/.local/share}/$(APP_NAME)"' \
		'mkdir -p "$$DATA_DIR"' \
		'cd "$$DATA_DIR"' \
		'exec "/usr/share/$(APP_NAME)/$(APP_NAME)" "$$@"' > "$(DEB_ROOT)/usr/bin/$(APP_NAME)"
	chmod 755 "$(DEB_ROOT)/usr/bin/$(APP_NAME)"
	printf '%s\n' \
		'Package: $(APP_NAME)' \
		'Version: $(VERSION)' \
		'Section: utils' \
		'Priority: optional' \
		'Architecture: amd64' \
		'Maintainer: taskbar-todolist <noreply@example.com>' \
		'Depends: libc6, libgtk-3-0, libayatana-appindicator3-1 | libappindicator3-1, libsqlite3-0' \
		'Description: Compact tray todo list for Linux desktops' \
		' A small desktop todo list available from the system tray.' > "$(DEB_ROOT)/DEBIAN/control"
	dpkg-deb --root-owner-group --build "$(DEB_ROOT)" "$(DEB_FILE)"

package-appimage: build
	rm -rf "$(APPDIR_ROOT)"
	install -d "$(APPIMAGE_DIR)/usr/bin" "$(APPIMAGE_DIR)/usr/share/$(APP_NAME)" "$(APPIMAGE_DIR)/usr/share/applications" "$(APPIMAGE_DIR)/usr/share/icons/hicolor/scalable/apps"
	install -m 755 "target/release/$(APP_NAME)" "$(APPIMAGE_DIR)/usr/share/$(APP_NAME)/$(APP_NAME)"
	install -m 755 "packaging/AppRun" "$(APPIMAGE_DIR)/AppRun"
	install -m 644 "packaging/$(APP_NAME).svg" "$(APPIMAGE_DIR)/$(APP_NAME).svg"
	install -m 644 "packaging/$(APP_NAME).svg" "$(APPIMAGE_DIR)/usr/share/icons/hicolor/scalable/apps/$(APP_NAME).svg"
	sed 's|@EXEC@|AppRun|g; s|@ICON@|$(APP_NAME)|g' "packaging/$(APP_NAME).desktop.in" > "$(APPIMAGE_DIR)/$(APP_NAME).desktop"
	sed 's|@EXEC@|$(APP_NAME)|g; s|@ICON@|$(APP_NAME)|g' "packaging/$(APP_NAME).desktop.in" > "$(APPIMAGE_DIR)/usr/share/applications/$(APP_NAME).desktop"
	$(APPIMAGETOOL) "$(APPIMAGE_DIR)" "$(APPIMAGE_FILE)"
