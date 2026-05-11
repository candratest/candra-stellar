#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec,
};

// Struktur data untuk menyimpan tugas mahasiswa
#[contracttype]
#[derive(Clone, Debug)]
pub struct StudentTask {
    id: u64,
    nama_mahasiswa: String,
    mata_kuliah: String,
    deskripsi_tugas: String,
    status: String,
}

// Storage key untuk data tugas mahasiswa
const TASK_DATA: Symbol = symbol_short!("TASKS");

#[contract]
pub struct StudentTaskContract;

#[contractimpl]
impl StudentTaskContract {
    // Fungsi untuk mengambil semua tugas mahasiswa
    pub fn get_tasks(env: Env) -> Vec<StudentTask> {
        env.storage()
            .instance()
            .get(&TASK_DATA)
            .unwrap_or(Vec::new(&env))
    }

    // Fungsi untuk membuat tugas mahasiswa baru
    pub fn create_task(
        env: Env,
        nama_mahasiswa: String,
        mata_kuliah: String,
        deskripsi_tugas: String,
    ) -> String {
        // 1. Ambil data tugas dari storage
        let mut tasks: Vec<StudentTask> = env
            .storage()
            .instance()
            .get(&TASK_DATA)
            .unwrap_or(Vec::new(&env));

        // 2. Buat object tugas baru
        let task = StudentTask {
            id: env.prng().gen::<u64>(),
            nama_mahasiswa,
            mata_kuliah,
            deskripsi_tugas,
            status: String::from_str(&env, "Belum selesai"),
        };

        // 3. Tambahkan tugas baru ke list tugas lama
        tasks.push_back(task);

        // 4. Simpan kembali data tugas ke storage
        env.storage().instance().set(&TASK_DATA, &tasks);

        String::from_str(&env, "Tugas mahasiswa berhasil ditambahkan")
    }

    // Fungsi untuk menghapus tugas berdasarkan id
    pub fn delete_task(env: Env, id: u64) -> String {
        // 1. Ambil data tugas dari storage
        let mut tasks: Vec<StudentTask> = env
            .storage()
            .instance()
            .get(&TASK_DATA)
            .unwrap_or(Vec::new(&env));

        // 2. Cari index tugas yang akan dihapus
        for i in 0..tasks.len() {
            if tasks.get(i).unwrap().id == id {
                tasks.remove(i);

                env.storage().instance().set(&TASK_DATA, &tasks);

                return String::from_str(&env, "Tugas mahasiswa berhasil dihapus");
            }
        }

        String::from_str(&env, "Tugas mahasiswa tidak ditemukan")
    }

    // Fungsi untuk mengubah status tugas menjadi selesai
    pub fn complete_task(env: Env, id: u64) -> String {
        let mut tasks: Vec<StudentTask> = env
            .storage()
            .instance()
            .get(&TASK_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..tasks.len() {
            let mut task = tasks.get(i).unwrap();

            if task.id == id {
                task.status = String::from_str(&env, "Selesai");

                tasks.set(i, task);
                env.storage().instance().set(&TASK_DATA, &tasks);

                return String::from_str(&env, "Status tugas berhasil diubah menjadi selesai");
            }
        }

        String::from_str(&env, "Tugas mahasiswa tidak ditemukan")
    }
}

mod test;