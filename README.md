# OS110_Assignment2_MediumExercism

#### Nama             : Ari Purnama Aji
#### Username Exercism: AriPurnamaAji

#### Daftar problem yang telah saya selesaikan:
- Hamming
- Isogram
- Perfect Number
- Pythagorean Triplet
- Triangle

Esai ini dibuat untuk memenuhi tugas mata kuliah Sistem Operasi di semester 110, yang membahas salah satu pendekatan penyelsaian masalah pada **Rust medium exercism**. Pada kesempatan kali ini saya akan membahas bagaimana solusi saya dalam menyelasaikan **Perfect-Number problem**.

# Perfect Number
## The Problem
Problem ini meminta kita untuk menentukan apakah bilangan sempurna, berlimpah, atau kurang berdasarkan skema klasifikasi Nicomachus (60 - 120 M) untuk bilangan asli.
1. **Perfect (Sempurna)**, jumlah alikuot = bilangan.
2. **Abundant (Berlimpah)**, jumlah alikuot > bilangan.
3. **Deficient (Kurang)**, jumlah alikuot < bilangan.

## My Solution
Dalam menyelsaikan problem ini, pertama saya memikirkan terlebih dahulu bagaimana saya mendapatkan faktor-faktor dari suatu bilangan sempurna yang nantinya akan dijumlah, yang disebut juga dengan **jumlah alikuot**. Kita harus pahami betul definisi dari bilangan sempurna agar dapat mengetahui faktor-faktornya tersebut.

Dalam matematika, bilangan  sempurna adalah bilangan bulat positif yang merupakan penjumlahan dari pembagi positif sejati, yaitu penjumlahan dari pembagi positif tidak termasuk bilangan itu sendiri. Arti lainnya, bilangan sempurna adalah bilangan yang merupakan setengah penjumlahan dari semua pembagi positif (termasuk bilangan itu sendiri), atau σ (n) = 2n. (Dikutip dari : https://mon26harista.wordpress.com/2012/05/04/perfect-number-bilangan-sempurna/)

Mulai dari situlah saya dapat memahami bagaimana saya mendapatkan faktor-faktor dari suatu bilangan tersebut. 

### Fungsi Classification
```Rust
#[derive(Debug, PartialEq, Eq)]

pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}
```

### Penjelasan Fungsi Classification
Fungsi tersebut merupakan fungsi yang berisi klasifikasi ketetapan nilai untuk menentukan bahwa bilangan tersebut apakah merupakan bilangan berlimpah (abudant), bilangan sempurna (perfect) atau bilangan kurang (deficient).

### Fungsi classify
```Rust
pub fn classify(num: u64) -> Option<Classification> {
    //unimplemented!("classify {}", num);
    let mut f:Vec<u64> = Vec::new();
    for i in 1..(num/2)+1 {
        if num % i == 0 {
            f.push(i)
        }
    }

    let mut aliquot_sum:u64 = 0;
    for i in f.iter() {
        aliquot_sum = aliquot_sum + i;
    }
    if num == 0 {
        None
    }
    else if aliquot_sum == num {
        Some(Classification::Perfect)
    }
    else if aliquot_sum > num {
        Some(Classification::Abundant)
    }
    else if aliquot_sum < num {
        Some(Classification::Deficient)
    }
    else {
        None
    }
}
```

### Penjelasan Fungsi classify
1. Pertama di dalam fungsi `classify` saya memebuat `Vec` kosong yang bertipe data `u64` bernama `f` yang mutable untuk menampung faktor-faktor yang didapat nantinya. 
2. Selanjutnya saya membuat `for` untuk mencari faktor-faktornya dengan cara mengecek setiap bilang dari 1 sampai setengah penjumlahan dari semua pembagi positif (termasuk bilangan itu sendiri) yang dapat diimplementasikan sebagai berikut `1..(num/2)+1`.
3. Jika bilangan habis dibagi dengan nilai `i` maka nilai `i` merupakan sebuah faktor yang nantinya dimasukkan kedalam Vector `f`.
4. Membuat variable `aliquot_sum` bertipe data `u64` yang mutable untuk menghitung jumlah dari faktor-faktor yang telah didapat.
5. Membuat `for` untuk menjumlahkan setiap faktor yang didapat dari dalam Vector `f`.
6. Karena fungsi ini untuk mengecek bilangan berdasarkan ketetapan nilai yang telah dibuat di dalam fungsi `Classification`, maka keluarannya adalah `Option<Classification>`.
7. Terakhir kita masuk kedalam tahap pengecekan, ada 5 tahap pengecekan: 1. Pertama kita cek apakah bilangan tersebut bernilai 0 atau tidak, jika iya return `None` yang artinya dia tidak masuk kedalam klasifikasi yang telah ditentukan pada fungsi `Classification`, 2. Bila jumlah alikuot = bilangan, maka bilangan masuk ke dalam klasifikasi bilangan sempurna (perfect), 3. Bila jumlah alikuot > bilangan, maka bilangan masuk ke dalam klasifikasi bilangan berlimpah (abundant), 4. Bila jumlah alikuot < bilangan, maka bilangan masuk ke dalam klasifikasi bilangan kurang (deficient), 5. Selain dari keempat pengecekan tersebut, maka return `None`.

## Full Code
```Rust
#[derive(Debug, PartialEq, Eq)]

pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    //unimplemented!("classify {}", num);
    let mut f:Vec<u64> = Vec::new();
    for i in 1..(num/2)+1 {
        if num % i == 0 {
            f.push(i)
        }
    }

    let mut aliquot_sum:u64 = 0;
    for i in f.iter() {
        aliquot_sum = aliquot_sum + i;
    }
    if num == 0 {
        None
    }
    else if aliquot_sum == num {
        Some(Classification::Perfect)
    }
    else if aliquot_sum > num {
        Some(Classification::Abundant)
    }
    else if aliquot_sum < num {
        Some(Classification::Deficient)
    }
    else {
        None
    }
}

//Original Problem Link : https://exercism.io/tracks/rust/exercises/perfect-numbers/solutions/b68dd705d8a5493a93106b8689615616
//Reference : https://mon26harista.wordpress.com/2012/05/04/perfect-number-bilangan-sempurna/
```
