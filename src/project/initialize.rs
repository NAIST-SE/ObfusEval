// use super::{BundleInfo, Initialize, ProjectService, ProjectSourceInfo};
// use anyhow::Result;
// use duct::cmd;
// use itertools::Itertools;
// use std::{
//     fs::{self, OpenOptions},
//     path::PathBuf,
// };

// impl Initialize for ProjectService {
//     fn initialize(&self, dry_run: &bool) -> Result<()> {
//         let _ = dry_run;
//         if self.is_initialized() {
//             return Ok(());
//         }

//         let repo_path = &self.path.cache_home.join("repo");
//         self.source.get_repository(&repo_path, dry_run)?;

//         // 配置
//         // todo!();

//         OpenOptions::new()
//             .create(true)
//             .write(true)
//             .open(self.is_initialized_file())?;
//         Ok(())
//     }

//     fn is_initialized(&self) -> bool {
//         self.is_initialized_file().exists()
//     }
// }

// impl ProjectService {
//     fn is_initialized_file(&self) -> PathBuf {
//         self.path.cache_home.join(".is_initialized")
//     }
// }

// impl ProjectSourceInfo {
//     fn repository_name(&self) -> String {
//         PathBuf::from(&self.repository)
//             .file_stem()
//             .unwrap()
//             .to_string_lossy()
//             .into_owned()
//     }

//     fn get_repository(&self, path: &PathBuf, dry_run: &bool) -> Result<()> {
//         let repository_dir = path.join(&self.repository_name());
//         if repository_dir.exists() {
//             if *dry_run {
//                 println!("git pull");
//             } else {
//                 cmd("git", ["pull"]).dir(&repository_dir).read()?;
//             }
//             return Ok(());
//         }

//         if *dry_run {
//             println!("git clone {} --branch {}", &self.repository, &self.branch);
//         } else {
//             fs::create_dir_all(path)?;
//             cmd("git", ["clone", &self.repository, "--branch", &self.branch])
//                 .dir(&path)
//                 .read()?;
//         }

//         Ok(())
//     }
// }

// impl<'in_project> Initialize for BundleInfo<'in_project> {
//     fn initialize(&self, dry_run: &bool) -> Result<()> {
//         if self.is_initialized() {
//             return Ok(());
//         }

//         if *dry_run {
//             self.list_result_path()
//                 .iter()
//                 .for_each(|x| println!("mkdir -p {}", x.display()));
//             return Ok(());
//         }

//         fs::create_dir_all(&self.root)?;
//         for x in self.list_result_path() {
//             fs::create_dir_all(&x)?;
//         }

//         Ok(())
//     }

//     fn is_initialized(&self) -> bool {
//         !self.list_result_path().iter().any(|x| !x.exists())
//     }
// }

// impl<'in_project> BundleInfo<'in_project> {
//     fn list_result_path(&self) -> Vec<PathBuf> {
//         let binary_dir_path = self
//             .optimization
//             .iter()
//             .flat_map(|optimization| {
//                 if let Some(obfuscation) = &self.obfuscation {
//                     obfuscation
//                         .iter()
//                         .map(move |x| self.optimization_root(&optimization, Some(&x.name)))
//                         .collect::<Vec<_>>()
//                 } else {
//                     vec![self.optimization_root(&optimization, None)]
//                 }
//             })
//             .collect::<Vec<_>>();

//         let test_binary_dir_path = self
//             .optimization
//             .iter()
//             .flat_map(|optimization| {
//                 if let Some(obfuscation) = &self.obfuscation {
//                     obfuscation
//                         .iter()
//                         .map(move |x| self.test_root(&optimization, Some(&x.name)))
//                         .collect::<Vec<_>>()
//                 } else {
//                     vec![self.test_root(&optimization, None)]
//                 }
//             })
//             .collect::<Vec<_>>();

//         let obfuscated_src_dir_path = self
//             .obfuscation
//             .iter()
//             .flat_map(|obfuscation| {
//                 obfuscation
//                     .iter()
//                     .map(|x| self.src_dir(Some(&x.name)))
//                     .collect::<Vec<PathBuf>>()
//             })
//             .collect::<Vec<_>>();

//         [
//             binary_dir_path,
//             test_binary_dir_path,
//             obfuscated_src_dir_path,
//         ]
//         .concat()
//         .into_iter()
//         .sorted()
//         .collect()
//     }
// }
