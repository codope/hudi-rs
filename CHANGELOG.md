<!--
  ~ Licensed to the Apache Software Foundation (ASF) under one
  ~ or more contributor license agreements.  See the NOTICE file
  ~ distributed with this work for additional information
  ~ regarding copyright ownership.  The ASF licenses this file
  ~ to you under the Apache License, Version 2.0 (the
  ~ "License"); you may not use this file except in compliance
  ~ with the License.  You may obtain a copy of the License at
  ~
  ~   http://www.apache.org/licenses/LICENSE-2.0
  ~
  ~ Unless required by applicable law or agreed to in writing,
  ~ software distributed under the License is distributed on an
  ~ "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
  ~ KIND, either express or implied.  See the License for the
  ~ specific language governing permissions and limitations
  ~ under the License.
-->

# Changelog

All notable changes to this project will be documented in this file.

## [0.4.0] - 2025-07-03

### 🚀 Features

- Support reading avro data block for MOR tables (#309) by @xushiyan
- Add table and timeline APIs for retrieving useful info (#313) by @xushiyan
- Add blocking APIs for `Table` and `FileGroupReader` (#321) by @xushiyan
- Support more timestamp formats for time-travel and incremental APIs (#302) by @hanbings
- Add C++ APIs for `FileGroupReader` (#322) by @xushiyan
- Support reading MOR delete block (#356) by @xushiyan
- Update python setup to support 3.13 (#361) by @xushiyan

### 🐛 Bug Fixes

- Make local timezone test more robust (#308) by @xushiyan
- Resolve env vars for creating standalone file group reader (#345) by @xushiyan
- Handle zero event time ordering (#357) by @xushiyan
- Handle schema resolution for empty commit (#359) by @xushiyan

### 🚜 Refactor

- Move `Makefile` to root directory (#283) by @nakul-py
- Use `OptionResolver` to handle options (#341) by @xushiyan
- Improve schema resolution flow (#364) by @xushiyan

### 📚 Documentation

- Update readme docs and the project description (#348) by @xushiyan

### 🛠️ Build

- *(deps)* Upgrade arrow, pyo3, datafusion, rustc (#297) by @xushiyan
- *(deps)* Bump apache/skywalking-eyes from 0.6.0 to 0.7.0 (#300) by @dependabot[bot]
- *(deps)* Bump ruff from 0.5.2 to 0.9.10 in /python (#307) by @dependabot[bot]
- *(deps)* Bump ruff from 0.11.0 to 0.11.2 in /python (#315) by @dependabot[bot]
- *(deps)* Bump ruff from 0.11.2 to 0.11.7 in /python (#327) by @dependabot[bot]
- *(deps)* Bump mypy from 1.10.1 to 1.15.0 in /python (#298) by @dependabot[bot]
- *(deps)* Update zip-extract requirement from 0.2 to 0.3 (#332) by @dependabot[bot]
- *(deps)* Add `-dev` for current development version (#362) by @xushiyan
- *(deps)* Upgrade datafusion & arrow, and restrict deps upgrade to patch-level (#386) by @xushiyan

### ⚙️ Miscellaneous Tasks

- Fix src verify script (#279) by @xushiyan
- Update release guide and issue templates (#282) by @xushiyan
- Update changelog for 0.3.0 (#288) by @xushiyan
- Update dependabot settings and readme links (#294) by @xushiyan
- Fix codecov report generation (#316) by @xushiyan
- Fix codecov upload issue (#318) by @xushiyan
- *(ci)* Update python test setup and upload nightly wheels (#320) by @xushiyan
- *(ci)* Parallel run integration tests (#340) by @xushiyan
- Update `asf.yaml` for more automation (#349) by @xushiyan
- *(ci)* Replace archived actions-rs actions (#351) by @assignUser

## New Contributors

* @assignUser made their first contribution in #351

* @hanbings made their first contribution in #302

* @nakul-py made their first contribution in #283

<!-- generated by git-cliff -->

## [0.3.0] - 2025-02-02

### 🚀 Features

- Define Hudi error types across hudi-core (#124) by @gohalo
- Support filter pushdown for datafusion (#203) by @jonathanc-n
- Add demo app and integration tests (#226) by @xushiyan
- Add `TimelineSelector` to support timeline loading (#233) by @xushiyan
- Add `hoodie.read.listing.parallelism` config (#235) by @xushiyan
- Support row filters for `FileGroupReader` (#237) by @xushiyan
- Implement incremental query for COW tables (#236) by @xushiyan
- Implement log file reader for parquet log block (#244) by @xushiyan
- Implement basic record merge semantics (#249) by @xushiyan
- Add APIs for MOR snapshot reads (#247) by @xushiyan
- Support time travel query for MOR tables (#256) by @xushiyan
- Support incremental read MOR tables (#258) by @xushiyan
- Support MOR read-optimized query (#259) by @xushiyan
- Support reading MOR with rollback (#264) by @xushiyan
- Align python table APIs with rust (#267) by @xushiyan
- Add APIs to support incremental query impl (#272) by @xushiyan

### 🐛 Bug Fixes

- Simplify partition filter format by taking tuple of strings (#170) by @kazdy
- Improve api to get file slices splits (#185) by @xushiyan
- Handle schema retrieval for datafusion api (#187) by @xushiyan
- Include commit_seqno for merge order (#250) by @xushiyan
- Format Hudi config enum should show the full config key (#254) by @Kunal-Singh-Dadhwal
- Derive record merge strategy based on table configs (#260) by @xushiyan
- Handle as-of timestamp for excluding file groups (#268) by @xushiyan
- Build up incremental file groups (#273) by @xushiyan

### 🚜 Refactor

- Reorganize custom error types (#215) by @xushiyan
- Add API stubs for performing incremental queries (#220) by @xushiyan
- Enhance `Filter` and related structs (#221) by @xushiyan
- Improve `TimelineSelector` API (#234) by @xushiyan
- Improve `BaseFile` APIs (#239) by @xushiyan
- Improve file system view's listing flow (#251) by @xushiyan
- Use static MetaField schema for incr query (#252) by @xushiyan
- Rename crate `hudi-tests` to `hudi-test` (#262) by @xushiyan
- Remove use of `Filter` from public APIs (#266) by @xushiyan

### 📚 Documentation

- Update README examples (#194) by @xushiyan
- Update release and dev guides (#195) by @xushiyan
- Add example to `hudi-datafusion` crate (#202) by @jonathanc-n
- Add `CREATE EXTERNAL TABLE` example in datafusion crate (#213) by @jonathanc-n
- Clarify issues in the dev guide  (#224) by @xushiyan
- Add in-code docs for `FileGroup` (#269) by @xushiyan
- Update `README.md` to show table API examples (#274) by @xushiyan

### 🛠️ Build

- *(deps)* Bump codecov/codecov-action from 4 to 5 (#184) by @dependabot[bot]
- *(deps)* Upgrade datafusion and object store (#182) by @kazdy
- *(deps)* Upgrade datafusion to 42.2.0 (#192) by @xushiyan
- *(deps)* Upgrade Datafusion, Arrow, and Rust versions (#197) by @jonathanc-n
- *(deps)* Update pyo3 requirement from 0.22.2 to 0.22.4 (#212) by @jonathanc-n
- *(deps)* Clean up dependencies (#240) by @xushiyan
- *(dep)* Upgrade rustc, arrow, and tarpaulin setting (#276) by @xushiyan

### ⚙️ Miscellaneous Tasks

- Update release script and guide (#200) by @xushiyan
- Update changelog for 0.2.0 (#201) by @xushiyan
- Update pull request guidelines for contributors (#204) by @jonathanc-n
- Add more dev commands and update the project's short description (#217) by @xushiyan
- Update codecov threshold (#222) by @xushiyan
- Update codecov config (#245) by @xushiyan
- Update codecov-action to v5 (#248) by @K-dash
- *(ci)* Add rust dependency caching with rust-cache action (#265) by @K-dash
- Fix src verify script (#279)
- Update release guide and issue templates (#282)

## New Contributors

* @K-dash made their first contribution in #265

* @Kunal-Singh-Dadhwal made their first contribution in #254

* @jonathanc-n made their first contribution in #203

<!-- generated by git-cliff -->

## [0.2.0] - 2024-11-25

### 🚀 Features

- Support loading hudi global configs (#118) by @zzhpro
- Add base file records' in-memory size to `FileStats` (#140) by @xushiyan
- Support partition prune api (#119) by @KnightChess
- Add partition filter arg in Python APIs (#153) by @xushiyan
- Add `HudiFileGroupReader` with consolidated APIs to read records (#164) by @xushiyan
- Add `TableBuilder` API for creating `Table` instances (#163) by @kazdy
- Implement datafusion `TableProviderFactory` (#162) by @kazdy

### 🐛 Bug Fixes

- Register object store with datafusion (#107) by @abyssnlp
- Handle validating table when `DropsPartitionFields` not present (#142) by @xushiyan
- Make partition loading more efficient (#152) by @xushiyan
- Simplify partition filter format by taking tuple of strings (#170)
- Improve api to get file slices splits (#185)
- Handle schema retrieval for datafusion api (#187)

### 🚜 Refactor

- Extract common test code for creating table (#117) by @gohalo
- Improve APIs for handling options (#161) by @xushiyan
- Improve `TableBuilder` API for taking single option (#171) by @xushiyan
- Minor improvement to fix coverage report status (#173) by @xushiyan

### 📚 Documentation

- Update readme logo and example (#65) by @xushiyan
- Update in-code comments (#132) by @KnightChess
- Add hudi core API docs with examples (#113) by @KnightChess
- Add in-code docs to hudi-core APIs (#166) by @xushiyan
- Add python binding docstrings (#169) by @kazdy
- Add step-by-step release guide (#66) by @xushiyan

### 🎨 Styling

- Enforce Python code style (#101) by @muyihao

### 🛠️ Build

- Use exact versions for arrow and datafusion (#105) by @xushiyan
- Bump up datafusion to version 41, arrow to 52.2 (#120) by @yjshen
- *(deps)* Update zip-extract requirement from 0.1.3 to 0.2.1 (#130) by @dependabot[bot]
- *(deps)* Upgrade datafusion, pyarrow, pyo3, python versions  (#149) by @kazdy
- *(deps)* Upgrade arrow dependencies (#168) by @kazdy
- *(release)* Bump version to 0.2.0-rc.1
- *(deps)* Upgrade datafusion and object store (#182)
- *(deps)* Upgrade datafusion to 42.2.0 (#192)
- *(release)* Bump version to 0.2.0-rc.2

### ⚙️ Miscellaneous Tasks

- Improve release scripts (#68) by @xushiyan
- Add `CHANGELOG.md` with git-cliff config (#69) by @xushiyan
- Configure labeler for PRs from forked repos (#83) by @xushiyan
- Fix labeler config (#85) by @xushiyan
- Fix labeler config for dev-x (#87) by @xushiyan
- Merge python code coverage report with rust (#67) by @xushiyan
- Add pull request template (#89) by @xushiyan
- Enable dependabot (#94) by @xushiyan
- Add path ignore files for ci workflow (#93) by @abyssnlp
- Improve workflows for code checking and PR (#110) by @xushiyan
- Disable labeler due to permission and policy (#115) by @xushiyan
- *(ci)* Fix PR title linting to support change scope (#138) by @kazdy
- Add feature request template for GH issues (#167) by @kazdy

## New Contributors

* @KnightChess made their first contribution in #119

* @gohalo made their first contribution in #117

* @zzhpro made their first contribution in #118

* @yjshen made their first contribution in #120

* @abyssnlp made their first contribution in #107

* @muyihao made their first contribution in #101

<!-- generated by git-cliff -->

## [0.1.0] - 2024-07-15

### 🚀 Features

- Initial rust implementation to integrate with datafusion (#1) by @xushiyan
- Add python binding (#21) by @xushiyan
- Implement `HudiTable` as python API (#23) by @xushiyan
- Use `object_store` for common storage APIs (#25) by @xushiyan
- Implement Rust and Python APIs to read file slices (#28) by @xushiyan
- Add APIs for time-travel read (#33) by @xushiyan
- Implement datafusion API using ParquetExec (#35) by @xushiyan
- Add `HudiConfigs` for parsing and managing named configs (#37) by @xushiyan
- Add config validation when creating table (#49) by @xushiyan
- Add internal config to skip validation (#51) by @xushiyan
- Support time travel with read option (#52) by @xushiyan
- Support taking env vars for cloud storages (#55) by @xushiyan

### 🐛 Bug Fixes

- Handle replacecommit for loading file slices (#53) by @xushiyan

### 🚜 Refactor

- Use `anyhow` for generic errors (#26) by @xushiyan
- Use `object_store` API for Timeline (#27) by @xushiyan
- Make APIs async (#31) by @xushiyan
- Improve thread safety and error handling (#32) by @xushiyan
- Improve error handling in storage module (#34) by @xushiyan
- Adjust table APIs to skip passing options (#56) by @xushiyan

### 📚 Documentation

- Update readme, contributing guide, and issue template (#57) by @xushiyan
- Update CONTRIBUTING with minor changes (#58) by @codope

### 🎨 Styling

- Enforce rust code style (#14) by @xushiyan

### 🛠️ Build

- Clean up and trim down dependencies (#54) by @xushiyan
- Add info for rust and python artifacts (#60) by @xushiyan
- Add release workflow (#63) by @xushiyan

### 🧪 Testing

- Add tests crate and adopt testing tables (#30) by @xushiyan
- Add test cases for different table setup (#36) by @xushiyan

### ⚙️ Miscellaneous Tasks

- Setup ci for license file and headers (#2) by @xushiyan
- Fix failing check and test case (#10) by @xushiyan
- Fix asf notification (#11) by @xushiyan
- Add commit linting (#12) by @xushiyan
- Use cargo tarpaulin to generate code coverage (#15) by @xushiyan
- Remove codecov to keep ci green (#17) by @xushiyan
- Fix codecov setup (#20) by @xushiyan
- Configure codecov (#50) by @xushiyan
- Add scripts to streamline source release (#64) by @xushiyan

## New Contributors

* @codope made their first contribution in #58
* @xushiyan made their first contribution in #1

<!-- generated by git-cliff -->
