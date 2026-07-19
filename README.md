# Zxui Programming Language

## Pendahuluan

Zxui adalah interpreter berbasis Tree-Walk AST yang cukup lambat tapi mudah untuk diimplementasikan dibanding stack-based/register-based vm. Untuk feature, Zxui memang kalah, karena memang bukan dibuat untuk prod-use. Ini adalah proyek educational dan untuk konten di youtube saya.

Kalau masih belum paham apa itu Tree-Walk AST atau lebih dikenal dengan AST Interpreter:

> An abstract syntax tree interpreter transforms source code into an abstract syntax tree (AST), then interprets it directly, or uses it to generate native code via JIT compilation. In this approach, each sentence needs to be parsed just once.

> As an advantage over bytecode, AST keeps the global program structure and relations between statements (which is lost in a bytecode representation), and when compressed provides a more compact representation. Thus, using AST has been proposed as a better intermediate format than bytecode.

> However, for interpreters, AST results in more overhead than a bytecode interpreter, because of nodes related to syntax performing no useful work, of a less sequential representation (requiring traversal of more pointers) and of overhead visiting the tree.[11]

Atau versi indonesia:

> Interpreter Abstract Syntax Tree (AST) mengonversi source code menjadi struktur AST, lalu mengeksekusinya secara langsung (interpretasi langsung) atau menggunakannya untuk men-generate native code melalui kompilasi JIT. Dengan pendekatan ini, setiap statement hanya perlu di-parse satu kali saja. 

> Sebagai kelebihan dibanding bytecode, AST tetap menjaga struktur program global dan hubungan antar statement (yang biasanya hilang dalam representasi bytecode). Selain itu, ketika dikompresi, AST memberikan representasi yang lebih ringkas (concise). Oleh karena itu, penggunaan AST sering diusulkan sebagai intermediate format yang lebih baik daripada bytecode.

> Namun, untuk interpreter, AST memberikan beban tambahan (overhead) yang lebih besar dibandingkan interpreter bytecode. Hal ini disebabkan oleh adanya node terkait sintaksis yang sebenarnya tidak melakukan proses komputasi apa pun, representasi yang kurang sekuensial (sehingga membutuhkan lebih banyak pointer traversal), serta adanya overhead tambahan yang terjadi saat melakukan traversal pada tree tersebut.

Sumber: [wikipedia.org](https://en.wikipedia.org/wiki/Interpreter_(computing))

Walaupun bahasa ini sangatlah 'sempit' tapi kita sudah bisa membuat beberapa hal di bahasa ini lho~! 

- CRUD? bisa!!
- Game sederhana? bisa!!
- .. yahh mungkin cuma itu

## Menjalankan

Kamu bisa menjalankan bahasa pemrograman ini dengan beberapa cara.

### Pertama - langsung kasih file

Kamu bisa aja langsung ngasih file seperti:
```bash
zxui file.zxui
```
Tetapi metode ini ada kekurangan fatal, yaitu tidak bisa pakai `import` statement sama sekali! alasannya akan dijelaskan nanti.

### Kedua - dengan pakai `run` command

Kamu bisa pakai 'run' command sebelum path atau pakai run command tanpa path tapi sudah berada di project path. contoh:
```bash
zxui run projectku/
```
Itu contoh kasus apabila kamu me-run project di direktori yang berbeda dari projectmu. ini contoh kasus kalau kamu di dalam direktori projectmu:
```bash
zxui run
```
Simple bukan? bisa aja sih kamu tulis `.` disana yaa ujung-ujungnya sama aja sih.

## Membuat proyek

Kamu bisa membuat proyekmu dengan sesederhana menjalankan perintah:
```bash
zxui init
```
lakukan itu di direktori yang kamu mau ATAU kamu bisa langsung menulis path:

```bash
zxui init projectku
```
Sederhana, tentunya.

## Building

Sebenarnya ini bisa dilakukan di multiplatform. tapi mungkin belum bisa pakai raylib things dulu karena belum gw buat agar bisa di seluruh device, tapi harusnya bahasanya masih bisa dipake di segala device yang ada rust compiler.

```ps1
.\build.bat
```
Itu saja. dan kamu langsung bisa melihat `zxui.exe` di root project.

Install rust bagi yang belum punya: [Install rust disini](https://rust-lang.org/tools/install/)

## Language References

Nah kita sampai ke bagian bahasa pemrogramannya itu sendiri. yep, `Language References`.

## Formal grammar

secara umum, formal grammar untuk Zxui adalah sebagai berikut:

```ebnf
program     ::= "program" block

block       ::= stmt*

stmt        ::= fun_decl
              | var_decl
              | class_decl
              | "{" block "}"
              | (identifier | "self") factor_tail* (assign_op expr)?
              | return_stmt
              | import_stmt
              | if_stmt
              | while_stmt
              | for_stmt
              | "break"
              | "continue"
              | expr

fun_decl    ::= "fun" identifier "(" params ")" "{" block "}"
class_decl  ::= "class" identifier [":" identifier] "{" class_block "}"
class_block ::= (fun_decl | "static" fun_decl)*

var_decl    ::= "let" identifier "=" expr
var_assign  ::= expr ("=" | "+=" | "-=" | "*=" | "/=") expr
return_stmt ::= "return" [expr]              (* only valid inside fun_decl body *)
import_stmt ::= "import" string              (* only valid at top-level (depth 0) *)
if_stmt     ::= "if" expr "{" block "}" ["else" ("{" block "}" | if_stmt)]
while_stmt  ::= "while" expr "{" block "}"
for_stmt    ::= "for" identifier "in" expr "{" block "}"

params      ::= [("self" | identifier) ("," identifier)*]
args        ::= [expr ("," expr)*]

expr        ::= logical_and ("or" logical_and)*
logical_and ::= comparison ("and" comparison)*
comparison  ::= additive (comp_op additive)*
additive    ::= term (("+" | "-") term)*
term        ::= unary (("*" | "/") unary)*
unary       ::= ("+" | "-" | "!") unary
              | factor

factor      ::= primary factor_tail*
factor_tail ::= "(" args ")"
              | "[" expr "]"
              | "." identifier

primary     ::= number
              | string
              | "null"
              | "true"
              | "false"
              | "(" expr ")"
              | "[" array_literal "]"
              | "{" map_literal "}"
              | "self"
              | "super" "." identifier
              | identifier

array_literal ::= [expr ("," expr)*]
map_literal   ::= [key_value ("," key_value)* [","]]
key_value     ::= identifier "=" expr

comp_op     ::= "<" | ">" | "<=" | ">=" | "==" | "!="
assign_op   ::= "=" | "+=" | "-=" | "*=" | "/="
```

Literally ini semua yang ada di bahasa ini. seluruh sintaksnya keexpose disini sih, tapi kita breakdown satu persatu ya.

## Tipe data

di Zxui, ada beberapa tipe data. Antara lain:

- `Number` - equivalent dengan tipe `f64` di rust.
- `String` - equivalent dengan tipe `String` di rust.
- `Boolean` - equivalent dengan tipe `bool` di rust.
- `Array` - equivalent dengan tipe `Vec` di rust, tetapi dibungkus lagi dengan Rc dan RefCell agar bisa di mutate dengan gampang.
- `Map` - equivalent dengan tipe `IndexMap` di rust dari crate indexmap, dan tentunya ordered (berurutan, tak seperti table di lua atau hashmap di rust). namun tetap dibungkus dengan Rc dan RefCell agar bisa dimutate.
- `Function` - equivalent dengan enum struct berisi nama, parameter, body dan closure.
- `Native Function` - agar bisa membinding antara fungsi di rust ke fungsi di bahasa Zxui
- `Null` - null value

Contoh tiap tipe data:
```swift
123.456 // Number
"Hello World" // String
true /*or*/ false // Boolean
[1,2,3,4] // Array
{ key = "value" } // Map
fun a() {} // Function (stmt, tapi bisa dipass sebagai `a` alias fungsi.)
println() // salah satu dari beberapa native function
null // null value
```

## Variables and Assignments

Di Zxui, untuk sekarang hanya ada satu cara mendefinisikan sebuah variable:
```swift
let name = expression;
```
Contoh:
```swift
let nama = "Pak Dadang";
let mahasiswa = {
    semester = 1,
    status = "maba",
    umur = 18,
};
```

Kamu bisa melakukan assignments dengan cara:
```swift
identifier = new_expression;
```
Contoh:
```swift
let nama = "John";
nama = "Alice";

// bisa juga compound assignment seperti:
nama += " Greyrat"; // nama = Alice Greyrat
```

Jenis-jenis compound assignment:
```swift
+=  -=  *=  /=
```

## Operasi Arithmetic, Unary, Logical dan Comparison

Berikut jenis-jenis operationnya:
```swift
// arithmetic
+  -  *  / 

// unary
+  -  !

// logical
and  or

// comparison
==  !=  <  >  <=  >=
```

## Function declaration, Return statement dan Function call

Cara membuat fungsi di Zxui sama seperti di kotlin, bedanya tanpa type notation sama sekali:
```swift
// declaration
fun greet(name) {
    // return statement
    return "Hello, " + name + "!"
}

// call
let greeter = greet("Pak Asep");
println(greeter)
```

## Block statement

Di Zxui, setiap block (`{` `}`) itu membuat scope baru. berbeda dengan `Python` yang scopenya itu berdasarkan fungsi.

Contoh:
```swift
{
    {
        let x = 5
    }

    println(x) // ini akan error undefined variable.
}
```

## If statement

Yaa sama saja seperti bahasa lainnya, optional parenthesis:

```swift
let x = 5
if x > 5 {
    // ..
} else if x < 5 {
    // ..
} else {
    // ..
}
```

## Loops

Di Zxui, ada dua jenis loop. yaitu `For-in-loop` dan `While-loop`:

```swift
// for-in-loop
for i in range(5) {
    println(i)
}

for i in [1,2,3,4,5] {
    println(i)
}

for m in { x = 5, y = 2 } {
    println(m.key, " = ", m.val) // m otomatis dapat .key dan .val
}

// while loop
while true {
    // ..
}
```

Tentunya, support continue dan break.

```swift
let x = 0

while true {
    x += 1
    if x >= 5 {
        continue
    }
    if x == 10 {
        break
    }
}
```

## Import statement

Nah di bahasa pemrograman Zxui, import statement ini memang harus di jelaskan sih behaviornya. seperti ini sederhananya:

di Zxui, kamu wajib membuat sebuah project apabila ingin menggunakan import statement. Ini karena memang by design, untuk tidak memusingkan user kalau relative by script location. Jadi jelas dia itu diambil dari mana.

Contoh di project `example`:

```
example/
    - root.zxui
    - main.zxui
    - math.zxui
```

`root.zxui`:
```swift
let project = {
    name = "example",
    main = "main.zxui",
}
```

`math.zxui`:
```swift
fun square(x) {
    return x * x
}
```

`main.zxui`:
```swift
import "root:math"

println("Hasil math.square(5): ", math.square(5))
```

Jika kita menjalankan `zxui run` maka akan otomatis keluar output seperti:

```bash
Hasil math.square(5): 25
```

INGAT! command `run` mengekspektasikan di direktori saat ini ATAU direktori yang ditentukan memiliki file `root.zxui` kalau tidak ada maka tidak bisa dipakai. dan wajib berisi variable project dengan map `name` dan `main`.

## OOP / Class

DI bahasa pemrograman Zxui sekarang sudah bisa melakukan OOP-things seperti membuat constructor, dan lain sebagainya (aku ga terlalu paham OOP karena ga biasa tentang begituan, seringnya sih make C dan Rust jadi ya... OOP ini ala kadarnya).

Oke, lanjut ke penjelasannya saja.

Di Zxui, kita butuh `fun init()` sebagai sebuah constructor. Kamu tidak butuh lagi sebuah `self` di parameter pertama fungsi ini karena sudah di 'inject' otomatis ke dalam setiap fungsi didalam class atau bisa dibilang `instance method`.

Contoh:

```swift
class Person {
    fun init(nama, kelas) {
        self.nama = nama
        self.kelas = kelas
    }

    fun greet() {
        // Halo, Hafidh! Kamu kelas 10 PPLG-1
        println("Halo, ", self.nama, "! ", "Kamu kelas ", self.kelas, "!")
    }

    static fun default() {
        return Person("Hafidh", "10 PPLG-1")
    }
}
```

Seperti contoh diatas, kamu tak butuh `self` sebagai parameter pertama sudah bisa langsung menggunakan `self`. Kamu juga bisa langsung membuat sebuah `instance method` dengan membuat sebuah fungsi seperti biasanya saja, tentunya tak perlu `self` sebagai parameter.

Kalau kamu ingin membuat sebuah `static method`, kamu butuh keyword tambahan bernama `static` untuk membuatnya. Contohnya seperti method `default()` diatas.

Untuk pemanggilan, cukup:

```swift
Person("John", "10 PPLG-1")
```

Sederhana bukan? yap tentunya.

Nah selanjutnya, ada yang namanya `inheritance`. di Zxui, saat ini hanya mengizinkan satu `inherit` per-class.

Contoh:

```swift
class Animal {
    fun init() {
        self.type = "Dog";
        self.say = "Woof!";
    }

    fun speak() {
        println("The ", self.type, " say ", self.say);
    }
}

class Cat : Animal {
    fun init() {
        self.type  = "Cat";
        self.say = "Meow!";
    }
}

let a = Animal()
a.speak() // The Dog say Woof!
let c = Cat()
c.speak() // The Cat say Meow!
```

Ya... sesederhana itu. kamu bisa memanggil `super.method()` juga.

## Builtin functions

di Zxui, ada beberapa builtin functions yang tersedia di dalamnya. 
Contoh Builtin functions yang ada:

```js
println(...args)
print(...args)
readline(prefix)
typeof(x)
range(stop)
range(start, stop)
range(start, stop, step)
```

## 'std' Module Schema

Di Zxui, saat ini hanya ada satu module di schema `std` atau biasa disebut sebagai `Standard Library`, Yaitu `ffi`. `Standard Library` ini dapat langsung memanggil fungsi bahasa `C` dari Zxui. untuk contohnya sendiri seperti berikut:

```swift
import "std:ffi"

let lib = ffi.load("./lib/raylib.dll")

lib.struct("Color", { r = "u8", g = "u8", b = "u8", a = "u8" })
// KAMU BISA LANGSUNG MENGGUNAKAN STRUCTNYA
// SEBAGAI PARAMETER ATAU RETURN VALUE!
lib.declare("ColorFromHSV", ["f32", "f32", "f32"], "Color") 
lib.declare("InitWindow", ["i32", "i32", "str"], "void")

// kalau mau memanggil fungsi hanya sesederhana:
lib.InitWindow(800, 600, "Hello World!")
// HANYA lib.namafungsi(...args)
```
> PENTING: urutan field di `lib.struct()` HARUS sama persis dengan urutan field di definisi struct C aslinya (cek header .h library tersebut). Alignment/padding dihitung otomatis mengikuti aturan C standar, tapi urutan yang salah akan menghasilkan data yang korup tanpa pesan error.

> CATATAN: untuk `ffi.load`, itu path nya relatif terhadap `root.zxui`. Biar kamu tidak pusing-pusing lagi ya....

Untuk daftar setiap tipe yang ada untuk ffi:

|FFI|Equivalent C|
----------|----------
`"void"`  | `void`
`"i8"`    | `char` 
`"u8"`    | `unsigned char`
`"i16"`   | `short` 
`"u16"`   | `unsigned short` 
`"i32"`   | `int` 
`"u32"`   | `unsigned int` 
`"i64"`   | `long`
`"u64"`   | `unsigned long` 
`"f32"`   | `float`
`"f64"`   | `double`
`"bool"`  | `_Bool`
`"str"`   | `const char*`
`"ptr"`   | `void*`

Kalau ada tipe yang selain dari itu (atau struct), bisa langsung tulis nama structnya. TAPI kamu harus mendefinisikannya dulu ya!

Dan omong-omong, struct kalau direpresentasikan di Zxui, itu adalah sebuah `Map`. Dan didalamnya memiliki sebuah `"reserved property"` bernama `__struct_name__`. Itu digunakan agar Zxui tahu apakah tipenya valid atau tidak. TETAPI kamu bisa membuat `"struct"` manual tanpa `.struct()` method:

```swift
let warna = { r = 255, g = 0, b = 0, a = 255 }
```

TAPI INGAT: kalau tanpa `__struct_name__`, maka Zxui akan menganggap struct itu sepenuhnya BENAR walau sebenarnya tidak. Yang mengakibatkan error message yang sangat tidak jelas nantinya. INGAT itu.

Contoh penggunaan lengkap:
```swift
import "std:ffi"

let raylib = ffi.load("./lib/raylib.dll")

raylib.declare("InitWindow", ["i32", "i32", "str"], "void")
raylib.declare("WindowShouldClose", [], "bool")
raylib.declare("GetFrameTime", [], "f32")
raylib.declare("BeginDrawing", [], "void")
raylib.declare("EndDrawing", [], "void")
raylib.declare("ClearBackground", ["Color"], "void")
raylib.declare("DrawRectangle", ["i32", "i32", "i32", "i32", "Color"], "void")
raylib.declare("IsKeyDown", ["i32"], "bool")
raylib.declare("CloseWindow", [], "void")

raylib.struct("Color", { r = "u8", g = "u8", b = "u8", a = "u8" })

let white = { r = 255, g = 255, b = 255, a = 255 }
let red = { r = 230, g = 41, b = 55, a = 255 }

let KEY_W = 87
let KEY_A = 65
let KEY_S = 83
let KEY_D = 68

let width = 800
let height = 600
let speed = 400

raylib.InitWindow(width, height, "Top Down Zxui")

let player = {
    x = width / 2,
    y = height / 2,
    w = 50,
    h = 50,
}

while !raylib.WindowShouldClose() {
    let dt = raylib.GetFrameTime()

    raylib.BeginDrawing()

    raylib.ClearBackground(white)

    if raylib.IsKeyDown(KEY_W) {
        player.y -= speed * dt
    }
    if raylib.IsKeyDown(KEY_A) {
        player.x -= speed * dt
    }
    if raylib.IsKeyDown(KEY_S) {
        player.y += speed * dt
    }
    if raylib.IsKeyDown(KEY_D) {
        player.x += speed * dt
    }

    raylib.DrawRectangle(player.x, player.y, player.w, player.h, red)

    raylib.EndDrawing()
}

raylib.CloseWindow()
```

## CRUD Example

Yaa ini example CRUD Todo-App di Zxui:
```swift
class Todo {
    fun init(nama) {
        self.nama = nama;
        self.selesai = false;
    }

    fun toggle() {
        if self.selesai {
            self.selesai = false;
        } else {
            self.selesai = true;
        }
    }

    fun tampilkan(nomor) {
        let tanda = "[ ]";
        if self.selesai {
            tanda = "[x]";
        }
        println(nomor, ". ", tanda, " ", self.nama);
    }
}

class TodoApp {
    fun init() {
        self.todos = [];
        self.running = true;
    }

    fun tampilMenu() {
        println("====================");
        println("      TODO APP      ");
        println("====================");
        println("1. Lihat todos");
        println("2. Tambah todo");
        println("3. Hapus todo");
        println("4. Tandai selesai/belum");
        println("5. Keluar");
        println("====================");
    }

    fun tampilTodos() {
        if self.todos.len() == 0 {
            println("Belum ada todo!");
        } else {
            let i = 0;
            for todo in self.todos {
                i += 1;
                todo.tampilkan(i);
            }
        }
    }

    fun cekNomor(idx) {
        return idx < 0 or idx >= self.todos.len();
    }

    fun tambahTodo() {
        let input = readline("Masukan todo: ");
        self.todos.push(Todo(input));
        println("Todo berhasil ditambahkan!");
    }

    fun hapusTodo() {
        if self.todos.len() == 0 {
            println("Belum ada todo yang bisa dihapus!");
        } else {
            self.tampilTodos();
            let input = readline("Hapus nomor berapa? ");
            let idx = Number(input) - 1;
            if self.cekNomor(idx) {
                println("Nomor tidak valid!");
            } else {
                self.todos.remove(idx);
                println("Todo berhasil dihapus!");
            }
        }
    }

    fun toggleTodo() {
        if self.todos.len() == 0 {
            println("Belum ada todo!");
        } else {
            self.tampilTodos();
            let input = readline("Tandai nomor berapa? ");
            let idx = Number(input) - 1;
            if self.cekNomor(idx) {
                println("Nomor tidak valid!");
            } else {
                let todo = self.todos[idx];
                todo.toggle();
                println("Status todo berhasil diubah!");
            }
        }
    }

    fun run() {
        while self.running {
            self.tampilMenu();
            let pilihan = readline("Pilih menu: ");

            if pilihan == "1" {
                self.tampilTodos();
            } else if pilihan == "2" {
                self.tambahTodo();
            } else if pilihan == "3" {
                self.hapusTodo();
            } else if pilihan == "4" {
                self.toggleTodo();
            } else if pilihan == "5" {
                println("Sampai jumpa!");
                self.running = false;
            } else {
                println("Pilihan tidak valid!");
            }
        }
    }
}

let app = TodoApp();
app.run();
```

## ASPEK TEKNIS (lewati kalau males)

Nah sebenarnya ada banyak sekali kompleksitas yang disembunyikan dari semua proses ini. Begitu juga dengan bahasa pemrograman lainnya, tampak dari depan terlihat sederhana tapi dibalik layar sebenarnya tak sesederhana yang kelihatannya.

Singkatnya, didalam sebuah Interpreter (disini berjenis AST Interpreter) ada rangkaian proses yang terpisah saat menjalankan sebuah program atau script.

Konsep yang biasanya kita pelajari secara umum adalah bahasa pemrograman itu mengubah dari bahasa yang dimengerti manusia menjadi bahasa yang dimengerti mesin. Ini memang tidak salah, tapi terlalu disederhanakan saja. Lebih detailnya, bahasa pemrograman dibagi menjadi dua jenis, ada yang di `Kompilasi` (bisa juga dibilang transpilasi, CMIIW) atau di `Interpretasi`.

Dari pada itu juga, bahasa pemrograman itu ada yang `Low-level` (Bahasa pemrograman yang dekat dengan mesin, contohnya Rust, C, C++) dan `High-Level` (Bahasa pemrograman yang tak perlu berpikir tentang manajemen memori, contohnya Python, Ruby, Lua dan bahkan Zxui sendiri).

Nah, di Interpreter sendiri terbagi menjadi dua bagian, ada `Frontend` dengan `Backend`. di frontend, ada beberapa proses, seperti:

- Membaca script dan disimpan disuatu tempat
- Mengubah teks yang ada di script menjadi serangkaian `Token` (Token adalah hasil dari perubahan sebuah karakter atau lebih menjadi sebuah hal yang mudah di modifikasi.) atau proses ini biasa disebut sebagai `Lexing` / `Lexer` atau `Tokenization`
- Mengubah serangkaian `Token` menjadi sebuah `AST` (Abstract Syntax Tree). Proses ini biasa disebut sebagai `Parsing` / `Parser`. Ada banyak teknik untuk melakukan parsing, salah satunya adalah recursive descent parser (Recursive Descent Parser adalah cara untuk mem-parsing dengan memanjat dari akar (root / bagian tertinggi) ke bagian terendah)
- Kalau AST Interpreter, maka kita langsung mengevaluate setiap `node` AST yang ada. TAPI kalau itu Bytecode interpreter seperti python, ruby, lua maka prosesnya akan berlanjut ke bagian Kompilasi menjadi sebuah `bytecode` (digunakan untuk dieksekusi dengan cepat oleh `vm` karena memangkas ukuran dan program menjadi linear karena tak perlu mentraverse AST lagi).

Untuk cara kerja kompilator kalian cari aja sendiri aku males nulis beginian lagi, cape woy ga ada yang baca juga buset dah... BTW, selain dari hal ini, sebenarnya yang ribet itu module standard library `ffi` sih, itu harus cari tahu bagaimana caranya agar bisa struct return by value lah, bagaimana caranya manggil `C` function lah dan sebagainya, aku males jelasin jadi ya... CARI TAU SENDIRI AJA KELEN DAH.

TYSM, CMIIW.

## Special Thanks To

Jujur aja untuk crafting interpreters ini bukunya bagus, tapi.... tapi nih ya... aku itu malas banget bacanya soalnya itu tuh puanjang banget dan ngebosenin banget jujur... jadi aku lebih milih liat kode kode dari repo orang orang ini atau sekedar mencari di google seperti bagaimana caranya A dan bagaimana caranya B kenapa bisa A kenapa tidak B dan hal-hal lain seperti itu. karena didalam bahasa pemrograman tak menyimpan satu konsep saja. tapi banyak konsep, dan seluruh konsep tersebut itu terlalu rapi, hingga aku bingung sekali harus mulai dari mana. maka dari itu aku melihat dari paling akar hingga paling pucuk.

- [ramtinJ95/rj-pylox](https://github.com/ramtinJ95/rj-pylox) - referensi awal untuk membuat bahasa ini di python
- [rspivak/lsbasi](https://github.com/rspivak/lsbasi) - gatau ini bahasa apa, gw cuma cari tahu cara membuat parser dari repo ini
- [craftinginterpreters.com](https://craftinginterpreters.com/the-lox-language.html) - sangat disarankan untuk membaca buku ini karena sangat bagus, bahasanya mudah dimengerti dengan banyak quote motivasi didalamnya.
- [jeschkies/lox-rs](https://github.com/jeschkies/lox-rs) - thanks kepada repo ini yang dimana telah membantuku bermigrasi dari python ke rust.

## Video

Niatnya aku mau mempublikasikan proses pembuatan bahasa pemrograman ini di youtube channel [@TelurTerbulat](https://youtube.com/@TelurTerbulat). yaa semoga aja aku punya niat ngedit wkwk.

## Penutup

Singkatnya, bahasa pemrograman ini masih berstatus belum selesai dan masih terus akan diupdate sampai kapanpun aku mood.
