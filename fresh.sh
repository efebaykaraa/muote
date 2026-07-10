#!/bin/bash

# 1) Eski user servislerini durdur/disable et
systemctl --user disable --now desktop-quote.service marxist-quote-fetch.timer 2>/dev/null || true

# 2) Paketleri kaldır
sudo pacman -Rns wikiquote-fetcher marxist-quote 2>/dev/null || true

# 3) Eski config/cache kalıntılarını sil
rm -rf ~/.config/marxist_quote
rm -rf ~/.cache/marxist_quote

# 4) Eski user systemd linklerini temizle
rm -f ~/.config/systemd/user/desktop-quote.service
rm -f ~/.config/systemd/user/marxist-quote-fetch.service
rm -f ~/.config/systemd/user/marxist-quote-fetch.timer
systemctl --user daemon-reload

# 5) Repo içindeki eski build/package çıktısını temizle
rm -rf pkg src target *.pkg.tar.zst
if [ -d ../wikiquote-fetcher ]; then
  rm -rf ../wikiquote-fetcher/pkg ../wikiquote-fetcher/target ../wikiquote-fetcher/*.pkg.tar.zst
fi

# 6) Yeni paketleri build et ve kur
if [ -d ../wikiquote-fetcher ]; then
  (cd ../wikiquote-fetcher && makepkg -si --noconfirm)
fi
makepkg -si
