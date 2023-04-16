use std::io;

fn main() {
    let mut parkir = [false; 10];
    loop {
        println!("Silakan pilih aksi:");
        println!("1. Parkir Mobil");
        println!("2. Keluarkan Mobil");
        println!("3. Keluar ");

        let mut pilihan = String::new();
        io::stdin().read_line(&mut pilihan).expect("Gagal membaca input");

        let pilihan: u32 = match pilihan.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match pilihan {
            1 => {
                let mut kosong = None;
                for (i, slot) in parkir.iter().enumerate() {
                    if !slot {
                        kosong = Some(i);
                        break;
                    }
                }
                match kosong {
                    Some(slot) => {
                        parkir[slot] = true;
                        println!("Mobil diparkir di slot {}", slot + 1);
                    }
                    None => println!("Maaf, semua slot parkir sudah terisi"),
                }
            }
            2 => {
                let mut slot = String::new();
                println!("Nomor slot mobil yang akan dikeluarkan:");
                io::stdin().read_line(&mut slot).expect("Gagal membaca input");

                let slot: usize = match slot.trim().parse() {
                    Ok(num) => num - 1,
                    Err(_) => continue,
                };

                if parkir[slot] {
                    parkir[slot] = false;
                    println!("Mobil dikeluarkan dari slot {}", slot + 1);
                } else {
                    println!("Slot {} kosong", slot + 1);
                }
            }
            3 => break,
            _ => println!("Pilihan tidak valid, silakan coba lagi"),
        }
    }
}