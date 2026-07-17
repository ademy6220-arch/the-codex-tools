struct CryptographicKey {
    key_data: Vec<u8>,
}

impl CryptographicKey {
    fn new(data: Vec<u8>) -> Self {
        Self { key_data: data }
    }
}

// Der Destruktor (Drop) wird automatisch aufgerufen, wenn die Variable out of scope geht
impl Drop for CryptographicKey {
    fn drop(&mut self) {
        // Überschreibe alle Bytes physisch mit 0
        for byte in self.key_data.iter_mut() {
            *byte = 0;
        }
        println!("[Sicherheit] Sensibler Schlüssel im Arbeitsspeicher wurde genullt.");
    }
}

fn main() {
    {
        let _secure_key = CryptographicKey::new(vec![0xAA, 0xBB, 0xCC, 0xDD]);
        println!("[*] Schlüssel wird für Entschlüsselung verwendet...");
    } // <-- _secure_key verlässt hier den Scope. Drop() wird automatisch ausgeführt.
    
    println!("[*] Programm wird fortgesetzt.");
}
