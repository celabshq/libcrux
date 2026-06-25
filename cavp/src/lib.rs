use core::fmt;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/// Read a test vector file.
pub fn read_file<Ty: Tv>(file: &str) -> Result<TestVector<Ty>, Error> {
    let reader = BufReader::new(File::open(file).map_err(|_| Error::FileOpen)?);
    TestVector::read(reader)
}

/// Read a test vector from a string.
pub fn read_string<Ty: Tv>(tv: &str) -> Result<TestVector<Ty>, Error> {
    let reader = BufReader::new(tv.as_bytes());
    TestVector::read(reader)
}

/// Errors
#[derive(Debug, Clone, Copy)]
pub enum Error {
    FileOpen,
    Read,
}

/// A CAVP test vector.
#[derive(Debug, Clone, Default)]
pub struct TestVector<TestType: Tv> {
    pub header: TestType::H,
    pub tests: Vec<TestType::T>,
}

impl<T: Tv> TestVector<T> {
    /// Forward the read to the underlying type `T`.
    fn read(mut reader: impl BufRead) -> Result<Self, Error> {
        let mut tv = Self::default();
        let mut test = T::T::default();
        let mut header = T::H::default();

        // Read a single entry
        loop {
            log::debug!("reading next line ...");
            let mut line = String::new();
            let bytes_read = reader.read_line(&mut line).map_err(|_| Error::Read)?;
            if bytes_read == 0 {
                return Ok(tv);
            }

            let mut line = line.trim();

            // Filter out empty lines and comments
            if line.len() == 0 || line.starts_with("#") {
                continue;
            }
            log::debug!("reading {}", line);

            // Read a section header
            //
            // A section header has the form `[ ... ]`
            if line.starts_with("[") && line.ends_with("]") {
                line = line.strip_suffix("]").ok_or(Error::Read)?;
                line = line.strip_prefix("[").ok_or(Error::Read)?;
                log::debug!("line {line}");
                T::H::read_header::<T>(line, &mut header)?;

                // Continue with the next line
                continue;
            }

            // We're done with the header, set it.
            tv.header = header.clone();
            // log::debug!("header: {:?}", header);

            // Set parameters
            // test.out_len = out_len;

            // Read a test case
            //
            // A test case has the form
            // ```
            // ... = ...
            //
            // ```
            T::T::read_test(line, &mut test, &mut tv)?;
        }
    }
}

// This module only helps hiding the test trait.
mod helper {
    use super::*;

    /// The trait to use globally
    pub trait Tv: Default {
        type T: Test + Clone;
        type H: Header + Clone;
    }

    /// Helper trait for reading tests
    pub trait Test: Default + Clone {
        /// Read test vector specific test values
        fn read_test<Ty: Tv<T = Self>>(
            line: &str,
            test: &mut Self,
            tv: &mut TestVector<Ty>,
        ) -> Result<(), Error>
        where
            Self: Sized;
    }

    /// Heper trait for reading headers
    pub trait Header: Default + Clone {
        /// Read test vector specific header values
        fn read_header<Ty: Tv<H = Self>>(line: &str, header: &mut Self) -> Result<(), Error>
        where
            Self: Sized;
    }
}
use helper::*;

/// A SHA3 test.
#[derive(Debug, Clone, Default)]
pub struct Sha3Test {
    /// Note that `msg_length` is not necessary the same as `msg.len()`
    pub msg_length: usize,
    pub msg: Vec<u8>,
    pub digest: Vec<u8>,
}

/// Empty SHA3 header.
#[derive(Debug, Clone, Default)]
pub struct Sha3Header {}

#[derive(Debug, Clone, Default)]
pub struct Sha3 {}
impl Tv for Sha3 {
    type T = Sha3Test;
    type H = Sha3Header;
}

impl Header for Sha3Header {
    fn read_header<Ty: Tv<H = Self>>(_line: &str, _test: &mut Self) -> Result<(), Error>
    where
        Self: Sized,
    {
        // We don't read anything here for now.
        Ok(())
    }
}

impl Test for Sha3Test {
    fn read_test<Ty: Tv<T = Self>>(
        line: &str,
        test: &mut Self,
        tv: &mut TestVector<Ty>,
    ) -> Result<(), Error> {
        let mut key_value = line.split("=");
        let key = key_value.next().map(|v| v.trim());
        log::debug!("key: {key:?}");

        match key {
            Some("Len") => {
                test.msg_length = to_usize(key_value)?;
            }
            Some("Msg") => {
                test.msg = to_bytes(key_value)?;
            }
            Some("MD") => {
                test.digest = to_bytes(key_value)?;
                tv.tests.push(test.clone());
            }
            _ => return Err(Error::Read),
        }

        Ok(())
    }
}

/// A Shake msg test.
#[derive(Debug, Clone, Default)]
pub struct ShakeMsgTest {
    /// Note that `msg_length` is not necessary the same as `msg.len()`
    pub msg_length: usize,
    pub msg: Vec<u8>,
    pub digest: Vec<u8>,
}

/// Empty Shake msg header
#[derive(Debug, Clone, Default)]
pub struct ShakeMsgHeader {}

/// A Shake msg test.
#[derive(Debug, Clone, Default)]
pub struct ShakeMsg {}
impl Tv for ShakeMsg {
    type T = ShakeMsgTest;
    type H = ShakeMsgHeader;
}

impl Header for ShakeMsgHeader {
    fn read_header<Ty: Tv<H = Self>>(_line: &str, _test: &mut Self) -> Result<(), Error>
    where
        Self: Sized,
    {
        // We don't read anything here for now.
        Ok(())
    }
}

impl Test for ShakeMsgTest {
    fn read_test<Ty: Tv<T = Self>>(
        line: &str,
        test: &mut Self,
        tv: &mut TestVector<Ty>,
    ) -> Result<(), Error>
    where
        Self: Sized,
    {
        let mut key_value = line.split("=");
        let key = key_value.next().map(|v| v.trim());
        log::debug!("key: {key:?}");

        match key {
            Some("Len") => {
                test.msg_length = to_usize(key_value)?;
            }
            Some("Msg") => {
                test.msg = to_bytes(key_value)?;
            }
            Some("Output") => {
                test.digest = to_bytes(key_value)?;
                tv.tests.push(test.clone());
            }
            _ => return Err(Error::Read),
        }

        Ok(())
    }
}

/// A Shake test with variable output length.
#[derive(Debug, Clone, Default)]
pub struct ShakeVariableOutTest {
    /// We ignore the COUNT for now
    // pub count: usize,

    /// Output length
    pub out_len: usize,

    /// Note that `msg_length` is not necessary the same as `msg.len()`
    pub msg_length: usize,

    /// The input message
    pub msg: Vec<u8>,

    /// The expected output
    pub digest: Vec<u8>,
}

impl fmt::Display for ShakeVariableOutTest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "out_len: {}\n", self.out_len)?;
        write!(f, "msg_length: {}\n", self.msg_length)?;
        write!(f, "msg: {}\n", hex::encode(&self.msg))?;
        write!(f, "digest: {}\n", hex::encode(&self.digest))
    }
}

/// Shake variable out header
#[derive(Debug, Clone, Default)]
pub struct ShakeVariableOutHeader {
    /// The length of the input message.
    pub input_length: usize,
}

/// A Shake msg test.
#[derive(Debug, Clone, Default)]
pub struct ShakeVariableOut {}
impl Tv for ShakeVariableOut {
    type T = ShakeVariableOutTest;
    type H = ShakeVariableOutHeader;
}

impl Header for ShakeVariableOutHeader {
    fn read_header<Ty: Tv<H = Self>>(line: &str, header: &mut Self) -> Result<(), Error>
    where
        Self: Sized,
    {
        let mut key_value = line.split("=");
        let key = key_value.next().map(|v| v.trim());
        log::debug!("key: {key:?}");

        match key {
            Some("Input Length") => {
                header.input_length = to_usize(key_value)?;
            }
            // We ignore everything else for now
            _ => return Ok(()),
        }

        Ok(())
    }
}

impl Test for ShakeVariableOutTest {
    fn read_test<Ty: Tv<T = Self>>(
        line: &str,
        test: &mut Self,
        tv: &mut TestVector<Ty>,
    ) -> Result<(), Error>
    where
        Self: Sized,
    {
        let mut key_value = line.split("=");
        let key = key_value.next().map(|v| v.trim());
        log::debug!("key: {key:?}");

        match key {
            Some("Outputlen") => {
                test.msg_length = to_usize(key_value)?;
            }
            Some("Msg") => {
                test.msg = to_bytes(key_value)?;
            }
            Some("Output") => {
                test.digest = to_bytes(key_value)?;
                tv.tests.push(test.clone());
            }
            // We ignore these for now
            Some("COUNT") => log::debug!("Ignoring COUNT"),
            _ => return Err(Error::Read),
        }
        log::debug!("done reading {key:?}");

        Ok(())
    }
}

// Currently unused utilities.
#[allow(unused)]
mod utils {
    use super::*;

    trait FromStr {
        fn from_str(s: &str) -> Result<Self, Error>
        where
            Self: Sized;
    }

    impl FromStr for usize {
        fn from_str(s: &str) -> Result<Self, Error> {
            s.parse().map_err(|_| Error::Read)
        }
    }

    impl FromStr for Vec<u8> {
        fn from_str(s: &str) -> Result<Self, Error> {
            hex::decode(s).map_err(|_| Error::Read)
        }
    }

    /// A key-value pair
    ///
    /// This is represented as `key = value` in the rsp file with `value: T`
    struct KeyValue<T: FromStr> {
        key: String,
        value: T,
    }

    impl<T: FromStr> KeyValue<T> {
        fn new(key: String, value: T) -> Self {
            Self { key, value }
        }
    }

    /// Read key value pair
    fn read_kv_string<T: FromStr>(kv: &str) -> Result<KeyValue<T>, Error> {
        let mut key_value = kv.split("=");
        let key = key_value.next().map(|v| v.trim()).ok_or(Error::Read)?;
        log::debug!("key: {key:?}");
        let value = key_value.next().map(|v| v.trim()).ok_or(Error::Read)?;
        log::debug!("value: {value:?}");
        Ok(KeyValue::new(key.to_string(), T::from_str(value)?))
    }

    /// Read key value pair with type
    fn read_kv_type<T: FromStr>(kv: &str, key: &str) -> Result<KeyValue<T>, Error> {
        let kv = read_kv_string(kv)?;
        if kv.key != key {
            log::error!("Expected key {key} but got {}", kv.key);
            return Err(Error::Read);
        }
        Ok(kv)
    }
}

/// Helper to convert strings to bytes
fn to_bytes(mut value: std::str::Split<&str>) -> Result<Vec<u8>, Error> {
    let next = value.next().map(|v| v.trim());
    log::debug!("value: {next:?}");
    Ok(hex::decode(next.ok_or(Error::Read)?).map_err(|_| Error::Read)?)
}

/// Helper to convert strings to usize
fn to_usize(mut value: std::str::Split<&str>) -> Result<usize, Error> {
    let next = value.next().map(|v| v.trim());
    log::debug!("value: {next:?}");
    Ok(next.map(|v| v.parse().ok()).flatten().ok_or(Error::Read)?)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn sha3() {
        let _ = pretty_env_logger::try_init();
        let tv = r"
# Sample test vector for SHA3 256

[L = 256]

Len = 0
Msg = 00
MD = a7ffc6f8bf1ed76651c14756a061d662f580ff4de43b49fa82d80a4b80f8434a

Len = 8
Msg = e9
MD = f0d04dd1e6cfc29a4460d521796852f25d9ef8d28b44ee91ff5b759d72c1e6d6

Len = 16
Msg = d477
MD = 94279e8f5ccdf6e17f292b59698ab4e614dfe696a46c46da78305fc6a3146ab7
";
        let tv: TestVector<Sha3> = super::read_string(tv).unwrap();
        eprintln!("tv: {tv:x?}");
    }

    #[test]
    fn shake128() {
        let _ = pretty_env_logger::try_init();
        let tv = r"
# Sample test vector for Shake128
#  Generated on Thu Jan 28 14:46:45 2016

[Outputlen = 128]

Len = 0
Msg = 00
Output = 7f9c2ba4e88f827d616045507605853e

Len = 8
Msg = 0e
Output = fa996dafaa208d72287c23bc4ed4bfd5
";
        let tv: TestVector<ShakeMsg> = super::read_string(tv).unwrap();
        eprintln!("tv: {tv:x?}");
    }

    #[test]
    fn shake128_variable_out() {
        let _ = pretty_env_logger::try_init();
        let tv = r"
# Sample test vector for Shake128
#  Generated on Thu Jan 28 14:46:47 2016

[Tested for Output of byte-oriented messages]
[Input Length = 128]
[Minimum Output Length (bits) = 125]
[Maximum Output Length (bits) = 1120]
COUNT = 0
Outputlen = 128
Msg = 84e950051876050dc851fbd99e6247b8
Output = 8599bd89f63a848c49ca593ec37a12c6

COUNT = 1
Outputlen = 128
Msg = 9a335790abf769877c9e6cd3d5199e8c
Output = 2ece1768a6ef6568a2dff699613f49d0
";
        let tv: TestVector<ShakeVariableOut> = super::read_string(tv).unwrap();
        eprintln!("tv: {tv:?}");
    }
}
