use std::{fs::Metadata, io::Read, path::Path};

use glob_match::glob_match;
use hex::ToHex;
use ignore::WalkBuilder;
use path_slash::PathExt;
use sha1::{Digest, Sha1};
use turbopath::{AbsoluteSystemPathBuf, AnchoredSystemPathBuf, PathError};

use crate::{package_deps::GitHashes, Error};

fn git_like_hash_file(path: &AbsoluteSystemPathBuf, metadata: &Metadata) -> Result<String, Error> {
    let mut hasher = Sha1::new();
    let mut f = path.open()?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    hasher.update("blob ".as_bytes());
    hasher.update(metadata.len().to_string().as_bytes());
    hasher.update(&[b'\0']);
    hasher.update(buffer.as_slice());
    let result = hasher.finalize();
    Ok(result.encode_hex::<String>())
}

fn get_package_file_hashes_from_processing_gitignore(
    turbo_root: &AbsoluteSystemPathBuf,
    package_path: &AnchoredSystemPathBuf,
    inputs: &[&str],
) -> Result<GitHashes, Error> {
    let full_package_path = turbo_root.resolve(package_path);
    let mut hashes = GitHashes::new();

    let mut walker_builder = WalkBuilder::new(&full_package_path);
    let mut includes = Vec::new();
    let mut excludes = Vec::new();
    // for pattern in inputs {
    //     if pattern.starts_with("!") {
    //         excludes.push(full_package_path.join_literal(&pattern[1..]).
    // to_string());     } else {
    //         includes.push(full_package_path.join_literal(pattern).to_string());
    //     }
    // }
    for pattern in inputs {
        if pattern.starts_with("!") {
            let glob = Path::new(&pattern[1..])
                .to_slash()
                .ok_or_else(|| PathError::invalid_utf8_error(pattern[1..].as_bytes()))?;
            excludes.push(glob);
        } else {
            let glob = Path::new(pattern)
                .to_slash()
                .ok_or_else(|| PathError::invalid_utf8_error(pattern.as_bytes()))?;
            includes.push(glob);
        }
    }
    let include_pattern = if includes.is_empty() {
        None
    } else {
        Some(format!("{{{}}}", includes.join(",")))
    };
    let exclude_pattern = if excludes.is_empty() {
        None
    } else {
        Some(format!("{{{}}}", excludes.join(",")))
    };
    let walker = walker_builder
        .follow_links(false)
        .git_ignore(true)
        .require_git(false)
        .build();
    for dirent in walker {
        let dirent = dirent?;
        let metadata = dirent.metadata()?;
        // We need to do this here, rather than as a filter, because the root
        // directory is always yielded and not subject to the supplied filter.
        if metadata.is_dir() {
            continue;
        }
        let path = AbsoluteSystemPathBuf::new(dirent.path())?;
        let relative_path = full_package_path.anchor(&path)?;
        let relative_path = relative_path.to_unix()?;
        if let Some(include_pattern) = include_pattern.as_ref() {
            if !glob_match(include_pattern, relative_path.as_str()?).unwrap_or(false) {
                continue;
            }
        }
        if let Some(exclude_pattern) = exclude_pattern.as_ref() {
            if glob_match(exclude_pattern, relative_path.as_str()?).unwrap_or(false) {
                continue;
            }
        }
        let hash = git_like_hash_file(&path, &metadata)?;
        hashes.insert(relative_path, hash);
    }
    Ok(hashes)
}

#[cfg(test)]
mod tests {
    use turbopath::RelativeUnixPath;

    use super::*;

    fn tmp_dir() -> (tempfile::TempDir, AbsoluteSystemPathBuf) {
        let tmp_dir = tempfile::tempdir().unwrap();
        let dir = AbsoluteSystemPathBuf::new(tmp_dir.path().to_path_buf())
            .unwrap()
            .to_realpath()
            .unwrap();
        (tmp_dir, dir)
    }

    #[test]
    fn test_get_package_file_hashes_from_processing_gitignore() {
        let root_ignore_contents = ["ignoreme", "ignorethisdir/"].join("\n");
        let pkg_ignore_contents = ["pkgignoreme", "pkgignorethisdir/"].join("\n");

        let (_tmp, turbo_root) = tmp_dir();

        let pkg_path = AnchoredSystemPathBuf::from_raw("child-dir/libA").unwrap();
        let unix_pkg_path = pkg_path.to_unix().unwrap();
        let file_hash: Vec<(&str, &str, Option<&str>)> = vec![
            ("top-level-file", "top-level-file-contents", None),
            ("other-dir/other-dir-file", "other-dir-file-contents", None),
            ("ignoreme", "anything", None),
            (
                "child-dir/libA/some-file",
                "some-file-contents",
                Some("7e59c6a6ea9098c6d3beb00e753e2c54ea502311"),
            ),
            (
                "child-dir/libA/some-dir/other-file",
                "some-file-contents",
                Some("7e59c6a6ea9098c6d3beb00e753e2c54ea502311"),
            ),
            (
                "child-dir/libA/some-dir/another-one",
                "some-file-contents",
                Some("7e59c6a6ea9098c6d3beb00e753e2c54ea502311"),
            ),
            (
                "child-dir/libA/some-dir/excluded-file",
                "some-file-contents",
                Some("7e59c6a6ea9098c6d3beb00e753e2c54ea502311"),
            ),
            ("child-dir/libA/ignoreme", "anything", None),
            ("child-dir/libA/ignorethisdir/anything", "anything", None),
            ("child-dir/libA/pkgignoreme", "anything", None),
            ("child-dir/libA/pkgignorethisdir/file", "anything", None),
        ];

        let root_ignore_file = turbo_root.join_literal(".gitignore");
        root_ignore_file
            .create_with_contents(&root_ignore_contents)
            .unwrap();
        let pkg_ignore_file = turbo_root.resolve(&pkg_path).join_literal(".gitignore");
        pkg_ignore_file.ensure_dir().unwrap();
        pkg_ignore_file
            .create_with_contents(&pkg_ignore_contents)
            .unwrap();

        let mut expected = GitHashes::new();
        for (raw_unix_path, contents, expected_hash) in file_hash.iter() {
            let unix_path = RelativeUnixPath::new(&raw_unix_path).unwrap();
            let file_path = turbo_root.join_unix_path(unix_path).unwrap();
            file_path.ensure_dir().unwrap();
            file_path.create_with_contents(contents).unwrap();
            if let Some(hash) = expected_hash {
                let unix_pkg_file_path = unix_path.strip_prefix(&unix_pkg_path).unwrap();
                expected.insert(unix_pkg_file_path, (*hash).to_owned());
            }
        }

        let hashes =
            get_package_file_hashes_from_processing_gitignore(&turbo_root, &pkg_path, &[]).unwrap();
        assert_eq!(hashes, expected);

        expected = GitHashes::new();
        for (raw_unix_path, contents, expected_hash) in file_hash.iter() {
            let unix_path = RelativeUnixPath::new(&raw_unix_path).unwrap();
            let file_path = turbo_root.join_unix_path(unix_path).unwrap();
            file_path.ensure_dir().unwrap();
            file_path.create_with_contents(contents).unwrap();
            if let Some(hash) = expected_hash {
                let unix_pkg_file_path = unix_path.strip_prefix(&unix_pkg_path).unwrap();
                if unix_pkg_file_path.ends_with("file")
                    && !unix_pkg_file_path.ends_with("excluded-file")
                {
                    expected.insert(unix_pkg_file_path, (*hash).to_owned());
                }
            }
        }

        let hashes = get_package_file_hashes_from_processing_gitignore(
            &turbo_root,
            &pkg_path,
            &["**/*file", "!some-dir/excluded-file"],
        )
        .unwrap();
        assert_eq!(hashes, expected);
    }
}
