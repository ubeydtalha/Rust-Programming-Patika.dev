use std::cell::RefCell;

fn main() {
    let sayi = RefCell::new(5);

    {
        // `sayi`'ye immutable bir referans alınır
        let sayi_borrow = sayi.borrow();
        println!("sayi: {}", *sayi_borrow);

        // Bu blokta `sayi`'ye başka bir immutable referans alınabilir,
        // ancak mutable bir referans alınamaz.
    } // `sayi_borrow` burada kapsam dışına çıkar

    {
        // `sayi`'ye mutable bir referans alınır
        let mut sayi_borrow_mut = sayi.borrow_mut();
        *sayi_borrow_mut += 1;

        // Bu blokta `sayi`'ye başka bir referans alınamaz.
    } // `sayi_borrow_mut` burada kapsam dışına çıkar

    // Değişikliklerin uygulandığını doğrulayalım
    println!("sayi: {}", *sayi.borrow());
}
