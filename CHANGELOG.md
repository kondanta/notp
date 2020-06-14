# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog], and this project adheres to
[Semantic Versioning]. The file is auto-generated using [Conventional Commits].

[keep a changelog]: https://keepachangelog.com/en/1.0.0/
[semantic versioning]: https://semver.org/spec/v2.0.0.html
[conventional commits]: https://www.conventionalcommits.org/en/v1.0.0-beta.4/

## Overview

- [unreleased](#unreleased)
- [`2.1.0`](#210) – _2020.06.04_
- [`2.0.0`](#200) – _2020.05.12_

## _[Unreleased]_

- feat: add clipboard support ([`7c5f69c`])
- docs: add jilu custom config ([`8f5193c`])
- docs: switch to jilu from clog ([`7883d83`])
- chore: fix clippy warnings ([`b7c2344`])
- docs: update readme ([`be11b3a`])
- test: add missing ops tests ([`5c1e759`])
- style: run rust fmt ([`302e7e2`])
- refactor: swap closure name to more understandable one ([`f7545c5`])
- docs: add more docstring on public functions ([`f501243`])
- test: add tests for add operation ([`7da6c38`])
- refactor: return function result directly ([`6cbf99c`])
- refactor: stop using early return in ne ([`c4f751a`])
- refactor: use result in ops matcher ([`366f414`])
- test: refactor test names ([`90a5890`])
- refactor: stop using early returns ([`b6fa738`])
- refactor: remove hash based generator ([`752c0f5`])
- test: add some unit tests ([`362fd9d`])
- refactor: print error messages to the user ([`cd67e72`])
- refactor: remove commented out function ([`aeffd8a`])
- refactor: return actual result from add ([`4285a87`])
- refactor: return actual result from get ([`a25a848`])
- refactor: add init function to kv store ([`05b6a69`])
- refactor: remove folder creation in get func ([`770f8d9`])
- feat: add remove folder ([`43d5b48`])
- refactor: separate create folder function ([`02b8bff`])
- refactor: get rid of scope operator ([`4ebd0fa`])
- refactor: rename is to does ([`e4afde7`])
- refactor: add OTP error type ([`4e32465`])
- feat: add new otp generator library ([`b2f7836`])
- refactor: add OTP trait ([`6301e74`])
- style: put empty line ([`3070654`])
- chore: update dependencies ([`f5db77c`])
- refactor: try to extract dispatchers function body ([`8d9fd56`])
- feat: add version checker ([`b25fec2`])
- refactor: remove redundant function ([`c3fba1e`])
- refactor: encapsulate params in sub commands ([`37deb4c`])
- chore: update dependencies ([`605c918`])
- refactor: simplify main.rs ([`0870e12`])
- refactor: rewrite commands as subcommand enum ([`63839ba`])
- chore: use ok_or_else instead of ok_or ([`e5beb80`])
- refactor: rewrite operations dispatcher ([`0cdb48a`])
- chore: update gitignore ([`5a7f241`])
- refactor: change token's option to result ([`c7ac3eb`])
- fix: fix the issue of panicking ([`d3f79ae`])
- refactor: swap main with the new dispatcher ([`9b53d71`])
- docs: add missing comments ([`e706a20`])
- refactor: introduce dispatcher for operations ([`9e1e599`])
- refactor: introduce operationtypes ([`eda7350`])
- ci: use nightly insted of stable ([`64dfddc`])

## [2.1.0]

_2020.06.04_

### Contributions

This release is made possible by the following people (in alphabetical order).
Thank you all for your contributions. Your work – no matter how significant – is
greatly appreciated by the community. 💖

- Taylan Dogan (<doganntaylan@gmail.com>)

### Changes

#### Documentation

- **update changelog** ([`8f66a37`])

- **update changelog.md** ([`6ae9cca`])

- **add ci status in readme** ([`adeb006`])

#### Bug Fixes

- **force default feature to depend kv-store** ([`3306395`])

- **fix clappy's depreciated lint values** ([`b80cfde`])

- **fix broken modules after major update** ([`fdfcf2a`])

#### Miscellaneous Tasks

- **bump version** ([`53fc9d7`])

- **switch to rust 2018 module path** ([`f7f201f`])

- **update dependencies** ([`cbf0ab4`])

#### Features

- **enable conditional compiling** ([`e585e57`])

  Now we can use different database backend(s) for storing data using
  --feature-flags.

  There are no new backend(s) for this project, but there might be in the
  future :)

- **implement generic param. struct** ([`922df8c`])

- **implement kv storage for datastorage** ([`9a53fd6`])

- **add store trait** ([`66b9baa`])

#### Refactoring

- **adapt main.rs to operational changes** ([`3703c2e`])

- **database interactions** ([`51b25d9`])

- **remove TODO** ([`ecd22f6`])

  I was going to add init function but new function already utilizes the
  create_folder_if_not_exists pattern. So no need.

#### ci

- **use fmt with nightly** ([`f8860e6`])

- **add cd yaml file for github actions** ([`aeabfc4`])

- **create ci yml for github actions** ([`4133d25`])

## [2.0.0]

_2020.05.12_

### Contributions

This release is made possible by the following people (in alphabetical order).
Thank you all for your contributions. Your work – no matter how significant – is
greatly appreciated by the community. 💖

- Taylan Dogan (<doganntaylan@gmail.com>)

### Changes

#### Documentation

- **update changelog** ([`881e239`])

- **update readme** ([`5af591e`])

- **add more comments on store functions** ([`e0df818`])

- **add description for cli arg parser** ([`51e5fe5`])

- **add CHANGELOG and changelog generator config** ([`a0984d7`])

- **update readme** ([`66109be`])

  I've been keeping track of the issues/task from somewhere else. I really
  dont want to use GitHub's issue's and milestones.

- **update readme** ([`0b65bb8`])

- **add delete key explanation** ([`5f90cff`])

- **add macOS default config path in the doc** ([`5b4b9eb`])

- **add readme** ([`3aa305a`])

- **add doc comments** ([`33d9ea9`])

#### Miscellaneous Tasks

- **bump version** ([`801b148`])

- **update dependencies** ([`a6da04d`])

- **satisfy clippy** ([`b8742d7`])

- **update dependencies** ([`5cadb61`])

- **update dependencies** ([`45a7bbc`])

- **bump version** ([`d50d241`])

- **add more metadata** ([`813814d`])

- **rename project** ([`d30da13`])

- **add cargo metadata** ([`d47f2a9`])

- **add cargo lock file** ([`344cc85`])

- **add custom profiles and update dependencies** ([`a027791`])

#### Features

- **add notp type for errors and result** ([`5f0924e`])

- **read Key from stdin securely** ([`21b9cdb`])

- **add delete operation** ([`6032758`])

- **implement KV store** ([`1c60262`])

- **add --quiet option** ([`f8c53e7`])

  With -q option, topt will work with CLI aliases and functions.

- **parse cli commands** ([`094fd5f`])

- **add get** ([`4039af9`])

- **add get arg. for getting a key** ([`06d0d53`])

- **add mapify** ([`26e0ffa`])

- **add operations module** ([`0cf5a85`])

- **add getters for secret fields** ([`c7acfd9`])

- **try to add encryption [WIP]** ([`f8e1bb3`])

- **add main** ([`7d97db4`])

- **add otp** ([`580ccf4`])

- **add cli args** ([`a1d91bc`])

- **add utils** ([`da483d9`])

#### Code Styling

- **rustic way to implement if else** ([`e6f5986`])

- **fix --list output** ([`aefdf43`])

#### Bug Fixes

- **use unwrap_or_else** ([`d829436`])

- **make key param required only add and get** ([`703b50d`])

- **use unwrap_or_else instead of unwrap_or** ([`daf4d21`])

- **remove newline when get is quiet** ([`58b28f9`])

- **add folder creation if not exists** ([`2631aa9`])

- **satisfy clippy** ([`2216a8a`])

- **replace unwrap** ([`9a70ae3`])

- **use vector address instead of move operation on mapify** ([`f0b4fe2`])

- **decryption problem** ([`f3b9d11`])

- **otp generation issue** ([`673f4e4`])

#### Refactoring

- **put stdin reader function into utils** ([`45de5fb`])

- **change match with if-else** ([`90a3691`])

- **remove old imports and add `#[allow(unused)]`** ([`6f78a78`])

- **use &str instead of String** ([`2bdd63c`])

- **put mapify into its original file since it is not going to be generic** ([`5226ea7`])

- **change key field** ([`f1ab193`])

- **reading encrypted data** ([`f91fedc`])

#### Tests

- **add tests for mapify** ([`08df7b8`])

<!-- [releases] -->

[unreleased]: https://github.com/kondanta/notp/compare/v2.1.0...HEAD
[2.1.0]: https://github.com/kondanta/notp/releases/tag/v2.1.0
[2.0.0]: https://github.com/kondanta/notp/releases/tag/v2.0.0

<!-- [commits] -->

[`7c5f69c`]: https://github.com/kondanta/notp/commit/7c5f69caead4719859c92dc38fd97a73be354f0b
[`8f5193c`]: https://github.com/kondanta/notp/commit/8f5193cd66565d6585ff18383d38d07d70f8ef30
[`7883d83`]: https://github.com/kondanta/notp/commit/7883d8371c636ef57578f77d4f25544c1e284937
[`b7c2344`]: https://github.com/kondanta/notp/commit/b7c2344fe747e3961a7612042ba0c0025a01d6f8
[`be11b3a`]: https://github.com/kondanta/notp/commit/be11b3a37c57a47f6c2fe979e1b94ee8bae4c78e
[`5c1e759`]: https://github.com/kondanta/notp/commit/5c1e75986a2f1b2a531309ddbc59ccab40674f91
[`302e7e2`]: https://github.com/kondanta/notp/commit/302e7e2406076ece6663440b279cb21bfed1ffd3
[`f7545c5`]: https://github.com/kondanta/notp/commit/f7545c5d0b45af3b67aa43221ccb2a2f75fb0b4b
[`f501243`]: https://github.com/kondanta/notp/commit/f50124398507b72d2141381d7930d369f132d7b2
[`7da6c38`]: https://github.com/kondanta/notp/commit/7da6c38fa4034ebbadb2760ab41f676e92a3653f
[`6cbf99c`]: https://github.com/kondanta/notp/commit/6cbf99c7ae048f4a748e450f63a4c49a7416f339
[`c4f751a`]: https://github.com/kondanta/notp/commit/c4f751ae4ad6ee922b24fe56b8f9f8bfd245b339
[`366f414`]: https://github.com/kondanta/notp/commit/366f414ea98bd6fe78686b18a6e271c6b16a79c0
[`90a5890`]: https://github.com/kondanta/notp/commit/90a58904321d13b8f7e34ea4e100a9f645697331
[`b6fa738`]: https://github.com/kondanta/notp/commit/b6fa738f44274d5d481a683d714223618d4eca80
[`752c0f5`]: https://github.com/kondanta/notp/commit/752c0f5daee2bda66ff8ca33774bd3bd4aca2047
[`362fd9d`]: https://github.com/kondanta/notp/commit/362fd9d16325416981a4e50b58871a84baef18a9
[`cd67e72`]: https://github.com/kondanta/notp/commit/cd67e7211b9a2b578b013164815f47211025ef3d
[`aeffd8a`]: https://github.com/kondanta/notp/commit/aeffd8a131618ac509e0dd365ceebec35ace1122
[`4285a87`]: https://github.com/kondanta/notp/commit/4285a875dcfc29db312a79b788cc1c60bd62f016
[`a25a848`]: https://github.com/kondanta/notp/commit/a25a8480c07bac70b5c2782d51611518d6e14dfe
[`05b6a69`]: https://github.com/kondanta/notp/commit/05b6a6971254de35f282d190daa9a68b30e793d6
[`770f8d9`]: https://github.com/kondanta/notp/commit/770f8d9bd6b6243c9ed37b62cdeed49c1a22baf2
[`43d5b48`]: https://github.com/kondanta/notp/commit/43d5b48446f296526461f181fdaf8152e3a6f333
[`02b8bff`]: https://github.com/kondanta/notp/commit/02b8bffd60b8e7fb7a14d25a3e1f38dbe34510cb
[`4ebd0fa`]: https://github.com/kondanta/notp/commit/4ebd0fa8ad048c9becbb0937163120fd7cd48b40
[`e4afde7`]: https://github.com/kondanta/notp/commit/e4afde7f3b06b58f77039b5292248a3f7ee6458a
[`4e32465`]: https://github.com/kondanta/notp/commit/4e3246599ec24c11871f006c8301fc0eb2b1470f
[`b2f7836`]: https://github.com/kondanta/notp/commit/b2f7836768e9751c850d5570c17d6d92fba7ea02
[`6301e74`]: https://github.com/kondanta/notp/commit/6301e74dad1a100fec7ad3fb4fbd0108b75fa5e3
[`3070654`]: https://github.com/kondanta/notp/commit/30706544b16716e23a87134a40389c4e91d9eed8
[`f5db77c`]: https://github.com/kondanta/notp/commit/f5db77c61a3945d23ccd807ac978842d745fb0bd
[`8d9fd56`]: https://github.com/kondanta/notp/commit/8d9fd56788697798417cc26bbd63568a05cfdafb
[`b25fec2`]: https://github.com/kondanta/notp/commit/b25fec2cbd80baccafc5d4019f1505c3e24bc8f8
[`c3fba1e`]: https://github.com/kondanta/notp/commit/c3fba1e7bf84e861707af92170a5b0ba98e79f4e
[`37deb4c`]: https://github.com/kondanta/notp/commit/37deb4ce4fe71d5ee9926c4253673412c28e5c0b
[`605c918`]: https://github.com/kondanta/notp/commit/605c918010a4926957cd7da1919384a570dbae05
[`0870e12`]: https://github.com/kondanta/notp/commit/0870e129164f7ecce1d98693dc0b289a88b6a753
[`63839ba`]: https://github.com/kondanta/notp/commit/63839ba5c439eb2ed9fa2ae7b7f2f1b3a9ce9fc6
[`e5beb80`]: https://github.com/kondanta/notp/commit/e5beb800855fc97f2b564ec6637527b2c33b3dac
[`0cdb48a`]: https://github.com/kondanta/notp/commit/0cdb48a5b99761f4634d35d1310b70c95cb03c78
[`5a7f241`]: https://github.com/kondanta/notp/commit/5a7f2413c0786a56690b188e3ebec3e8927a1bc5
[`c7ac3eb`]: https://github.com/kondanta/notp/commit/c7ac3ebafa81c42b5861fd46db9fb371c0c107bf
[`d3f79ae`]: https://github.com/kondanta/notp/commit/d3f79ae8301b6b5fe2f6bfce45abe8efa18a8890
[`9b53d71`]: https://github.com/kondanta/notp/commit/9b53d71d70a4ede62f03637256e65f6e16436013
[`e706a20`]: https://github.com/kondanta/notp/commit/e706a20f6fcbd09bb1a94bcb87b1dbeada0f104b
[`9e1e599`]: https://github.com/kondanta/notp/commit/9e1e5992652361792423b69882106872ace43399
[`eda7350`]: https://github.com/kondanta/notp/commit/eda735027b77aec705a8bde03a0b92595a36254c
[`64dfddc`]: https://github.com/kondanta/notp/commit/64dfddc2807c3c21e721cd8e1f280b70594ea546
[`8f66a37`]: https://github.com/kondanta/notp/commit/8f66a373fb5c817b711b9a5ee960c3487d250ac2
[`3306395`]: https://github.com/kondanta/notp/commit/3306395e863b0c89b7fa172d8bdd2dbc04b78d5a
[`6ae9cca`]: https://github.com/kondanta/notp/commit/6ae9cca7620fa587b5e573894ced2c8550d26a31
[`53fc9d7`]: https://github.com/kondanta/notp/commit/53fc9d77167eaba91933380a754c732b178cc6ca
[`e585e57`]: https://github.com/kondanta/notp/commit/e585e57c5fe453384c7041fdab5dfc53f5f5e482
[`3703c2e`]: https://github.com/kondanta/notp/commit/3703c2ef24c960261fc9cd4632def51eb8df97ed
[`b80cfde`]: https://github.com/kondanta/notp/commit/b80cfded096dc612e0732b4bb2a220f5df260bd0
[`51b25d9`]: https://github.com/kondanta/notp/commit/51b25d94a53c18a797f939675f9724b088e37c30
[`922df8c`]: https://github.com/kondanta/notp/commit/922df8cc7472188408611a5116f911bfa3de68aa
[`f7f201f`]: https://github.com/kondanta/notp/commit/f7f201fb4d26c84c768ca604b0f69f877f5aaa73
[`9a53fd6`]: https://github.com/kondanta/notp/commit/9a53fd605fdbbc7438b2b591121474dab0063bfc
[`66b9baa`]: https://github.com/kondanta/notp/commit/66b9baa1d27f5ec1df0814b58edb0b9dc9a3e0db
[`cbf0ab4`]: https://github.com/kondanta/notp/commit/cbf0ab427615d405002cbedb44b002094094c914
[`adeb006`]: https://github.com/kondanta/notp/commit/adeb006ff8faea08b5da6361def3855bc4088c7d
[`f8860e6`]: https://github.com/kondanta/notp/commit/f8860e6cd0aed519db10f8c5c43dbc5a7cf57279
[`aeabfc4`]: https://github.com/kondanta/notp/commit/aeabfc470496ac65f2fec5936e769043daa0c45a
[`4133d25`]: https://github.com/kondanta/notp/commit/4133d25ccbf77695929c7ecd509bb88192aa3ff8
[`ecd22f6`]: https://github.com/kondanta/notp/commit/ecd22f629e97ca46343368ee733b999abe9149b2
[`fdfcf2a`]: https://github.com/kondanta/notp/commit/fdfcf2ac6cd1b4ab8bc1ad925e7c0ede6883949c
[`881e239`]: https://github.com/kondanta/notp/commit/881e2392b1c84769e7b484a3fd9095d010f0af7b
[`801b148`]: https://github.com/kondanta/notp/commit/801b14846f625c6e84389a4c7c0ddd4884b67416
[`5f0924e`]: https://github.com/kondanta/notp/commit/5f0924ec5782288fd06655d1875cd8c171e7038f
[`5af591e`]: https://github.com/kondanta/notp/commit/5af591e28ea4afd42b8496b104ffa19da3659de7
[`e0df818`]: https://github.com/kondanta/notp/commit/e0df818eaed4e54325a5a982a7488ae65e1bd2ba
[`51e5fe5`]: https://github.com/kondanta/notp/commit/51e5fe599e9e5d6298fc7c03a55874a11d9c28b0
[`e6f5986`]: https://github.com/kondanta/notp/commit/e6f5986688c1b22aa37f528809155bd15212697d
[`21b9cdb`]: https://github.com/kondanta/notp/commit/21b9cdb22d24666fed3b277c4c8cead2fac26dbe
[`a0984d7`]: https://github.com/kondanta/notp/commit/a0984d7f4dacf2f27914cb4e59c86e979f63c8d2
[`aefdf43`]: https://github.com/kondanta/notp/commit/aefdf43b07601d9e87fa490b94c8062c8d57cdba
[`66109be`]: https://github.com/kondanta/notp/commit/66109beee87ffa0d2e0677981f68821461010a76
[`d829436`]: https://github.com/kondanta/notp/commit/d829436627ab4c67d803687e2a6fe851c0ace9f5
[`703b50d`]: https://github.com/kondanta/notp/commit/703b50d7ef97b4c954285074885bb2d095798f2f
[`daf4d21`]: https://github.com/kondanta/notp/commit/daf4d21d85f9e2650a671cea56c5e7fd4fb473ee
[`a6da04d`]: https://github.com/kondanta/notp/commit/a6da04d8b55a7498224970022a00fd73e179db09
[`b8742d7`]: https://github.com/kondanta/notp/commit/b8742d755208acedcbe5dcfeb7700315cda4e8ad
[`6032758`]: https://github.com/kondanta/notp/commit/603275856f717f3c5b30ea09f126255a24015096
[`1c60262`]: https://github.com/kondanta/notp/commit/1c60262e873ce56f5fbb2fc93d36ea52722cb02f
[`45de5fb`]: https://github.com/kondanta/notp/commit/45de5fb8f30dbf9442bc055f085f250a71b6c14d
[`5cadb61`]: https://github.com/kondanta/notp/commit/5cadb610381b799fcf621245de69b47e33cf4a68
[`45a7bbc`]: https://github.com/kondanta/notp/commit/45a7bbc4391e751a6b7fb6b205b38fdb08737d69
[`90a3691`]: https://github.com/kondanta/notp/commit/90a3691f997ab38119821d88ac2962e978c513b4
[`58b28f9`]: https://github.com/kondanta/notp/commit/58b28f9e2fac649b5b5eeba08c36a18cecd33f93
[`d50d241`]: https://github.com/kondanta/notp/commit/d50d24184398ce613d7536905d8f0e9b018a0273
[`813814d`]: https://github.com/kondanta/notp/commit/813814dafe901113077f0ed0276f1cb651b5451f
[`d30da13`]: https://github.com/kondanta/notp/commit/d30da13ad8caa9b03aea56b1e5306e9b52985bbe
[`d47f2a9`]: https://github.com/kondanta/notp/commit/d47f2a93293b688372b4892a4489842a412e147b
[`0b65bb8`]: https://github.com/kondanta/notp/commit/0b65bb85749cde6ed7d432edeb932596a228192c
[`344cc85`]: https://github.com/kondanta/notp/commit/344cc85829429e7c54973ac01653364ae68ab8e7
[`f8c53e7`]: https://github.com/kondanta/notp/commit/f8c53e7901b932b5f353bc1731da2608e4f7679b
[`5f90cff`]: https://github.com/kondanta/notp/commit/5f90cff41ed2452d7f44ac3a2556a1fae5083cc2
[`5b4b9eb`]: https://github.com/kondanta/notp/commit/5b4b9eb461b6efa0ffacbbcc0142538f350ace93
[`2631aa9`]: https://github.com/kondanta/notp/commit/2631aa9a2454487d1613b780dc062ea472e4e771
[`a027791`]: https://github.com/kondanta/notp/commit/a0277918456577029fd772b9b58c53c611ec9b6d
[`2216a8a`]: https://github.com/kondanta/notp/commit/2216a8aef10c6d2b30b93141ab14899b142839f7
[`9a70ae3`]: https://github.com/kondanta/notp/commit/9a70ae33db7a99b1e93414c5553fc760110ea9ed
[`3aa305a`]: https://github.com/kondanta/notp/commit/3aa305a27a119a60619b5e4d625cb00269801fe6
[`33d9ea9`]: https://github.com/kondanta/notp/commit/33d9ea9f24975ed43af3beaa8d691040d6e85211
[`094fd5f`]: https://github.com/kondanta/notp/commit/094fd5f6c14f6313edcb609ff81ad5c9c2b3c955
[`6f78a78`]: https://github.com/kondanta/notp/commit/6f78a78f2f6df57c6b0585170cd12ccb57263821
[`f0b4fe2`]: https://github.com/kondanta/notp/commit/f0b4fe2545805e61b0ba6dad3889b6b9bf9ebcc2
[`4039af9`]: https://github.com/kondanta/notp/commit/4039af963fe48544ef088b0817da3ee00cc3bb79
[`2bdd63c`]: https://github.com/kondanta/notp/commit/2bdd63cf30a54a6b4e4b4cf4bff7e143dfda60a8
[`5226ea7`]: https://github.com/kondanta/notp/commit/5226ea7fda38b5fdf497e73687b4a46fcfb27429
[`06d0d53`]: https://github.com/kondanta/notp/commit/06d0d53d8f46a5a8cbcc82113a24da824d82ee7b
[`08df7b8`]: https://github.com/kondanta/notp/commit/08df7b8be53cbb3e1bfaf09c99cf0476a457a5fc
[`26e0ffa`]: https://github.com/kondanta/notp/commit/26e0ffa26819818ec9caa70a5807a46b95324705
[`f1ab193`]: https://github.com/kondanta/notp/commit/f1ab193e486596404341d5a19c308632e2908459
[`0cf5a85`]: https://github.com/kondanta/notp/commit/0cf5a853ac9605959afafb2efd6f7fb6240845df
[`c7acfd9`]: https://github.com/kondanta/notp/commit/c7acfd9e539ec3cb9508dca24dabfeccf2626456
[`f91fedc`]: https://github.com/kondanta/notp/commit/f91fedc192e9620c9e322795c4d29a0a14686a51
[`f3b9d11`]: https://github.com/kondanta/notp/commit/f3b9d11ab01e66602c614b8dd9725539b60481ec
[`f8e1bb3`]: https://github.com/kondanta/notp/commit/f8e1bb3790724dcb4b4442dfa27b7bf2de01679a
[`673f4e4`]: https://github.com/kondanta/notp/commit/673f4e4b5305fb7307688c3cdbf22fe4521b9b31
[`7d97db4`]: https://github.com/kondanta/notp/commit/7d97db4f3c5191c9c6e50ca37660dddcc951f08d
[`580ccf4`]: https://github.com/kondanta/notp/commit/580ccf4f4edbf824c75920182010acc1edd82024
[`a1d91bc`]: https://github.com/kondanta/notp/commit/a1d91bc0690e9b045d94b3c4a2748969bbf547f1
[`da483d9`]: https://github.com/kondanta/notp/commit/da483d940fa7af75b9a38ea4458783fa8b15fdd1

<!--
Config(
  github: ( repo: "kondanta/notp" ),
)
-->
