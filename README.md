# Zxui programming language

Ya... Experiment membuat sebuah bahasa pemrograman sendiri di python, kalau ditanya alasan kenapa pake python? karena python yang sejauh ini yang aku coba yang paling mudah untuk membuat bahasa pemrograman dibanding C dan Rust, awalnya saya sok-sokan mencoba membuat bahasa pemrograman di C, tapi karena C sendiri secara bawaan tak memiliki tipe data String yang jelas seperti python/rust itu sangat ribet sekali dan sering segfault atau bahkan stack frame sampe bermasalah/corrupt total dan gabisa debugging karena stack framenya kek tai ga tau dari mana asalnya sama sekali, pake strcpy strcat dan sebagainya itu jahanam, kalau di Rust, lebih agak ribetnya itu karena gw nyerah aja, males udah karena kek dipaksa banget dikit dikit pake match dikit dikit result aku malas pake itu jadi pada akhirnya milih python karena lebih sederhana dan lebih banyak sekali sumber yang bisa dijadikan sebuah patokan atau tutorial bagi saya. sumber tutorial tidak lain tidak bukan dari buku crafting interpreters dan contoh untuk pythonnya di:

https://github.com/ramtinJ95/rj-pylox (python lox)

https://github.com/rspivak/lsbasi (gatau ini bahasa apa, gw cuma cari tahu cara membuat parser dari repo ini)

sangat disarankan untuk membaca buku ini karena sangat bagus, bahasanya mudah dimengerti dengan banyak quote motivasi didalamnya:

https://craftinginterpreters.com/the-lox-language.html#top

nanti, video tentang pembuatan ini akan di upload setelah semua proses ini selesai di channel youtube @TelurTerbulat

saat ini jika dijalankan `python main.py`:

```rust
[TokenType.PROGRAM(PROGRAM),
 TokenType.INT(1),
 TokenType.PLUS(+),
 TokenType.INT(1),
 TokenType.EOF(EOF)]
Program(block=BinOp(left=Int(ty=TokenType.INT(1)),
                    op=TokenType.PLUS(+),
                    right=Int(ty=TokenType.INT(1))))
```

## TODOs

- [x] simple lexer
- [x] lexer proper untuk angka ( ) / * - * 
- [x] ast
- [x] simple parser untuk lexer sebelumnya
- [ ] lexing keyword, identifier, dll
- [ ] parser
- [ ] interpreter
