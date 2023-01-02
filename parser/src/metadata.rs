#![allow(non_snake_case, dead_code)]

use serde_derive::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
use std::fs;

/// Error types while using FileMeta DataStructure
pub enum FileMetaError {
    /// Error thrown when there is some issue while reading the file path provided
    FileError(String),
    /// Error thrown when deserializing the ".torrent" bencode encoded data into FileMeta struct
    DataError(String),
}

/// DataStructure that maps all the data inside of bencode encoded ".torrent" file
/// into something rust program can use.
#[derive(Debug, Deserialize)]
pub struct FileMeta {
    /// **(Required)** It's a URL that specifies the location of the tracker, which is a server that helps coordinate communication between the clients that are downloading and uploading the file.
    pub announce: String,

    /// **(Optional)** It's a list of backup trackers in case the primary tracker is unavailable. Each tracker in the list is specified by a URL, it contains the url of "announce" field as well, so if this field is present, then
    /// we can surely omit value in the "announce" field
    #[serde(rename = "announce-list")]
    pub announce_list: Option<Vec<Vec<String>>>,

    /// **(Required)** It's a  dictionary that contains metadata about the file or group of files.
    pub info: Info,

    /// **(Optional)** UNIX timestamp that indicates when the file was created
    #[serde(rename = "creation date")]
    pub creation_data: Option<i64>,

    /// **(Optional)** A comment about the torrent
    pub comment: Option<String>,

    /// **(Optional)** String that indicates the character encoding that was used to encode the name of the file
    pub encoding: Option<String>,

    /// **(Optional)** String indicating the name and version of the software that was used to create the torrent file
    #[serde(rename = "created by")]
    pub created_by: Option<String>,
}

/// The fields within the Info DataStructure are used to build "info hash", so it must the required
/// fields and its data must not be missed
#[derive(Debug, Deserialize, Serialize)]
pub struct Info {
    pub name: Option<String>,
    pub length: Option<i64>,
    pub files: Option<Vec<File>>,
    #[serde(rename = "piece length")]
    pub piece_length: Option<i64>,
    /// Consists of byte string of concatenation of all 20-byte SHA1 hash values, one per piece
    #[serde(with = "serde_bytes")]
    pub pieces: Vec<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct File {
    pub length: i64,
    pub path: Vec<String>,
    pub md5sum: Option<String>,
}

impl FileMeta {
    /// Just pass in your path to the torrent file, it will try to return a
    /// DataStructure[FileMeta] that contains all the metadata that was within the ".torrent" file
    /// Eg.
    /// ```
    /// let meta : FileMeta = FileMeta::fromTorrenFile("x/y/z/zz.torrent")
    ///
    /// ```
    ///
    pub fn fromTorrenFile(&self, file_path: &String) -> Result<FileMeta, FileMetaError> {
        // Creates a buffer to store the bytes of the file
        match fs::read(file_path) {
            Ok(bytes) => match self.fromRawTorrentFile(bytes) {
                Ok(meta) => Ok(meta),
                Err(err) => Err(err),
            },
            Err(err) => Err(FileMetaError::DataError(err.to_string())),
        }
    }

    /// Passing the bytes of the ".torrent" file will try to generate a DataStructure[FileMeta] from the given bencode encoded data
    /// Eg.
    /// ```
    /// let torrentFile: Vec<u8> = fs::read("x/y/z/zz.torrent").unwrap();
    /// let meta: FileMeta = FileMeta::fromRawTorrenFile(torrentFile);
    ///
    /// ```
    ///
    pub fn fromRawTorrentFile(&self, file: Vec<u8>) -> Result<FileMeta, FileMetaError> {
        match serde_bencode::de::from_bytes::<FileMeta>(&file) {
            Ok(d) => Ok(d),
            Err(err) => Err(FileMetaError::DataError(err.to_string())),
        }
    }

    // TODO : Add a parsing method for "magnet links"
    // / TODO : Find a way to parse magnet link
    //pub fn parseMagnetLink(magnet_link: &String) -> () {
    //    // TODO : MAKE IT USABLE | AS OF RIGHT NOW IT"S NOT USABLE
    //    match MagnetURI::from_str(&magnet_link.as_ref()) {
    //        Ok(data) => {}
    //        Err(_) => {
    //            // Throws you some kind of error when the magnet link isn't valid
    //            println!("Enter a valid magnet link!");
    //        }
    //    }
    //}
    /// InfoHash is the SHA1 hash of all the fields within the "info" field of bencode encoded
    /// torrent file
    /// Generates and gives you the info hash of the
    /// Eg.
    ///
    /// ```
    ///   
    /// let meta : FileMeta = FileMeta::fromTorrenFile("x/y/z/zz.torrent");
    /// let info_hash : Vec<u8> = meta.generateInfoHash();
    ///
    /// ```
    /// Gets you the Info Hash
    pub fn generateInfoHash(&self) -> Vec<u8> {
        // Serialize the info section of FileMeta and get all bytes in info field of a torrent file
        // i.e Converts the data of info field to "bytes", to generate Info Hash
        let info_byte = serde_bencode::ser::to_bytes(&self.info).unwrap();
        let mut hasher = Sha1::new();
        hasher.update(info_byte);
        hasher.finalize().into_iter().collect()
    }
}
