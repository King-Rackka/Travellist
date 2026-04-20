#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Kategori destinasi
#[contracttype]
#[derive(Clone, Debug)]
pub enum Category {
    Pantai,
    Gunung,
    Kota,
    Budaya,
    Alam,
    Kuliner,
}

// Tingkat prioritas
#[contracttype]
#[derive(Clone, Debug)]
pub enum Priority {
    High,
    Mid,
    Low,
}

// Struktur data destinasi perjalanan
#[contracttype]
#[derive(Clone, Debug)]
pub struct Destination {
    id: u64,
    name: String,
    country: String,
    category: Category,
    priority: Priority,
    visited: bool,
}

// Storage key
const TRAVEL_DATA: Symbol = symbol_short!("TRVL_DATA");

#[contract]
pub struct TravellistContract;

#[contractimpl]
impl TravellistContract {
    // Ambil semua destinasi
    pub fn get_destinations(env: Env) -> Vec<Destination> {
        return env
            .storage()
            .instance()
            .get(&TRAVEL_DATA)
            .unwrap_or(Vec::new(&env));
    }

    // Tambah destinasi baru
    pub fn add_destination(
        env: Env,
        name: String,
        country: String,
        category: Category,
        priority: Priority,
    ) -> String {
        let mut destinations: Vec<Destination> = env
            .storage()
            .instance()
            .get(&TRAVEL_DATA)
            .unwrap_or(Vec::new(&env));

        let destination = Destination {
            id: env.prng().gen::<u64>(),
            name,
            country,
            category,
            priority,
            visited: false,
        };

        destinations.push_back(destination);
        env.storage().instance().set(&TRAVEL_DATA, &destinations);

        return String::from_str(&env, "Destinasi berhasil ditambahkan");
    }

    // Tandai destinasi sebagai sudah dikunjungi / belum
    pub fn toggle_visited(env: Env, id: u64) -> String {
        let mut destinations: Vec<Destination> = env
            .storage()
            .instance()
            .get(&TRAVEL_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..destinations.len() {
            let mut dest = destinations.get(i).unwrap();
            if dest.id == id {
                dest.visited = !dest.visited;
                destinations.set(i, dest);
                env.storage().instance().set(&TRAVEL_DATA, &destinations);
                return String::from_str(&env, "Status kunjungan diperbarui");
            }
        }

        return String::from_str(&env, "Destinasi tidak ditemukan");
    }

    // Hapus destinasi berdasarkan id
    pub fn delete_destination(env: Env, id: u64) -> String {
        let mut destinations: Vec<Destination> = env
            .storage()
            .instance()
            .get(&TRAVEL_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..destinations.len() {
            if destinations.get(i).unwrap().id == id {
                destinations.remove(i);
                env.storage().instance().set(&TRAVEL_DATA, &destinations);
                return String::from_str(&env, "Destinasi berhasil dihapus");
            }
        }

        return String::from_str(&env, "Destinasi tidak ditemukan");
    }

    // Hitung jumlah total destinasi
    pub fn count_total(env: Env) -> u32 {
        let destinations: Vec<Destination> = env
            .storage()
            .instance()
            .get(&TRAVEL_DATA)
            .unwrap_or(Vec::new(&env));

        return destinations.len();
    }

    // Hitung jumlah destinasi yang sudah dikunjungi
    pub fn count_visited(env: Env) -> u32 {
        let destinations: Vec<Destination> = env
            .storage()
            .instance()
            .get(&TRAVEL_DATA)
            .unwrap_or(Vec::new(&env));

        let mut count: u32 = 0;
        for i in 0..destinations.len() {
            if destinations.get(i).unwrap().visited {
                count += 1;
            }
        }

        return count;
    }
}

mod test;