# Zxui programming language

Ya... Experiment membuat sebuah bahasa pemrograman sendiri di python, kalau ditanya alasan kenapa pake python? karena python yang sejauh ini yang aku coba yang paling mudah untuk membuat bahasa pemrograman dibanding C dan Rust, awalnya saya sok-sokan mencoba membuat bahasa pemrograman di C, tapi karena C sendiri secara bawaan tak memiliki tipe data String yang jelas seperti python/rust itu sangat ribet sekali dan sering segfault atau bahkan stack frame sampe bermasalah/corrupt total dan gabisa debugging karena stack framenya kek tai ga tau dari mana asalnya sama sekali, pake strcpy strcat dan sebagainya itu jahanam, kalau di Rust, lebih agak ribetnya itu karena gw nyerah aja, males udah karena kek dipaksa banget dikit dikit pake match dikit dikit result aku malas pake itu jadi pada akhirnya milih python karena lebih sederhana dan lebih banyak sekali sumber yang bisa dijadikan sebuah patokan atau tutorial bagi saya. sumber tutorial tidak lain tidak bukan dari buku crafting interpreters dan contoh untuk pythonnya di:

https://github.com/ramtinJ95/rj-pylox (python lox)

https://github.com/rspivak/lsbasi (gatau ini bahasa apa, gw cuma cari tahu cara membuat parser dari repo ini)

sangat disarankan untuk membaca buku ini karena sangat bagus, bahasanya mudah dimengerti dengan banyak quote motivasi didalamnya:

https://craftinginterpreters.com/the-lox-language.html#top

nanti, video tentang pembuatan ini akan di upload setelah semua proses ini selesai di channel youtube @TelurTerbulat

Work cuy!

## TODOs

- [x] simple lexer
- [x] lexer proper untuk angka ( ) / * - * 
- [x] ast
- [x] simple parser untuk lexer sebelumnya
- [x] basic interpreter untuk semua ini
- [x] lexing & parsing variable decl
- [x] lexing parsing variable assignment, function decl, function call
- [x] interpret semua ini

## Penyelesaian proyek

Seluruh hal yang ada di TODOs sebelumnya sudah selesai sih, jadi aku bingung, apa yang harus kulakukan, tapi sebenarnya, buat apa aku bingung dengan apa yang harus aku buat? sedangkan hal yang harus aku lakukan banyak sekali, seperti merewrite ini semua ke rust lah, menambahkan berbagai macam fitur seperti if-else statement, atau hal hal lainnya, karena ini proof-of-concept kalau aku bisa menulis ini semua di python, maka masuk akal kalau aku bisa menulis ini semua di rust bukan? maka dari itu aku ingin mencoba menulis ulang hingga bisa menjalankan kode yang sama persis di dalam example.zxui yang ada di root proyek ini.

kalau penasaran, isi nya cuma:

```kotlin
fun caller(name) {
    fun things() {
        println(name);
    }
    return things;
}

fun tambah(a, b) {
    return a + b;
}

let x = tambah(1, 5);

print(x + 5.2);
println();

let c = caller(55555);
let call = c()
```

tapi ya setidaknya totally working:

```bash
> python main.py              
11.2
55555
```

## Update

Oke sekarang sudah bisa menjalankan simple.zxui yang berisi:

```rust
let x = 5;
let y = x + 5;
y = y + x;
y
```

menghasilkan:
```bash
./target/release/zxui-lang .\simple.zxui
program at 1:1
let at 1:1
identifier at 1:5
= at 1:7
number at 1:1
; at 1:10
let at 2:1
identifier at 2:5
= at 2:7
identifier at 2:9
+ at 2:11
number at 2:2
; at 2:14
identifier at 3:1
= at 3:3
identifier at 3:5
+ at 3:7
identifier at 3:9
; at 3:10
identifier at 4:1
eof at 4:2
Program(
    [
        Let {
            name: "x",
            expr: Number(
                5.0,
            ),
        },
        Let {
            name: "y",
            expr: BinOp {
                left: Identifier(
                    "x",
                ),
                op: Plus,
                right: Number(
                    5.0,
                ),
            },
        },
        Assign {
            name: "y",
            expr: BinOp {
                left: Identifier(
                    "y",
                ),
                op: Plus,
                right: Identifier(
                    "x",
                ),
            },
        },
        ExprStmt(
            Identifier(
                "y",
            ),
        ),
    ],
)
Number(15.0)
```

Aku berharap kalau proyek ini akan terus berlanjut hahahaha