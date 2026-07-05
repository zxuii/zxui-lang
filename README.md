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

Nah sekarang aku udah nulis binding untuk rustnya, ya keliatannya sih seperti ini saat menjalankan example.zxui:

```bash
./zxui-lang .\example.zxui
program at 1:1
fun at 1:1
identifier(caller) at 1:5
( at 1:11
identifier(name) at 1:12
) at 1:16
{ at 1:18
fun at 2:5
identifier(things) at 2:9
( at 2:15
) at 2:16
{ at 2:18
identifier(println) at 3:9
( at 3:16
identifier(name) at 3:17
) at 3:21
; at 3:22
} at 4:5
return at 5:5
identifier(things) at 5:12
; at 5:18
} at 6:1
fun at 8:1
identifier(tambah) at 8:5
( at 8:11
identifier(a) at 8:12
, at 8:13
identifier(b) at 8:15
) at 8:16
{ at 8:18
return at 9:5
identifier(a) at 9:12
+ at 9:14
identifier(b) at 9:16
; at 9:17
} at 10:1
let at 12:1
identifier(x) at 12:5
= at 12:7
identifier(tambah) at 12:9
( at 12:15
number(1) at 12:12
, at 12:17
number(5) at 12:12
) at 12:20
; at 12:21
identifier(print) at 14:1
( at 14:6
identifier(x) at 14:7
+ at 14:9
number(5.2) at 14:14
) at 14:14
; at 14:15
identifier(println) at 15:1
( at 15:8
) at 15:9
; at 15:10
let at 17:1
identifier(c) at 17:5
= at 17:7
identifier(caller) at 17:9
( at 17:15
number(55555) at 17:17
) at 17:21
; at 17:22
let at 18:1
identifier(call) at 18:5
= at 18:10
identifier(c) at 18:12
( at 18:13
) at 18:14
eof at 19:1
Program(
    [
        FunDecl {
            name: "caller",
            params: [
                "name",
            ],
            body: [
                FunDecl {
                    name: "things",
                    params: [],
                    body: [
                        ExprStmt(
                            Call {
                                callee: "println",
                                args: [
                                    Identifier(
                                        "name",
                                    ),
                                ],
                            },
                        ),
                    ],
                },
                Return(
                    Identifier(
                        "things",
                    ),
                ),
            ],
        },
        FunDecl {
            name: "tambah",
            params: [
                "a",
                "b",
            ],
            body: [
                Return(
                    BinOp {
                        left: Identifier(
                            "a",
                        ),
                        op: Plus,
                        right: Identifier(
                            "b",
                        ),
                    },
                ),
            ],
        },
        Let {
            name: "x",
            expr: Call {
                callee: "tambah",
                args: [
                    Number(
                        1.0,
                    ),
                    Number(
                        5.0,
                    ),
                ],
            },
        },
        ExprStmt(
            Call {
                callee: "print",
                args: [
                    BinOp {
                        left: Identifier(
                            "x",
                        ),
                        op: Plus,
                        right: Number(
                            5.2,
                        ),
                    },
                ],
            },
        ),
        ExprStmt(
            Call {
                callee: "println",
                args: [],
            },
        ),
        Let {
            name: "c",
            expr: Call {
                callee: "caller",
                args: [
                    Number(
                        55555.0,
                    ),
                ],
            },
        },
        Let {
            name: "call",
            expr: Call {
                callee: "c",
                args: [],
            },
        },
    ],
)
```

Aku berharap kalau proyek ini akan terus berlanjut hahahaha