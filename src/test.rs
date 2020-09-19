#[allow(unused_imports)]
use crate::*;

#[test]
fn test_run() {
    let (train_data, train_labels, test_data, test_labels) = Cifar10::default()
        .show_images(false)
        .build()
        .expect("Failed to build CIFAR-10 data");
}

#[test]
fn test_flat_f32() {

    let (train_data, train_labels, test_data, test_labels) = Cifar10::default()
        .show_images(false)
        .build_as_flat_f32()
        .expect("Failed to build CIFAR-10 data");
        
}
