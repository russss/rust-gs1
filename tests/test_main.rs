use epc::{decode_binary, decode_binary_box};
use hex;


#[test]
fn test_decode() {
    let data = [48, 57, 96, 98, 195, 161, 168, 0, 0, 107, 51, 244];
    let result = decode_binary_box(&data).unwrap();
    println!("{:?}", result.to_uri());

    let data = [0, 176, 122, 20, 12, 95, 156, 81, 64, 0, 3, 238];
    println!("{:?}", decode_binary(&data));

    let data = [226, 0, 0, 25, 6, 12, 2, 9, 6, 144, 211, 194];
    println!("{:?}", decode_binary(&data));
}

// Examples from GS1 EPC E.3
#[test]
fn test_examples() {
    // SGTIN-96
    let data = decode_binary_box(&hex::decode("3074257BF7194E4000001A85").unwrap()).unwrap();
    assert_eq!(data.to_uri(), "urn:epc:id:sgtin:0614141.812345.6789");
    assert_eq!(data.to_tag_uri(), "urn:epc:tag:sgtin-96:3.0614141.812345.6789");


    // SGTIN-198
    /*
    let data = decode_binary_box(&hex::decode("3674257BF6B7A659B2C2BF100000000000000000000000000000").unwrap()).unwrap();
    assert_eq!(data.to_uri(), "urn:epc:id:sgtin:0614141.712345.32a%2Fb");
    assert_eq!(data.to_tag_uri(), "urn:epc:tag:sgtin-198:3.0614141.712345.32a%2Fb");
    */
}
