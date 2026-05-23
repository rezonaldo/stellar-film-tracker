#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Struktur data yang akan menyimpan informasi film
#[contracttype]
#[derive(Clone, Debug)]
pub struct Film {
    id: u64,
    title: String,
    genre: String,
}

// Storage key untuk data daftar film
const FILM_DATA: Symbol = symbol_short!("FILM_DATA");

#[contract]
pub struct FilmTrackerContract;

#[contractimpl]
impl FilmTrackerContract {
    // Fungsi untuk melihat semua film yang ada di daftar
    pub fn get_films(env: Env) -> Vec<Film> {
        // 1. ambil data film dari storage
        return env.storage().instance().get(&FILM_DATA).unwrap_or(Vec::new(&env));
    }

    // Fungsi untuk menambahkan film baru ke dalam daftar
    pub fn add_film(env: Env, title: String, genre: String) -> String {
        // 1. ambil data daftar film dari storage
        let mut films: Vec<Film> = env.storage().instance().get(&FILM_DATA).unwrap_or(Vec::new(&env));
        
        // 2. Buat objek film baru
        let new_film = Film {
            id: env.prng().gen::<u64>(), // Menghasilkan ID unik acak
            title: title,
            genre: genre,
        };
        
        // 3. tambahkan film baru ke daftar yang sudah ada
        films.push_back(new_film);
        
        // 4. simpan kembali daftar film ke storage
        env.storage().instance().set(&FILM_DATA, &films);
        
        return String::from_str(&env, "Film berhasil ditambahkan ke daftar!");
    }

    // Fungsi untuk menghapus film dari daftar berdasarkan ID
    pub fn remove_film(env: Env, id: u64) -> String {
        // 1. ambil data film dari storage 
        let mut films: Vec<Film> = env.storage().instance().get(&FILM_DATA).unwrap_or(Vec::new(&env));

        // 2. cari index film yang akan dihapus menggunakan perulangan
        for i in 0..films.len() {
            if films.get(i).unwrap().id == id {
                films.remove(i);

                // 3. Simpan perubahan ke storage
                env.storage().instance().set(&FILM_DATA, &films);
                return String::from_str(&env, "Berhasil menghapus film dari daftar");
            }
        }

        return String::from_str(&env, "Film tidak ditemukan di daftar")
    }
}

mod test;