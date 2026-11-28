# Resource pack dan gambar tombol

## Cara kerja

Resource pack adalah file `.zip` yang susunan foldernya sama dengan folder
`assets/` Veloren. Impor lewat launcher (menu **Resource Pack**). Berkas di pack
menggantikan berkas asli dengan nama yang sama; pack paling atas menang.

```text
tombol-saya.zip
└── voxygen/
    └── element/
        └── ui/
            └── touch/
                ├── jump.png
                ├── attack.png
                └── joystick.png
```

- Zip boleh membungkus semuanya dalam `assets/` atau satu folder bernama pack.
- Harus ada minimal satu folder `voxygen/`, `common/`, `world/`, atau `server/`.
- Perubahan berlaku saat game dibuka berikutnya.
- Pack bisa dipakai untuk tombol, tekstur UI, model `.vox`, suara, dan terjemahan.
  Pack yang berkasnya rusak bisa membuat game gagal dibuka: nonaktifkan di launcher.

## Gambar tombol

Format PNG persegi, latar transparan, disarankan 256x256. Nama berkas harus persis
seperti tabel. Tombol yang punya gambar tidak menampilkan teks lagi, jadi gambarnya
harus bisa dikenali. Tombol tanpa gambar tetap memakai teks.

### Prompt dasar

Salin prompt ini, lalu ganti `[SUBJEK]` dengan isi kolom "Subjek". Pakai prompt dasar
yang sama untuk semua tombol supaya gayanya seragam.

```text
Touch control button icon for a fantasy voxel adventure game: [SUBJEK].
Original design, chunky pixel art on a 32x32 grid, scaled up with crisp hard edges.
Square dark slate stone button with a beveled 3D edge, light from the top left.
One centered symbol in off-white with a 1px dark outline, small warm gold accents.
Front view, symmetrical framing, 1:1 square, transparent background outside the button.
No text, no letters, no logos, no characters from existing games.
```

Kalau generatornya tidak bisa membuat latar transparan, minta "plain solid magenta
background" lalu hapus latarnya dengan aplikasi penghapus background.

### Gerak

| Berkas | Subjek |
|---|---|
| `joystick.png` | a round translucent control pad ring with a small center knob, no square button |
| `dpad_up.png` | a bold arrow pointing up |
| `dpad_down.png` | a bold arrow pointing down |
| `dpad_left.png` | a bold arrow pointing left |
| `dpad_right.png` | a bold arrow pointing right |

### Aksi utama

| Berkas | Subjek |
|---|---|
| `attack.png` | a single sword slashing diagonally |
| `alt.png` | two crossed daggers with a spark |
| `jump.png` | a boot with an upward arrow |
| `roll.png` | a circular motion arrow around a small curled figure |
| `interact.png` | an open hand reaching forward |
| `sneak.png` | a crouching figure silhouette |
| `down.png` | a downward arrow over water waves |
| `giveup.png` | a small white flag |
| `respawn.png` | a glowing campfire with a circular arrow |

### Menu

| Berkas | Subjek |
|---|---|
| `inventory.png` | a leather backpack |
| `diary.png` | a closed journal with a bookmark |
| `menu.png` | three horizontal bars |
| `more.png` | three dots in a row |
| `less.png` | a bold X mark |
| `extra.png` | a plus sign inside a gear |
| `chat.png` | a speech bubble |
| `type.png` | a small keyboard |
| `map.png` | a folded treasure map |
| `crafting.png` | a hammer over an anvil |
| `social.png` | two small people side by side |
| `camera.png` | an eye inside a viewfinder |

### Perlengkapan dan gerakan

| Berkas | Subjek |
|---|---|
| `swap.png` | two weapons with circular swap arrows |
| `draw.png` | a sword being pulled from a scabbard |
| `sheathe.png` | a sword resting inside a scabbard |
| `block.png` | a wooden shield with a metal rim |
| `lantern.png` | a glowing hand lantern |
| `glide.png` | a simple cloth hang glider |
| `mount.png` | a saddle with an upward arrow |
| `unmount.png` | a saddle with a downward arrow |
| `trade.png` | two coins with exchange arrows |
| `stay.png` | a paw print with an open palm stop sign |
| `follow.png` | a paw print with a forward arrow |
| `sit.png` | a figure sitting on a log |
| `crawl.png` | a figure crawling |
| `dance.png` | a dancing figure with music notes |
| `greet.png` | a waving hand |
| `autowalk.png` | footprints with a repeating arrow |
| `walk.png` | a single slow footstep |
| `select.png` | a crosshair target |
| `zoomin.png` | a magnifying glass with a plus |
| `zoomout.png` | a magnifying glass with a minus |
| `zoomlock.png` | a magnifying glass with a padlock |
| `accept.png` | a green check mark |
| `decline.png` | a red cross mark |

### Slot

Tombol slot menimpa hotbar game di bagian bawah. Gambar yang penuh akan menutupi
ikon skill, jadi pakai bingkai kosong atau biarkan tanpa gambar.

| Berkas | Subjek |
|---|---|
| `slot1.png` … `slot10.png` | an empty square item frame with a fully transparent center, no symbol |
| `slotpage.png` | two small arrows pointing left and right |
