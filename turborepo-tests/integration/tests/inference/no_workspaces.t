Setup
  $ . ${TESTDIR}/../../../helpers/setup.sh
  $ . ${TESTDIR}/no_workspaces_setup.sh $(pwd)/no_workspaces

  $ cd $TARGET_DIR && ${TURBO} run build --filter=nothing -vv
  [-0-9:.TWZ+]+ \[DEBUG] turborepo_lib::shim: Global turbo version: .* (re)
  [-0-9:.TWZ+]+ \[DEBUG] turborepo_lib::shim: No local turbo binary found at: .* (re)
  [-0-9:.TWZ+]+ \[DEBUG] turborepo_lib::shim: No local turbo binary found at: .* (re)
  [-0-9:.TWZ+]+ \[DEBUG] turborepo_lib::shim: Repository Root: .*/no_workspaces (re)
  [-0-9:.TWZ+]+ \[DEBUG] turborepo_lib::shim: Running command as global turbo (re)
  [-0-9:.TWZ+]+ \[DEBUG] turborepo_lib::cli: pkg_inference_root set to "" (re)
  [-0-9:.TWZ+]+ \[DEBUG] turbo: Found go binary at "[\-\w\/]+" (re)
  [-0-9:.TWZ+]+ \[DEBUG] turbo: build tag: (go|rust) (re)
  [-0-9:.TWZ+]+ \[INFO]  turbo: skipping turbod since we appear to be in a non-interactive context (re)
  [-0-9:.TWZ+]+ \[DEBUG] turbo: global hash env vars: vars=\[] (re)
  [-0-9:.TWZ+]+ \[DEBUG] turbo: global hash: value=d497c4d703917bce (re)
  [-0-9:.TWZ+]+ \[DEBUG] turbo: local cache folder: path="" (re)
  \xe2\x80\xa2 Running build (esc)
  \xe2\x80\xa2 Remote caching disabled (esc)
  
  No tasks were executed as part of this run.
  
   Tasks:    0 successful, 0 total
  Cached:    0 cached, 0 total
    Time:\s*[\.0-9]+m?s  (re)
  
  $ cd $TARGET_DIR/parent && ${TURBO} run build --filter=nothing -vv
  [-0-9:.TWZ+]+ \[DEBUG] turborepo_lib::shim: Global turbo version: .* (re)
  [-0-9:.TWZ+]+ \[DEBUG] turborepo_lib::shim: No local turbo binary found at: .* (re)
  [-0-9:.TWZ+]+ \[DEBUG] turborepo_lib::shim: No local turbo binary found at: .* (re)
  [-0-9:.TWZ+]+ \[DEBUG] turborepo_lib::shim: Repository Root: .*/no_workspaces/parent (re)
  [-0-9:.TWZ+]+ \[DEBUG] turborepo_lib::shim: Running command as global turbo (re)
  [-0-9:.TWZ+]+ \[DEBUG] turborepo_lib::cli: pkg_inference_root set to "" (re)
  [-0-9:.TWZ+]+ \[DEBUG] turbo: Found go binary at "[\-\w\/]+" (re)
  [-0-9:.TWZ+]+ \[DEBUG] turbo: build tag: (go|rust) (re)
  [-0-9:.TWZ+]+ \[INFO]  turbo: skipping turbod since we appear to be in a non-interactive context (re)
  [-0-9:.TWZ+]+ \[DEBUG] turbo: global hash env vars: vars=\[] (re)
  [-0-9:.TWZ+]+ \[DEBUG] turbo: global hash: value=99aadb0cc485750b (re)
  [-0-9:.TWZ+]+ \[DEBUG] turbo: local cache folder: path="" (re)
  \xe2\x80\xa2 Running build (esc)
  \xe2\x80\xa2 Remote caching disabled (esc)
  
  No tasks were executed as part of this run.
  
   Tasks:    0 successful, 0 total
  Cached:    0 cached, 0 total
    Time:\s*[\.0-9]+m?s  (re)
  
  $ cd $TARGET_DIR/parent/child && ${TURBO} run build --filter=nothing -vv
  [-0-9:.TWZ+]+ \[DEBUG] turborepo_lib::shim: Global turbo version: .* (re)
  [-0-9:.TWZ+]+ \[DEBUG] turborepo_lib::shim: No local turbo binary found at: .* (re)
  [-0-9:.TWZ+]+ \[DEBUG] turborepo_lib::shim: No local turbo binary found at: .* (re)
  [-0-9:.TWZ+]+ \[DEBUG] turborepo_lib::shim: Repository Root: .*/no_workspaces/parent/child (re)
  [-0-9:.TWZ+]+ \[DEBUG] turborepo_lib::shim: Running command as global turbo (re)
  [-0-9:.TWZ+]+ \[DEBUG] turborepo_lib::cli: pkg_inference_root set to "" (re)
  [-0-9:.TWZ+]+ \[DEBUG] turbo: Found go binary at "[\-\w\/]+" (re)
  [-0-9:.TWZ+]+ \[DEBUG] turbo: build tag: (go|rust) (re)
  [-0-9:.TWZ+]+ \[INFO]  turbo: skipping turbod since we appear to be in a non-interactive context (re)
  [-0-9:.TWZ+]+ \[DEBUG] turbo: global hash env vars: vars=\[] (re)
  [-0-9:.TWZ+]+ \[DEBUG] turbo: global hash: value=674ae109880f14ca (re)
  [-0-9:.TWZ+]+ \[DEBUG] turbo: local cache folder: path="" (re)
  \xe2\x80\xa2 Running build (esc)
  \xe2\x80\xa2 Remote caching disabled (esc)
  
  No tasks were executed as part of this run.
  
   Tasks:    0 successful, 0 total
  Cached:    0 cached, 0 total
    Time:\s*[\.0-9]+m?s  (re)
  