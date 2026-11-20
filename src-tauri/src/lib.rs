use bzip2::read::BzDecoder;
use flate2::read::GzDecoder;
use serde::Serialize;
use std::{fs::{self, File}, io::{self, Read, Write}, path::{Component, Path, PathBuf}};
use xz2::read::XzDecoder;

#[derive(Debug, Serialize)]
struct Inspection { file_name:String, path:String, size:u64, format:String, supported:bool, description:String }
#[derive(Debug, Serialize)]
struct ExtractionResult { files_written:u64, bytes_written:u64, destination:String }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Format { Zip, Tar, Gzip, Bzip2, Xz, SevenZip, Rar, Iso, Unknown }
impl Format {
 fn label(self)->&'static str { match self { Self::Zip=>"ZIP",Self::Tar=>"TAR",Self::Gzip=>"GZIP",Self::Bzip2=>"BZIP2",Self::Xz=>"XZ",Self::SevenZip=>"7-Zip",Self::Rar=>"RAR",Self::Iso=>"ISO 9660",Self::Unknown=>"Unknown binary" } }
 fn supported(self)->bool { matches!(self,Self::Zip|Self::Tar|Self::Gzip|Self::Bzip2|Self::Xz) }
}
fn detect(path:&Path)->Result<Format,String>{
 let mut file=File::open(path).map_err(|e|format!("Cannot open file: {e}"))?;
 let mut header=[0u8;512]; let n=file.read(&mut header).map_err(|e|format!("Cannot read file: {e}"))?; let h=&header[..n];
 if h.starts_with(b"PK\\x03\\x04")||h.starts_with(b"PK\\x05\\x06")||h.starts_with(b"PK\\x07\\x08"){return Ok(Format::Zip)}
 if h.starts_with(b"7z\\xBC\\xAF\\x27\\x1C"){return Ok(Format::SevenZip)}
 if h.starts_with(b"Rar!\\x1A\\x07\\x00")||h.starts_with(b"Rar!\\x1A\\x07\\x01\\x00"){return Ok(Format::Rar)}
 if h.starts_with(&[0x1f,0x8b]){return Ok(Format::Gzip)}
 if h.starts_with(b"BZh"){return Ok(Format::Bzip2)}
 if h.starts_with(&[0xfd,b'7',b'z',b'X',b'Z',0x00]){return Ok(Format::Xz)}
 if n>=262&&&h[257..262]==b"ustar"{return Ok(Format::Tar)}
 if n>=32774&&&h[32769..32774]==b"CD001"{return Ok(Format::Iso)}
 Ok(Format::Unknown)
}
#[tauri::command]
fn inspect_file(path:String)->Result<Inspection,String>{
 let p=PathBuf::from(&path); let metadata=fs::metadata(&p).map_err(|e|format!("Cannot read file metadata: {e}"))?;
 if !metadata.is_file(){return Err("The selected path is not a regular file.".into())}
 let format=detect(&p)?; let name=p.file_name().and_then(|n|n.to_str()).unwrap_or("Unknown file").to_string();
 let description=match format {
  Format::Unknown=>"No supported archive signature found. This may be proprietary game data or a raw binary file.".to_string(),
  Format::SevenZip=>"7-Zip signature detected. Extraction support is not enabled in this build yet.".to_string(),
  Format::Rar=>"RAR signature detected. Extraction support is not enabled in this build yet.".to_string(),
  Format::Iso=>"ISO 9660 image detected. ISO browsing and extraction are not enabled yet.".to_string(),
  _=>format!("{} signature detected. This format can be extracted.",format.label())
 };
 Ok(Inspection{file_name:name,path,size:metadata.len(),format:format.label().into(),supported:format.supported(),description})
}
fn safe_relative(path:&Path)->Result<PathBuf,String>{
 let mut clean=PathBuf::new();
 for component in path.components(){match component{Component::Normal(part)=>clean.push(part),Component::CurDir=>{},_=>return Err(format!("Unsafe archive path rejected: {}",path.display()))}}
 if clean.as_os_str().is_empty(){return Err("Archive entry has an empty path.".into())} Ok(clean)
}
fn copy_stream<R:Read,W:Write>(reader:R,writer:W)->io::Result<u64>{io::copy(reader,writer)}
fn single_stream<R:Read>(reader:R,source:&Path,destination:&Path)->Result<ExtractionResult,String>{
 fs::create_dir_all(destination).map_err(|e|format!("Cannot create destination: {e}"))?;
 let original=source.file_name().and_then(|n|n.to_str()).unwrap_or("archive.bin").to_ascii_lowercase();
 let output_name=[".gz",".gzip",".bz2",".xz",".bin"].iter().find_map(|ext|original.strip_suffix(ext)).filter(|s|!s.is_empty()).unwrap_or("extracted-data");
 let output_path=destination.join(output_name);
 if output_path.exists(){return Err(format!("Refusing to overwrite existing file: {}",output_path.display()))}
 let mut output=File::create(&output_path).map_err(|e|format!("Cannot create output file: {e}"))?;
 let bytes=copy_stream(reader,&mut output).map_err(|e|format!("Decompression failed: {e}"))?;
 output.sync_all().map_err(|e|format!("Could not flush output: {e}"))?;
 Ok(ExtractionResult{files_written:1,bytes_written:bytes,destination:destination.to_string_lossy().into_owned()})
}
#[tauri::command]
fn extract_archive(path:String,destination:String)->Result<ExtractionResult,String>{
 let source=PathBuf::from(&path); let dest=PathBuf::from(&destination);
 if !source.is_file(){return Err("Source file does not exist.".into())}
 if !dest.is_dir(){return Err("Choose an existing destination folder.".into())}
 let format=detect(&source)?; if !format.supported(){return Err(format!("{} extraction is not supported yet.",format.label()))}
 match format {
  Format::Zip=>{
   let file=File::open(&source).map_err(|e|e.to_string())?;
   let mut archive=zip::ZipArchive::new(file).map_err(|e|format!("Invalid ZIP archive: {e}"))?;
   let(mut files_written,mut bytes_written)=(0u64,0u64);
   for i in 0..archive.len(){
    let mut entry=archive.by_index(i).map_err(|e|format!("Cannot read ZIP entry: {e}"))?;
    let Some(relative)=entry.enclosed_name().map(Path::to_path_buf) else{return Err(format!("Unsafe ZIP entry path rejected: {}",entry.name()))};
    let out=dest.join(relative);
    if entry.is_dir(){fs::create_dir_all(&out).map_err(|e|e.to_string())?;continue}
    if let Some(parent)=out.parent(){fs::create_dir_all(parent).map_err(|e|e.to_string())?}
    if out.exists(){return Err(format!("Refusing to overwrite existing file: {}",out.display()))}
    let mut target=File::create(&out).map_err(|e|e.to_string())?;
    bytes_written+=io::copy(&mut entry,&mut target).map_err(|e|format!("ZIP extraction failed: {e}"))?;
    target.sync_all().map_err(|e|e.to_string())?;files_written+=1;
   }
   Ok(ExtractionResult{files_written,bytes_written,destination})
  },
  Format::Tar=>{
   let file=File::open(&source).map_err(|e|e.to_string())?;let mut archive=tar::Archive::new(file);
   let entries=archive.entries().map_err(|e|format!("Invalid TAR archive: {e}"))?;
   let(mut files_written,mut bytes_written)=(0u64,0u64);
   for item in entries{
    let mut entry=item.map_err(|e|format!("Cannot read TAR entry: {e}"))?;
    let raw=entry.path().map_err(|e|e.to_string())?.into_owned();let relative=safe_relative(&raw)?;let out=dest.join(relative);
    if entry.header().entry_type().is_dir(){fs::create_dir_all(&out).map_err(|e|e.to_string())?;continue}
    if !entry.header().entry_type().is_file(){continue}
    if let Some(parent)=out.parent(){fs::create_dir_all(parent).map_err(|e|e.to_string())?}
    if out.exists(){return Err(format!("Refusing to overwrite existing file: {}",out.display()))}
    let mut target=File::create(&out).map_err(|e|e.to_string())?;
    bytes_written+=io::copy(&mut entry,&mut target).map_err(|e|format!("TAR extraction failed: {e}"))?;
    target.sync_all().map_err(|e|e.to_string())?;files_written+=1;
   }
   Ok(ExtractionResult{files_written,bytes_written,destination})
  },
  Format::Gzip=>single_stream(GzDecoder::new(File::open(&source).map_err(|e|e.to_string())?),&source,&dest),
  Format::Bzip2=>single_stream(BzDecoder::new(File::open(&source).map_err(|e|e.to_string())?),&source,&dest),
  Format::Xz=>single_stream(XzDecoder::new(File::open(&source).map_err(|e|e.to_string())?),&source,&dest),
  _=>Err("Unsupported format.".into())
 }
}
pub fn run(){
 tauri::Builder::default().plugin(tauri_plugin_dialog::init()).plugin(tauri_plugin_opener::init())
 .invoke_handler(tauri::generate_handler![inspect_file,extract_archive])
 .run(tauri::generate_context!()).expect("error while running Extracted");
}
#[cfg(test)]
mod tests{
 use super::*;use std::{fs,time::{SystemTime,UNIX_EPOCH}};
 fn temp_file(bytes:&[u8])->PathBuf{let id=SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();let p=std::env::temp_dir().join(format!("extracted-test-{id}.bin"));fs::write(&p,bytes).unwrap();p}
 #[test]fn detects_zip_signature_with_bin_extension(){let p=temp_file(b"PK\\x03\\x04payload");assert_eq!(detect(&p).unwrap(),Format::Zip);fs::remove_file(p).unwrap();}
 #[test]fn detects_seven_zip_without_claiming_support(){let p=temp_file(b"7z\\xBC\\xAF\\x27\\x1Cpayload");assert_eq!(detect(&p).unwrap(),Format::SevenZip);assert!(!Format::SevenZip.supported());fs::remove_file(p).unwrap();}
 #[test]fn rejects_parent_traversal(){assert!(safe_relative(Path::new("../outside.txt")).is_err());assert!(safe_relative(Path::new("folder/ok.txt")).is_ok());}
 #[test]fn unknown_binary_is_not_supported(){let p=temp_file(b"arbitrary game data");assert_eq!(detect(&p).unwrap(),Format::Unknown);fs::remove_file(p).unwrap();}
}