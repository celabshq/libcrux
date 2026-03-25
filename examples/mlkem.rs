use std::{
    fs::File,
    io::{Read, Write},
};

use clap::{Parser, Subcommand};
use libcrux::primitives::kem;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

#[derive(Subcommand)]
enum GenerateCli {
    GenerateKey {
        /// Optionally, a file name to store the keys into.
        ///
        /// The keys will be store into `$out.pub` and `$out.priv` when this is used.
        #[arg(short, long)]
        out: Option<String>,
    },
    Encaps {
        /// Public key input file to encrypt to.
        #[arg(short, long)]
        key: String,

        /// Optionally, a file name to store the encapsulation output.
        ///
        /// This defaults to `mlkem.ct`.
        #[arg(short, long)]
        ct: Option<String>,

        /// Optionally, a file name to store the shared secret output.
        ///
        /// This defaults to `mlkem-encapsulated.ss`.
        #[arg(short, long)]
        ss: Option<String>,
    },
    Decaps {
        /// Private key input file to decapsulate with.
        #[arg(short, long)]
        key: String,

        /// Encapsulated secret.
        #[arg(short, long)]
        ct: String,

        /// Optionally, a file name to store the shared secret.
        ///
        /// This defaults to `mlkem-decapsulated.ss``.
        #[arg(short, long)]
        ss: Option<String>,
    },
}

#[derive(Parser)]
struct Cli {
    /// Generate a key pair and write it out.
    ///
    /// When no output files are given, `mlkem.pub` and `mlkem` are used.
    #[command(subcommand)]
    cmd: GenerateCli,

    /// The key length, 512, 768, or 1024
    ///
    /// Defaults to 768
    #[arg(short, long)]
    algorithm: Option<u16>,
}

fn main() {
    pretty_env_logger::init();

    let cli = Cli::parse();

    let alg = if let Some(l) = cli.algorithm {
        match l {
            512 => kem::Algorithm::MlKem512,
            768 => kem::Algorithm::MlKem768,
            1024 => kem::Algorithm::MlKem1024,
            _ => {
                eprintln!("Invalid algorithm variant {l}");
                return;
            }
        }
    } else {
        kem::Algorithm::MlKem768
    };

    let mut rng = ChaCha20Rng::from_os_rng();

    match cli.cmd {
        GenerateCli::GenerateKey { out: file } => {
            // Generate a key pair and write it out.
            let (sk_name, pk_name) = match file {
                Some(n) => (format!("{n}.priv"), format!("{n}.pub")),
                None => ("mlkem.priv".to_owned(), "mlkem.pub".to_owned()),
            };

            let (secret_key, public_key) = kem::key_gen(alg, &mut rng).unwrap();

            println!("Writing private key to {sk_name}");
            File::create(sk_name.clone())
                .expect(&format!("Can not create file {sk_name}"))
                .write_all(&secret_key.encode())
                .expect("Error writing private key");
            println!("Writing public key to {pk_name}");
            File::create(pk_name.clone())
                .expect(&format!("Can not create file {pk_name}"))
                .write_all(&public_key.encode())
                .expect("Error writing public key");
        }

        GenerateCli::Encaps { key, ct: out, ss } => {
            let pk = bytes_from_file(key);
            let pk = kem::PublicKey::decode(alg, &pk).expect("Error decoding public key");

            let (shared_secret, ciphertext) =
                pk.encapsulate(&mut rng).expect("Error encapsulating");

            let ct_out = match out {
                Some(n) => n,
                None => "mlkem.ct".to_owned(),
            };
            let ss_out = match ss {
                Some(n) => n,
                None => "mlkem-encapsulated.ss".to_owned(),
            };

            println!("Writing ciphertext to {ct_out}");
            let mut out_file =
                File::create(ct_out.clone()).expect(&format!("Can not create file {ct_out}"));
            out_file
                .write_all(&ciphertext.encode())
                .expect("Error writing public key");

            println!("Writing shared secret to {ss_out}");
            let mut out_file =
                File::create(ss_out.clone()).expect(&format!("Can not create file {ss_out}"));
            out_file
                .write_all(&shared_secret.encode())
                .expect("Error writing public key");
        }
        GenerateCli::Decaps { key, ss: out, ct } => {
            let sk = bytes_from_file(key);
            let sk = kem::PrivateKey::decode(alg, &sk).expect("Error decoding private key");

            let ct = bytes_from_file(ct);

            let ct = kem::Ct::decode(alg, &ct).expect("Error decoding ct.");
            let shared_secret = ct.decapsulate(&sk).expect("Error decapsulating.");

            let out = match out {
                Some(n) => n,
                None => "mlkem-decapsulated.ss".to_owned(),
            };

            println!("Writing shared secret to {out}");
            let mut out_file =
                File::create(out.clone()).expect(&format!("Can not create file {out}"));
            out_file
                .write_all(&shared_secret.encode())
                .expect("Error writing public key");
        }
    }
}

fn bytes_from_file(file: String) -> Vec<u8> {
    println!("Reading file {file}");
    let mut bytes = Vec::new();
    File::open(file.clone())
        .expect(&format!("Error opening file {file}"))
        .read_to_end(&mut bytes)
        .expect(&format!("Error reading file {file}."));
    bytes
}
