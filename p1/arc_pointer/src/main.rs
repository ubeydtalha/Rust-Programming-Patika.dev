use std::sync::{Arc, Mutex};
use std::thread;


fn main() {
    // Arc ile paylaşılan bir Mutex içerisinde bir i32 değeri oluşturulur
    let sayi = Arc::new(Mutex::new(0));

    // İş parçacıklarını saklamak için bir vektör oluşturulur
    let mut handles: Vec<thread::JoinHandle<()>> = vec![];
    
    for _ in 0..10{

        // Arc klonlanır, böylece her iş parçacığı sayıya erişebilir
        let sayi = Arc::clone(&sayi);

        // Yeni bir iş parçacığı oluşturulur
        let handle = thread::spawn(
            move  || {
                // Mutex kilidi alınır ve içerisindeki veriye erişilir
                let mut num: std::sync::MutexGuard<'_, i32> = sayi.lock().unwrap();
            
                // Değer arttırılır
                *num += 1;
            }
        );

        // İş parçacığı iş parçacığı vektörüne eklenir
        handles.push(handle);

    }

    // Tüm iş parçacıklarının bitmesi beklenir  
    for handle in handles {
        handle.join().unwrap();
    }

    // Mutex kilidi alınır ve içerisindeki veriye erişilir
    let num: std::sync::MutexGuard<'_, i32> = sayi.lock().unwrap();
    println!("Son değer: {}", *num);
    


}