use std::{env, fs};

use console::style;
use indicatif::{MultiProgress, ProgressStyle};
use obfuscator::Tigress;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use self::code::Code;
use super::*;

#[derive(Debug)]
pub struct Dataset<T: Obfuscator + std::marker::Sync> {
    // name: String,
    pub base_dir: PathBuf,
    pub src_dir: PathBuf,
    pub obfuscator_db: Vec<T>,
    pub docker_compose_file: Option<PathBuf>,
    pub code_db: Vec<Code>,
}

impl<T: Obfuscator + std::marker::Sync> DatasetHandler for Dataset<T> {
    fn obfuscate_each_obfuscator(&self) -> Result<()> {
        env::set_current_dir(&self.base_dir)?;

        // todo: エラーを集約し，要素が一つでもある場合はエラーを返すようにする
        let _: Vec<_> = self
            .obfuscator_db
            .iter()
            .enumerate()
            .map(|(idx, obfuscator)| {
                println!(
                    "{} Obfuscate::{}",
                    style(format!("[{}/1]", idx + 1)).bold().dim(),
                    style(format!("{}", obfuscator.get_name())).bold().dim(),
                );

                self.obfuscate_each_code(obfuscator)
            })
            .collect();

        Ok(())
    }

    fn obfuscate_each_code(
        &self,
        obfuscator: &(impl Obfuscator + std::marker::Sync),
    ) -> Result<()> {
        // todo: 引数で進捗バーの表示を無効にできるようにする
        let mp: MultiProgress = MultiProgress::new();
        let obfuscation_len: u64 = obfuscator.get_transformation_names().len() as u64;
        let pb_style: ProgressStyle = ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] {prefix} {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
            )
            .unwrap()
            .progress_chars("#>-");

        // todo: エラーを集約し，要素が一つでもある場合はエラーを返すようにする
        let _: Vec<_> = self
            .code_db
            .par_iter()
            .map(|code| {
                let pb = mp.add(ProgressBar::new(obfuscation_len));
                pb.set_style(pb_style.clone());
                pb.set_prefix(format!("{:<10}", code.target));

                obfuscator.obfuscate_by_all_obfuscation(
                    &code.src_path,
                    &Dataset::<T>::get_dst_dir_path(code),
                    &code.function,
                    &Some(pb),
                )
            })
            .collect();

        Ok(())
    }

    fn organize_each_code(&self) -> Result<()> {
        todo!()
    }
}

impl<T: Obfuscator + std::marker::Sync> From<DatasetSerealizeModel> for Dataset<T>
where
    Vec<T>: FromIterator<Tigress>,
{
    fn from(model: DatasetSerealizeModel) -> Self {
        let obfuscator_db: Vec<T> = model
            .obfuscator_db
            .iter()
            .map(|p| match p.file_stem().unwrap().to_str() {
                Some("tigress") => Tigress::new(&p, &model.docker_compose_file),
                _ => unimplemented!(),
            })
            .collect();

        Self {
            // name: model.name,
            base_dir: model.base_dir,
            src_dir: model.src_dir,
            obfuscator_db: obfuscator_db,
            docker_compose_file: model.docker_compose_file,
            code_db: model.code_db,
        }
    }
}

impl<T: Obfuscator + std::marker::Sync> Dataset<T> {
    pub fn get_dst_dir_path(code: &Code) -> PathBuf {
        code.src_path.parent().unwrap().join("obfuscated_raw/")
    }

    pub fn get_dst_adj_dir_path(code: &Code) -> PathBuf {
        code.src_path.parent().unwrap().join("obfuscated/")
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DatasetSerealizeModel {
    #[serde(skip)]
    base_dir: PathBuf,
    name: String,
    src_dir: PathBuf,
    obfuscator_db: Vec<PathBuf>,
    docker_compose_file: Option<PathBuf>,
    code_db: Vec<Code>,
}

impl DatasetSerealizeModel {
    pub fn new(path: &PathBuf) -> Self {
        let dataset_file_path = fs::canonicalize(path).unwrap();
        let file: File = File::open(&dataset_file_path).unwrap();
        let rdr: BufReader<File> = BufReader::new(file);
        let mut model: DatasetSerealizeModel = serde_json::from_reader(rdr).unwrap();

        let dataset_dir_path = PathBuf::from(&dataset_file_path.parent().unwrap());
        model.set_base_dir(dataset_dir_path);

        model
    }

    fn set_base_dir(&mut self, base_dir: PathBuf) {
        self.base_dir = base_dir;

        self.src_dir = fs::canonicalize(&self.base_dir.join(&self.src_dir)).unwrap();

        self.obfuscator_db = self
            .obfuscator_db
            .iter()
            .map(|p| fs::canonicalize(self.base_dir.join(p)).unwrap())
            .collect();

        self.docker_compose_file = self.get_docker_compose_file_path(&self.base_dir);

        self.code_db
            .iter_mut()
            .for_each(|x| x.set_src_path(&self.src_dir));
    }

    fn get_docker_compose_file_path(&self, base_dir_path: &PathBuf) -> Option<PathBuf> {
        if let Some(f) = &self.docker_compose_file {
            Some(fs::canonicalize(base_dir_path.join(&f)).expect(&format!(
                "[SerealizeModel::Dataset] File not found: {:?}",
                f
            )))
        } else {
            None
        }
    }
}
