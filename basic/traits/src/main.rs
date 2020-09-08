// Eq を実装するための PartialEq
#[derive(Eq, PartialEq)]
struct A(i32);

// PartialEqを実装するための PartialOrd
#[derive(PartialEq, PartialOrd)]
struct B(f32);

// Copyを実装するための Clone
#[derive(Copy, Clone)]
struct C;

#[derive(Clone)]
struct D;

#[derive(Debug)]
struct E;

#[derive(Default)]
struct F;


fn main() {
    // Aは一致比較可能
    println!("{:?}", A(0) == A(1));

    // Bは大小比較可能
    println!("{:?}", B(1.0) > B(0.0));

    // Cはムーブではなくコピー
    let c0 = C;
    let _c1 = c0;
    let _c2 = c0;//Cがムーブならここでコンパイルエラー

    // Dはclone
    let d0 = D;
    let _d1 = d0.clone();

    // Eはデバックプリント
    println!("{:?}",E);

    // Fはデフォルト可能
    let _f = F::default();

}
