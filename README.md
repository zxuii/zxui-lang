# Zxui Programming Language

## Pendahuluan

Zxui adalah interpreter berbasis AST tree-walk yang cukup lambat tapi mudah untuk diimplementasikan dibanding stack-based/register-based vm. Untuk feature, Zxui memang kalah, karena memang bukan dibuat untuk prod-use. Ini adalah proyek educational dan untuk konten di youtube saya.

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

```bash
cargo build --release
```

tunggu beberapa saat dan kamu akan mendapatkan binaries di `target/release/zxui` bisa kamu jalankan langsung dari sana atau copy ke mana gitu terserah.

kalau gamau ribet tinggal:

```bash
cargo r -r [..args]
```
ini otomatis build dan run executable.

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
              | "{" block "}"
              | identifier (assign_op expr | factor_tail*)
              | return_stmt
              | import_stmt
              | if_stmt
              | while_stmt
              | for_stmt
              | "break"
              | "continue"
              | expr

fun_decl    ::= "fun" identifier "(" params ")" "{" block "}"
var_decl    ::= "let" identifier "=" expr
var_assign  ::= expr ("=" | "+=" | "-=" | "*=" | "/=") expr
return_stmt ::= "return" [expr]
import_stmt ::= "import" string
if_stmt     ::= "if" expr "{" block "}" ["else" ("{" block "}" | if_stmt)]
while_stmt  ::= "while" expr "{" block "}"
for_stmt    ::= "for" identifier "in" expr "{" block "}"

params      ::= [identifier ("," identifier)*]
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

di Zxui, kamu WAJIB untuk membuat `project` dulu dibanding pakai single-file untuk menggunakan import statement. Kenapa? Agar lokasi importnya jelas. yaitu berdasarkan lokasi `root.zxui` berada.

Jadi, alasan kenapa harus pakai prefix schema `root:` di import string, alasannya karena itu.

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

## Builtin modules dan Builtin functions

di Zxui, ada builtin module bernama `raylib`, walau masih belum lengkap bindingnya, tapi sudah bisa membuat game topdown sederhana. dan APInya kurang lebih sama persis seperti raylib tapi beda di penamaannya aja. untuk sekarang, API/fungsi yang tersedia ada di contoh kode ini:

```swift
import "builtin:raylib"

let width = 800
let height = 600
let speed = 400

raylib.initWindow(width, height, "Top Down Zxui")

let player = {
    x = width / 2,
    y = height / 2,
    w = 50,
    h = 50,
}

while !raylib.windowShouldClose() {
    let dt = raylib.getFrameTime()

    raylib.beginDrawing()

    raylib.clearBackground("white")

    if raylib.isKeyDown("w") {
        player.y -= speed * dt
    }
    if raylib.isKeyDown("a") {
        player.x -= speed * dt
    }
    if raylib.isKeyDown("s") {
        player.y += speed * dt
    }
    if raylib.isKeyDown("d") {
        player.x += speed * dt
    }

    raylib.drawRectangle(player.x, player.y, player.w, player.h, "red")

    raylib.endDrawing()
}

raylib.closeWindow()
```

Untuk builtin module, pakai prefix schema builtin ya.

untuk Builtin/Native functions ada:

```js
println(...args)
print(...args)
readline(prefix)
typeof(x)
number(x)
string(x)
boolean(x)
push(x, value)
pop(x)
len(x)
remove(x, idx)
range(stop)
range(start, stop)
range(start, stop, step)
keys(x)
values(x)
hasKey(x)
clear(x)
```

## CRUD Example

Yaa ini example CRUD Todo-App di Zxui:
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

## Special Thanks To

Jujur aja untuk crafting interpreters ini bukunya bagus, tapi.... tapi nih ya... aku itu malas banget bacanya soalnya itu tuh puanjang banget dan ngebosenin banget jujur... jadi aku lebih milih liat kode kode dari repo orang orang ini atau sekedar mencari di google seperti bagaimana caranya A dan bagaimana caranya B kenapa bisa A kenapa tidak B dan hal-hal lain seperti itu. karena didalam bahasa pemrograman tak menyimpan satu konsep saja. tapi banyak konsep, dan seluruh konsep tersebut itu terlalu rapi, hingga aku bingung sekali harus mulai dari mana. maka dari itu aku melihat dari paling akar hingga paling pucuk.

- [ramtinJ95/rj-pylox](https://github.com/ramtinJ95/rj-pylox) - referensi awal untuk membuat bahasa ini di python
- [rspivak/lsbasi](https://github.com/rspivak/lsbasi) - gatau ini bahasa apa, gw cuma cari tahu cara membuat parser dari repo ini
- [craftinginterpreters.com](https://craftinginterpreters.com/the-lox-language.html) - sangat disarankan untuk membaca buku ini karena sangat bagus, bahasanya mudah dimengerti dengan banyak quote motivasi didalamnya.
- [jeschkies/lox-rs](https://github.com/jeschkies/lox-rs) - thanks kepada repo ini yang dimana telah membantuku bermigrasi dari python ke rust.

## Video

Niatnya aku mau mempublikasikan proses pembuatan bahasa pemrograman ini di youtube channel [@TelurTerbulat](https://youtube.com/@TelurTerbulat). yaa semoga aja aku punya niat ngedit wkwk.

## Next TODO

Pengenalan: ini adalah section khusus yang akan terus berubah seiring berjalannya waktu, disini akan menceritakan apa aja yang aku lakukan dan apa yang aku ingin lakukan.

Aku ingin membuat sistem FFI di bahasa pemrograman ini secara langsung, jadi aku tak perlu lagi menyentuh builtins.rs untuk melakukan FFI terhadap raylib dkk.
Sebenarnya aku bimbang untuk membuat diantara ketiga ini:
- FFI
- CLASS
- BYTECODE

Jadi aku langsung aja melakukan sebuah GAMBLE:

dan didapatkan hasilnya Bytecode.... okelah. aku akan buat compiler + vm.

## Penutup

Singkatnya, bahasa pemrograman ini masih berstatus belum selesai dan masih terus akan diupdate sampai kapanpun aku mood.
