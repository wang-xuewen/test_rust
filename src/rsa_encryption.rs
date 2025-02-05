use log::info;
use log4rs;
use rand::rngs::OsRng;
use rsa::pkcs1::DecodeRsaPrivateKey;
use rsa::pkcs1::DecodeRsaPublicKey;
use rsa::pkcs1::EncodeRsaPrivateKey; // 用于导出 PKCS#1 PEM 格式
use rsa::pkcs1::EncodeRsaPublicKey;
use rsa::{PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey}; // 用于从 PEM 解析私钥
pub fn rsa_sample() {
    // -------------------------
    // 创建一个随机数生成器
    let mut rng = OsRng;

    // info!("生成 RSA 密钥对（2048 位）");
    // let private_key = RsaPrivateKey::new(&mut rng, 2048).expect("Failed to generate private key");
    // info!("将私钥编码为 PEM 格式字符串");
    // let private_pem = private_key
    //     .to_pkcs1_pem(rsa::pkcs8::LineEnding::LF)
    //     .expect("Failed to encode private key");
    // // let private_pem: Zeroizing<String> = Zeroizing::new("Secret Data".to_string());
    // // println!("{}", private_pem.as_str());
    // info!("Private Key in PEM Format:\n{}", private_pem.as_str());

    let private_pem_str = r#"-----BEGIN RSA PRIVATE KEY-----
MIIEpQIBAAKCAQEA4gRE9HOERcEUhKSNgqYtnVW9LIy+b5qM+jTEDoi956DhTytI
Ax+phOOrC/cI68+XXnPFZsNHy7ZxC2nONEzNYuS7ev9qxAxhhoRYLXDhsuqVsPg8
KuJAJRSDAhpdcw39vij4nccnlAjjFye73Qo7Mb7Gd7YAFBtX81I/u58QZrTqHVwk
pqZNWB+2YK//XV35PcOquKE4K9qyQfAydMKEmDjj6Q1yj6XoWJlaMspG/WRCUMM6
G52JP0Ln2gG3F5wFIy3jbYDq7AOVHkzX+ZHmlbCzceOVWBsBtfy8sjxOfFcdVGHO
KgQEP6fuBV/ohqqKDCwAGoA2RzIdkjtY6msWlwIDAQABAoIBAQCTDjYTW0nX3N3N
xAcy147cTNnuL5EEqmlJrqVV05DEMcAHM+EB70rr5Pn4qf4K+CHD2c25pBuFpyn8
RBZfnw0PaEW3rJI6Btg5nsiEoGqpb8p/v7PeLyGPPr5S0va0kojjfmLsvZS50+J1
wWuRUUPchQU7BJcefgAGmCBRohgYyipAAMiTH2U8z00LyFI9QU5xGybP+GOoPn59
P7QnEa4/Tn/+6bY2jcanI3JzznZL1gvvxx+lSurZIapl+P91TXiXQ/AJEJ7TPTDc
Riv83iBevzlxKyTSo8oGjLKK6Ml/Sh0Zxasf+812G+E9FFIBkkhInQJX0FwMlvHZ
YoSiWM1JAoGBAOegHN2Rcp3xP/S4KR5Pd7Knjv3pZ+4awF3/ffUGdIzK3dHGKQy3
YvT8By/QrhjeArpWrmmTlWup7FnSFzFfUzyxWjgXSIQDTIRijvDq0ZtlKU8UHYPN
7ePQl412QPP+LtCt6+Yd8AuvHCnfyisSYy35anwtr7AkelEtz8R7hqFbAoGBAPnN
DxnphprZqnaZxI3YxrqDlcKKtA81qG4i/HVtjJJdf3sedQ9imlHoMLq5/1mnKaB8
E7/YR1Ib0OAN73LfapDh/sFhzaiPM37g+2VflUU5BToDtDnQMt0/RV7t6jd8O4tu
QZLVgXApwY507mmyz4W+taiQ7+M9bAxXO+3VcYh1AoGBAI7WZ1af3l3WK4mflAPU
H821lPGyYVwtdRnCeAuFWpSEejxmBmSIJudLEKeE+gftySLeV5pV39xQIqfVbmYN
Egiomili+l4mpqYxHVMmi/JXdR0GG5lvgdduiDc9iJqu0nHv/zyek6yw5R5Rmpvr
L+xnFirUBbcLF78+EBVr079nAoGAD86E/RvE07mgSr7yLBOih5zZ9iR2vluj28xE
811KPtzBu1WzDJUttK8fnkE0wkSMosYXLdWOtchi0DqxgzBV+vMB/tSkgd0F4ip0
XfbNaELybLhdSCc/gLaHOjmNz5MB5ZHFfngaJ7HMuKn3iCKzdQAbWJ5LP7LcSm+e
sC8Ibx0CgYEAg2AGQd2FFvekl4LU+vho5nmJ+ieDeWMzEW9kY5Gv3UfvSkJCXgNL
seTQ1kWIIiQE6Yc9xT/FSs3YWC9YuUK5DMog0bH+xnqFxc1vVqMtR+8Khf5BhkVC
eY7i0K6c9dKEiAWBsvd3C8/ktcXSps8wjxGVH+X/2Re316biQfk6QV8=
-----END RSA PRIVATE KEY-----"#;
    // **从 PEM 字符串解析私钥**
    let private_key =
        RsaPrivateKey::from_pkcs1_pem(private_pem_str).expect("Failed to parse private key");
    println!("Private key successfully loaded!");

    // info!("start generating public key");
    // let public_key = RsaPublicKey::from(&private_key);
    // let public_pem = public_key
    //     .to_pkcs1_pem(rsa::pkcs8::LineEnding::LF)
    //     .expect("Failed to encode public key");
    // info!("Public Key PEM:\n{}", public_pem.as_str());

    let public_pem_str = r#"-----BEGIN RSA PUBLIC KEY-----
MIIBCgKCAQEA4gRE9HOERcEUhKSNgqYtnVW9LIy+b5qM+jTEDoi956DhTytIAx+p
hOOrC/cI68+XXnPFZsNHy7ZxC2nONEzNYuS7ev9qxAxhhoRYLXDhsuqVsPg8KuJA
JRSDAhpdcw39vij4nccnlAjjFye73Qo7Mb7Gd7YAFBtX81I/u58QZrTqHVwkpqZN
WB+2YK//XV35PcOquKE4K9qyQfAydMKEmDjj6Q1yj6XoWJlaMspG/WRCUMM6G52J
P0Ln2gG3F5wFIy3jbYDq7AOVHkzX+ZHmlbCzceOVWBsBtfy8sjxOfFcdVGHOKgQE
P6fuBV/ohqqKDCwAGoA2RzIdkjtY6msWlwIDAQAB
-----END RSA PUBLIC KEY-----"#;

    let public_key =
        RsaPublicKey::from_pkcs1_pem(public_pem_str).expect("Failed to parse public key");
    println!("Public key successfully loaded!");

    // 要加密的字符串
    let data = "Hello, RSA encryption!";
    info!("Original data: {}", data);

    // 加密
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let encrypted_data = public_key
        .encrypt(&mut rng, padding, data.as_bytes())
        .expect("Failed to encrypt data");
    info!("Encrypted data length: {}", encrypted_data.len());

    // 解密
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let decrypted_data = private_key
        .decrypt(padding, &encrypted_data)
        .expect("Failed to decrypt data");
    let decrypted_string = String::from_utf8(decrypted_data).expect("Failed to convert to string");
    info!("Decrypted data: {}", decrypted_string);
}
