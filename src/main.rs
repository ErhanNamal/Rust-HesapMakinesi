use std::io::{self, Write};

#[derive(Debug)]
struct TodoItem {
    id: u32,
    text: String,
    done: bool,
}

fn main() {
    let mut todos: Vec<TodoItem> = Vec::new();
    let mut next_id: u32 = 1;

    loop {
        println!();
        println!("================ RUST DEMO PROGRAMI ================");
        println!("1) Yeni görev ekle");
        println!("2) Görevleri listele");
        println!("3) Görevi tamamlandı olarak işaretle");
        println!("4) Basit hesap makinesi");
        println!("5) Çıkış");
        print!("Seçiminiz: ");
        // stdout'u hemen göstermek için flush
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        if io::stdin().read_line(&mut choice).is_err() {
            println!("Girdi okunurken hata oluştu, lütfen tekrar deneyin.");
            continue;
        }

        match choice.trim() {
            "1" => {
                let text = read_line("Görev açıklamasını yazın: ");
                if text.trim().is_empty() {
                    println!("Boş görev eklenemez.");
                    continue;
                }
                let item = TodoItem {
                    id: next_id,
                    text,
                    done: false,
                };
                todos.push(item);
                println!("Görev eklendi (id = {}).", next_id);
                next_id += 1;
            }
            "2" => {
                if todos.is_empty() {
                    println!("Henüz hiç görev yok.");
                } else {
                    println!("------ Görevler ------");
                    for t in &todos {
                        let durum = if t.done { "✓" } else { " " };
                        println!("[{}] {} - {}", durum, t.id, t.text);
                    }
                }
            }
            "3" => {
                if todos.is_empty() {
                    println!("İşaretlenecek görev yok.");
                    continue;
                }
                let id_str = read_line("Tamamlandı olarak işaretlemek istediğiniz görev id'si: ");
                match id_str.trim().parse::<u32>() {
                    Ok(id) => {
                        if let Some(item) = todos.iter_mut().find(|t| t.id == id) {
                            item.done = true;
                            println!("Görev tamamlandı olarak işaretlendi.");
                        } else {
                            println!("Bu id'ye sahip bir görev bulunamadı.");
                        }
                    }
                    Err(_) => {
                        println!("Geçerli bir sayı girin.");
                    }
                }
            }
            "4" => run_calculator(),
            "5" => {
                println!("Programdan çıkılıyor. Hoşça kalın!");
                break;
            }
            _ => {
                println!("Geçersiz seçim, lütfen 1-5 arasında bir değer girin.");
            }
        }
    }
}

fn read_line(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim_end().to_string()
}

fn run_calculator() {
    println!("------ Hesap Makinesi ------");
    let a_str = read_line("Birinci sayıyı girin: ");
    let b_str = read_line("İkinci sayıyı girin: ");
    let op = read_line("İşlem türü (+, -, *, /): ");

    let a: f64 = match a_str.trim().parse() {
        Ok(v) => v,
        Err(_) => {
            println!("Birinci sayı geçerli değil.");
            return;
        }
    };

    let b: f64 = match b_str.trim().parse() {
        Ok(v) => v,
        Err(_) => {
            println!("İkinci sayı geçerli değil.");
            return;
        }
    };

    let result = match op.trim() {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => {
            if b == 0.0 {
                println!("Sıfıra bölme yapılamaz.");
                return;
            }
            a / b
        }
        other => {
            println!("Bilinmeyen işlem türü: {other}");
            return;
        }
    };

    println!("Sonuç: {result}");
}
