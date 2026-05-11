# Smart Contract List Tugas Mahasiswa

Smart contract ini dibuat menggunakan **Soroban SDK** untuk mengelola daftar tugas mahasiswa.  
Contract ini dapat digunakan untuk menambahkan tugas, melihat semua tugas, menghapus tugas berdasarkan ID, dan menandai tugas sebagai selesai.

## Fitur

- Menampilkan semua daftar tugas mahasiswa
- Menambahkan tugas mahasiswa baru
- Menghapus tugas berdasarkan ID
- Mengubah status tugas menjadi selesai

## Struktur Data

Data tugas mahasiswa disimpan dalam struct `StudentTask`.

```rust
pub struct StudentTask {
    id: u64,
    nama_mahasiswa: String,
    mata_kuliah: String,
    deskripsi_tugas: String,
    status: String,
}

Contract ID = CA7X32W2XFMIMJDSCVHJJCU4HKUS6DPW3QXGFDQM3KGHIMR37OS6QCW6