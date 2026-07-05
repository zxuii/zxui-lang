# Zxui programming language

Zxui adalah bahasa pemrograman yang ditulis di rust dengan menggunakan arsitektur Tree-Walk interpreter yang meng-traverse setiap tree yang ada. walau lambat tapi karena cukup mudah di implemnetasikan kenapa tidak yakan? dan juga bahasa ini masih tahap pengembangan dan sangat jauh sekali dari kata selesai. kuharap aku terus melakukan update kepada bahasa pemrograman ini agar bisa menjadi bahasa pemrograman yang sangat bagus kedepannya. Aku berharap juga agar ini tidak lagi AST-based interpreter tetapi jadi Bytecode-based interpreter.

`closure.zxui`:
```js
fun caller(name) {
    fun things() {
        println(name, "fungsi things()");
        return 0;
    }
    return things;
}

fun tambah(a, b) {
    return a + b;
}

let x = tambah(1, 5);

println("tambah(1,5) = ", x + 5.2);

let c = caller("ini dipanggil dari: ");
println("nilai kembalian fungsi c(): ", c());
```

Kode di atas adalah demonstrasi sederhana bahasa pemrograman Zxui yang dimana dapat menggunakan closure.


## Special thanks to

- [ramtinJ95/rj-pylox](https://github.com/ramtinJ95/rj-pylox) - referensi awal untuk membuat bahasa ini di python
- [rspivak/lsbasi](https://github.com/rspivak/lsbasi) - gatau ini bahasa apa, gw cuma cari tahu cara membuat parser dari repo ini
- [craftinginterpreters.com](https://craftinginterpreters.com/the-lox-language.html) - sangat disarankan untuk membaca buku ini karena sangat bagus, bahasanya mudah dimengerti dengan banyak quote motivasi didalamnya.
- [jeschkies/lox-rs](https://github.com/jeschkies/lox-rs) - thanks kepada repo ini yang dimana telah membantuku bermigrasi dari python ke rust.

## Dokumentasi

nanti, video tentang pembuatan ini akan di upload setelah semua proses ini selesai di channel youtube [@TelurTerbulat](https://youtube.com/@TelurTerbulat)