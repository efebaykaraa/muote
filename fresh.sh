#!/bin/bash

# 1) Eski user servislerini durdur/disable et
systemctl --user disable --now desktop-quote.service marxist-quote-fetch.timer 2>/dev/null || true
systemctl --user disable --now engyls-quote.service 2>/dev/null || true

# 2) Paketleri kaldır
sudo pacman -Rns engyls marxist-quote 2>/dev/null || true

# 3) Eski config/cache kalıntılarını sil
rm -rf ~/.config/marxist_quote
rm -rf ~/.cache/marxist_quote

# 4) Eski user systemd linklerini temizle
rm -f ~/.config/systemd/user/desktop-quote.service
rm -f ~/.config/systemd/user/marxist-quote-fetch.service
rm -f ~/.config/systemd/user/marxist-quote-fetch.timer
rm -f ~/.config/systemd/user/engyls-quote.service
systemctl --user daemon-reload

# 5) Repo içindeki eski build/package çıktısını temizle
rm -rf pkg src target *.pkg.tar.zst
if [ -d ../engyls ]; then
  rm -rf ../engyls/pkg ../engyls/target ../engyls/*.pkg.tar.zst
fi

# 6) Yeni paketleri build et ve kur
if [ -d ../engyls ]; then
  (cd ../engyls && makepkg -si --noconfirm)
fi
makepkg -si
systemctl --user daemon-reload
systemctl --user enable --now desktop-quote.service marxist-quote-fetch.timer
