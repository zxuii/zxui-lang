# Zxui programming language

Zxui adalah bahasa pemrograman yang ditulis di rust dengan menggunakan arsitektur Tree-Walk interpreter yang meng-traverse setiap tree yang ada. walau lambat tapi karena cukup mudah di implemnetasikan kenapa tidak yakan? dan juga bahasa ini masih tahap pengembangan dan sangat jauh sekali dari kata selesai. Aku sudah cukup puas sih sebenarnya dengan bahasa pemrograman ini, aku tak menyangka aku bisa sejauh ini dalam mengembangkan ini semua sendirian dalam waktu kurang lebih 10 hari (berdasarkan commit pertama). Tidak menyangka aja, dari yang awalnya ditulis di python karena iseng, bisa menjadi bahasa pemrograman yang bisa menjalankan sebuah program bermakna.

`topdown.zxui`:
```swift
let width = 800
let height = 600
let speed = 400

initWindow(width, height, "Top Down Zxui")

let player = {
    x = width / 2,
    y = height / 2,
    w = 50,
    h = 50,
}

while !windowShouldClose() {
    let dt = getFrameTime()

    beginDrawing()

    clearBackground("white")

    if isKeyDown("w") {
        player.y -= speed * dt
    }
    if isKeyDown("a") {
        player.x -= speed * dt
    }
    if isKeyDown("s") {
        player.y += speed * dt
    }
    if isKeyDown("d") {
        player.x += speed * dt
    }

    drawRectangle(player.x, player.y, player.w, player.h, "red")

    endDrawing()
}

closeWindow()
```

Kode di atas adalah demonstrasi sederhana penggunaan raylib bawaan di bahasa pemrograman Zxui.

`todoapp.zxui`:
```swift
let todos = [];
let running = true;

fun tampilMenu() {
    println("===================="   );
    println("      TODO APP      "   );
    println("===================="   );
    println("1. Lihat todos"         );
    println("2. Tambah todo"         );
    println("3. Hapus todo"          );
    println("4. Tandai selesai/belum");
    println("5. Keluar"              );
    println("===================="   );
}

fun tampilTodos() {
    if len(todos) == 0 {
        println("Belum ada todo!");
    } else {
        for todo in todos {
            let tanda = "[ ]"
            if todo.selesai {
                tanda = "[x]"
            }
            println(todo.nama + " ", tanda)
        }
    }
}

fun tambahTodo() {
    let input = readline("Masukan todo: ");
    push(todos, { nama = input, selesai = false})
    println("Todo berhasil ditambahkan!");
}

fun cekNomor(idx) {
    return idx < 0 or idx >= len(todos)
}

fun hapusTodo() {
    if len(todos) == 0 {
        println("Belum ada todo yang bisa dihapus!");
    } else {
        tampilTodos();
        let input = readline("Hapus nomor berapa? ");
        let idx = number(input) - 1;
        if cekNomor(idx) {
            println("Nomor tidak valid!");
        } else {
            remove(todos, idx);
            println("Todo berhasil dihapus!");
        }
    }
}

fun toggleTodo() {
    if len(todos) == 0 {
        println("Belum ada todo!");
    } else {
        tampilTodos();
        let input = readline("Tandai nomor berapa? ");
        let idx = number(input) - 1;
        if cekNomor(idx) {
            println("Nomor tidak valid!");
        } else {
            let todo = todos[idx];
            if todo.selesai {
                todo.selesai = false;
            } else {
                todo.selesai = true;
            }
            println("Status todo berhasil diubah!");
        }
    }
}

while running {
    tampilMenu();
    let pilihan = readline("Pilih menu: ");

    if pilihan == "1" {
        tampilTodos();
    } else if pilihan == "2" {
        tambahTodo();
    } else if pilihan == "3" {
        hapusTodo();
    } else if pilihan == "4" {
        toggleTodo();
    } else if pilihan == "5" {
        println("Sampai jumpa!");
        running = false;
    } else {
        println("Pilihan tidak valid!");
    }
}
```

Kode di atas adalah demonstrasi aplikasi CRUD di bahasa pemrograman Zxui.

## Building

Kamu cukup bisa melakukan build dan run dengan menginstall cargo/rust compiler di website mereka lalu:

```bash
cargo b -r
```

atau langsung run:

```bash
cargo r -r <file.zxui>
```

kalau sudah build:

```bash
./target/release/zxui <file.zxui>
```

## TODOs

- [x] Membuat portingan Raylib <-> Zxui agar bisa membuat game topdown sederhana di zxui dengan ini. 

Todonya udah selesai... aku butuh tujuan baru, aku berniat untuk mengubah struktur tree-walk yang sekarang menjadi bytecode based interpreter. tapi itu butuh rewrite yang cukup super banyak terhadap interpreternya itu sendiri, aku harus mengubah satu atau dua file penuh dan menambah beberapa file lainnya dari 0 lagi. yaa untuk sekarang karena aku udah malas, mungkin bisa aku lanjutkan setelah jeda seminggu atau dua minggu. soalnya aku udah lock-in ke proyek ini selama seminggu lebih sih. feedback dari kalian mungkin akan membantuku! TYSM buat yang udah baca ya..!

## Special thanks to

Jujur aja untuk crafting interpreters ini bukunya bagus, tapi.... tapi nih ya... aku itu malas banget bacanya soalnya itu tuh puanjang banget dan ngebosenin banget jujur... jadi aku lebih milih liat kode kode dari repo orang orang ini atau sekedar mencari di google seperti bagaimana caranya A dan bagaimana caranya B kenapa bisa A kenapa tidak B dan hal-hal lain seperti itu. karena didalam bahasa pemrograman tak menyimpan satu konsep saja. tapi banyak konsep, dan seluruh konsep tersebut itu terlalu rapi, hingga aku bingung sekali harus mulai dari mana. maka dari itu aku melihat dari paling akar hingga paling pucuk.

- [ramtinJ95/rj-pylox](https://github.com/ramtinJ95/rj-pylox) - referensi awal untuk membuat bahasa ini di python
- [rspivak/lsbasi](https://github.com/rspivak/lsbasi) - gatau ini bahasa apa, gw cuma cari tahu cara membuat parser dari repo ini
- [craftinginterpreters.com](https://craftinginterpreters.com/the-lox-language.html) - sangat disarankan untuk membaca buku ini karena sangat bagus, bahasanya mudah dimengerti dengan banyak quote motivasi didalamnya.
- [jeschkies/lox-rs](https://github.com/jeschkies/lox-rs) - thanks kepada repo ini yang dimana telah membantuku bermigrasi dari python ke rust.

## History

Sebelumnya bahasa ini ditulis di satu file python yang berantakan, untuk sekarang memang udah lebih rapi dan menggunakan bahasa pemrograman rust yang lebih cepat secara performa karena memang compile-to-native ya kan ya, tapi tetep aja lambat kalau masih pake tree-based interpreter. aku inginnya sih langsung bisa stack-based vm interpreter. atau bahkan membuat compiler, tentunya aku malas berurusan dengan memory management jadi aku pake GC aja langsung kalau terpikirkan ide ini. ya semoga saja bahasa ini terus diupdate ygy. stay tune!

---

jujur aja, sepertinya tidak terlalu mudah untuk mengubah ini semua menjadi bytecode-based interpreter begitu saja... jadi aku mengubah tujuan agar bahasa pemrograman ini itu bisa membuat game di atas bahasa pemrograman ini sendiri saja.

## Dokumentasi

nanti, video tentang pembuatan ini akan di upload setelah semua proses ini selesai di channel youtube [@TelurTerbulat](https://youtube.com/@TelurTerbulat)