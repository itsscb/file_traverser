use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{
    fs,
    path::{Path, PathBuf},
};
use tracing::{debug, error, trace};

use crate::channel::GenericSender;

#[allow(dead_code)]
pub fn filter_and_send_files<F, S>(path: &PathBuf, file_tx: S, filter: F)
where
    F: Fn(&Path) -> bool + Send + Sync + Clone,
    S: GenericSender<PathBuf> + Clone + Send + Sync,
{
    if !filter(path) {
        debug!(path = ?path, tag="skipped", "does not match filter");
        return;
    }
    trace!(path = ?path, tag="entered");
    if path.is_dir() {
        let files = match fs::read_dir(path) {
            Ok(files) => files,
            Err(e) => {
                error!(path = ?path, error = ?e, "failed to read directory");
                return;
            }
        };

        let entries = files.flatten().collect::<Vec<_>>();

        entries
            .into_par_iter()
            .filter(|e| filter(&e.path()))
            .for_each(|entry| {
                if entry.metadata().map(|m| !m.is_dir()).unwrap_or(false) {
                    let _ = file_tx.send(entry.path());
                } else {
                    filter_and_send_files(&entry.path(), file_tx.clone(), filter.clone());
                }
            });
    }
    trace!(path = ?path, tag="done");
    drop(file_tx);
}

#[cfg(test)]
mod test {
    #![allow(clippy::unwrap_used)]

    use super::*;

    #[tokio::test]
    async fn test_async_traverse_and_send() {
        let handle = tokio::spawn(async move {
            let filter = |p: &Path| {
                p.is_dir()
                    && p.file_name()
                        .is_some_and(|v| v.to_str().is_some_and(|f| f != "target"))
                    || p.extension().and_then(|ext| ext.to_str()).unwrap_or("") == "lock"
            };

            let path = std::env::current_dir().unwrap();

            #[cfg(not(feature = "crossbeam"))]
            {
                use std::sync::mpsc;
                let (tx, rx) = mpsc::channel();
                filter_and_send_files(&path, tx, filter);
                let files: Vec<_> = rx.iter().collect();
                assert!(!files.is_empty());
                assert_eq!(files.len(), 1);
            }

            #[cfg(feature = "crossbeam")]
            {
                let (tx, rx) = crossbeam_channel::bounded(10);
                filter_and_send_files(&path, tx, filter);
                let files: Vec<_> = rx.iter().collect();
                assert!(!files.is_empty());
                assert_eq!(files.len(), 1);
            }
        });

        handle.await.unwrap();
    }

    #[test]
    fn test_traverse_and_send() {
        let filter = |p: &Path| {
            p.is_dir()
                && p.file_name()
                    .is_some_and(|v| v.to_str().is_some_and(|f| f != "target"))
                || p.extension().and_then(|ext| ext.to_str()).unwrap_or("") == "lock"
        };

        let path = std::env::current_dir().unwrap();

        #[cfg(not(feature = "crossbeam"))]
        {
            use std::sync::mpsc;
            let (tx, rx) = mpsc::channel();
            filter_and_send_files(&path, tx, filter);
            let files: Vec<_> = rx.iter().collect();
            assert!(!files.is_empty());
            assert_eq!(files.len(), 1);
        }

        #[cfg(feature = "crossbeam")]
        {
            let (tx, rx) = crossbeam_channel::bounded(10);
            filter_and_send_files(&path, tx, filter);
            let files: Vec<_> = rx.iter().collect();
            assert!(!files.is_empty());
            assert_eq!(files.len(), 1);
        }
    }
}
