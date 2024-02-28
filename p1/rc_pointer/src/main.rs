use std::rc::Rc;

struct Pie;

impl Pie {
    fn eat(&self) {
        println!("tastes better on the heap!")
    }
}

fn main() {
    let heap_pie = Rc::new(Pie);
    let heap_pie2 = heap_pie.clone();
    let heap_pie3 = heap_pie2.clone();
    
    heap_pie.eat();
    heap_pie3.eat();
    heap_pie2.eat();
    
    // all reference count smart pointers are dropped now
    // the heap data Pie finally deallocates
    // Rc akıllı işaretçisi ile bir i32 değeri heap üzerine taşınır
    let sayi = Rc::new(5);
    println!("İlk sayı referans sayısı: {}", Rc::strong_count(&sayi));

    {
        // `sayi` klonlanır, bu işlem verinin kendisini değil,
        // heap üzerindeki veriye olan referansları klonlar
        let sayi_klonu = Rc::clone(&sayi);
        println!("Klonlama sonrası sayı referans sayısı: {}", Rc::strong_count(&sayi));

        // `sayi_klonu` burada kullanılabilir
        // Bu blok sonunda `sayi_klonu` düşer, fakat orijinal veri heap üzerinde kalır
    }

    // `sayi_klonu` kapsam dışına çıktı, referans sayımı azalır
    println!("Son sayı referans sayısı: {}", Rc::strong_count(&sayi));

    // Bu noktada, `sayi` hala kullanılabilir
    // `main` fonksiyonunun sonunda `sayi` düşer ve heap üzerindeki veri deallocate edilir



}
