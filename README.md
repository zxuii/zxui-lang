# Zxui programming language

Zxui adalah bahasa pemrograman yang ditulis di rust dengan menggunakan arsitektur Tree-Walk interpreter yang meng-traverse setiap tree yang ada. walau lambat tapi karena cukup mudah di implemnetasikan kenapa tidak yakan? dan juga bahasa ini masih tahap pengembangan dan sangat jauh sekali dari kata selesai. kuharap aku terus melakukan update kepada bahasa pemrograman ini agar bisa menjadi bahasa pemrograman yang sangat bagus kedepannya. Aku berharap juga agar ini tidak lagi AST-based interpreter tetapi jadi Bytecode-based interpreter.

`closure.zxui`:
```kt
fun closure(name) {
    fun something() {
        println(name, "fungsi something()");
    }
    return something;
}

closure("ini dipanggil dari: ")();
```

Kode di atas adalah demonstrasi sederhana bahasa pemrograman Zxui yang dimana dapat menggunakan closure.


## Special thanks to

Jujur aja untuk crafting interpreters ini bukunya bagus, tapi.... tapi nih ya... aku itu malas banget bacanya soalnya itu tuh puanjang banget dan ngebosenin banget jujur... jadi aku lebih milih liat kode kode dari repo orang orang ini atau sekedar mencari di google seperti bagaimana caranya A dan bagaimana caranya B kenapa bisa A kenapa tidak B dan hal-hal lain seperti itu. karena didalam bahasa pemrograman tak menyimpan satu konsep saja. tapi banyak konsep, dan seluruh konsep tersebut itu terlalu rapi, hingga aku bingung sekali harus mulai dari mana. maka dari itu aku melihat dari paling akar hingga paling pucuk.

- [ramtinJ95/rj-pylox](https://github.com/ramtinJ95/rj-pylox) - referensi awal untuk membuat bahasa ini di python
- [rspivak/lsbasi](https://github.com/rspivak/lsbasi) - gatau ini bahasa apa, gw cuma cari tahu cara membuat parser dari repo ini
- [craftinginterpreters.com](https://craftinginterpreters.com/the-lox-language.html) - sangat disarankan untuk membaca buku ini karena sangat bagus, bahasanya mudah dimengerti dengan banyak quote motivasi didalamnya.
- [jeschkies/lox-rs](https://github.com/jeschkies/lox-rs) - thanks kepada repo ini yang dimana telah membantuku bermigrasi dari python ke rust.

## History

Sebelumnya bahasa ini ditulis di satu file python yang berantakan, untuk sekarang memang udah lebih rapi dan menggunakan bahasa pemrograman rust yang lebih cepat secara performa karena memang compile-to-native ya kan ya, tapi tetep aja lambat kalau masih pake tree-based interpreter. aku inginnya sih langsung bisa stack-based vm interpreter. atau bahkan membuat compiler, tentunya aku malas berurusan dengan memory management jadi aku pake GC aja langsung kalau terpikirkan ide ini. ya semoga saja bahasa ini terus diupdate ygy. stay tune!

## Dokumentasi

nanti, video tentang pembuatan ini akan di upload setelah semua proses ini selesai di channel youtube [@TelurTerbulat](https://youtube.com/@TelurTerbulat)