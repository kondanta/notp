<a name="2.0.0"></a>
## 2.0.0 You Can (Not) Advance (2020-05-12)


#### Bug Fixes

*   use unwrap_or_else ([d8294366](https://github.com/kondanta/notp/commit/d829436627ab4c67d803687e2a6fe851c0ace9f5))
* **cli:**  make key param required only add and get ([703b50d7](https://github.com/kondanta/notp/commit/703b50d7ef97b4c954285074885bb2d095798f2f))
* **ops:**  use unwrap_or_else instead of unwrap_or ([daf4d21d](https://github.com/kondanta/notp/commit/daf4d21d85f9e2650a671cea56c5e7fd4fb473ee))

#### Breaking Changes

* **store:**  implement KV store ([1c60262e](https://github.com/kondanta/notp/commit/1c60262e873ce56f5fbb2fc93d36ea52722cb02f), breaks [#](https://github.com/kondanta/notp/issues/))

#### Feature

* **cli:**  read Key from stdin securely ([21b9cdb2](https://github.com/kondanta/notp/commit/21b9cdb22d24666fed3b277c4c8cead2fac26dbe))
* **error:**  add notp type for errors and result ([5f0924ec](https://github.com/kondanta/notp/commit/5f0924ec5782288fd06655d1875cd8c171e7038f))
* **ops:**  add delete operation ([60327585](https://github.com/kondanta/notp/commit/603275856f717f3c5b30ea09f126255a24015096))
* **store:**  implement KV store ([1c60262e](https://github.com/kondanta/notp/commit/1c60262e873ce56f5fbb2fc93d36ea52722cb02f), breaks [#](https://github.com/kondanta/notp/issues/))

#### Refactor

* **ops:**  change match with if-else ([90a3691f](https://github.com/kondanta/notp/commit/90a3691f997ab38119821d88ac2962e978c513b4))
* **util:**  put stdin reader function into utils ([45de5fb8](https://github.com/kondanta/notp/commit/45de5fb8f30dbf9442bc055f085f250a71b6c14d))

#### Style

*   rustic way to implement if else ([e6f59866](https://github.com/kondanta/notp/commit/e6f5986688c1b22aa37f528809155bd15212697d))
* **store:**  fix --list output ([aefdf43b](https://github.com/kondanta/notp/commit/aefdf43b07601d9e87fa490b94c8062c8d57cdba))

#### Documents

*   add CHANGELOG and changelog generator config ([a0984d7f](https://github.com/kondanta/notp/commit/a0984d7f4dacf2f27914cb4e59c86e979f63c8d2))
*   update readme ([66109bee](https://github.com/kondanta/notp/commit/66109beee87ffa0d2e0677981f68821461010a76))
* **cli:**  add description for cli arg parser ([51e5fe59](https://github.com/kondanta/notp/commit/51e5fe599e9e5d6298fc7c03a55874a11d9c28b0))
* **readme:**  update readme ([5af591e2](https://github.com/kondanta/notp/commit/5af591e28ea4afd42b8496b104ffa19da3659de7))
* **store:**  add more comments on store functions ([e0df818e](https://github.com/kondanta/notp/commit/e0df818eaed4e54325a5a982a7488ae65e1bd2ba))



<a name="1.1.2"></a>
## 1.1.2 Transcending into the CLI (2020-05-12)


#### Refactor

*   remove old imports and add `#[allow(unused)]` ([6f78a78f](https://github.com/kondanta/notp/commit/6f78a78f2f6df57c6b0585170cd12ccb57263821))
*   reading encrypted data ([f91fedc1](https://github.com/kondanta/notp/commit/f91fedc192e9620c9e322795c4d29a0a14686a51))
* **crypt:**  change key field ([f1ab193e](https://github.com/kondanta/notp/commit/f1ab193e486596404341d5a19c308632e2908459))
* **ops:**  use &str instead of String ([2bdd63cf](https://github.com/kondanta/notp/commit/2bdd63cf30a54a6b4e4b4cf4bff7e143dfda60a8))
* **util:**  put mapify into its original file since it is not going to be generic ([5226ea7f](https://github.com/kondanta/notp/commit/5226ea7fda38b5fdf497e73687b4a46fcfb27429))

#### Features

*   add getters for secret fields ([c7acfd9e](https://github.com/kondanta/notp/commit/c7acfd9e539ec3cb9508dca24dabfeccf2626456))
*   try to add encryption [WIP] ([f8e1bb37](https://github.com/kondanta/notp/commit/f8e1bb3790724dcb4b4442dfa27b7bf2de01679a))
*   add main ([7d97db4f](https://github.com/kondanta/notp/commit/7d97db4f3c5191c9c6e50ca37660dddcc951f08d))
*   add otp ([580ccf4f](https://github.com/kondanta/notp/commit/580ccf4f4edbf824c75920182010acc1edd82024))
*   add cli args ([a1d91bc0](https://github.com/kondanta/notp/commit/a1d91bc0690e9b045d94b3c4a2748969bbf547f1))
*   add utils ([da483d94](https://github.com/kondanta/notp/commit/da483d940fa7af75b9a38ea4458783fa8b15fdd1))
* **cli:**
  *  add --quiet option ([f8c53e79](https://github.com/kondanta/notp/commit/f8c53e7901b932b5f353bc1731da2608e4f7679b))
  *  parse cli commands ([094fd5f6](https://github.com/kondanta/notp/commit/094fd5f6c14f6313edcb609ff81ad5c9c2b3c955))
  *  add get arg. for getting a key ([06d0d53d](https://github.com/kondanta/notp/commit/06d0d53d8f46a5a8cbcc82113a24da824d82ee7b))
* **ops:**
  *  add get ([4039af96](https://github.com/kondanta/notp/commit/4039af963fe48544ef088b0817da3ee00cc3bb79))
  *  add operations module ([0cf5a853](https://github.com/kondanta/notp/commit/0cf5a853ac9605959afafb2efd6f7fb6240845df))
* **util:**  add mapify ([26e0ffa2](https://github.com/kondanta/notp/commit/26e0ffa26819818ec9caa70a5807a46b95324705))

#### Documents

*   add readme ([3aa305a2](https://github.com/kondanta/notp/commit/3aa305a27a119a60619b5e4d625cb00269801fe6))
*   add doc comments ([33d9ea9f](https://github.com/kondanta/notp/commit/33d9ea9f24975ed43af3beaa8d691040d6e85211))
* **readme:**
  *  update readme ([0b65bb85](https://github.com/kondanta/notp/commit/0b65bb85749cde6ed7d432edeb932596a228192c))
  *  add delete key explanation ([5f90cff4](https://github.com/kondanta/notp/commit/5f90cff41ed2452d7f44ac3a2556a1fae5083cc2))
  *  add macOS default config path in the doc ([5b4b9eb4](https://github.com/kondanta/notp/commit/5b4b9eb461b6efa0ffacbbcc0142538f350ace93))

#### Test

* **util:**  add tests for mapify ([08df7b8b](https://github.com/kondanta/notp/commit/08df7b8be53cbb3e1bfaf09c99cf0476a457a5fc))

#### Fix

*   remove newline when get is quiet ([58b28f9e](https://github.com/kondanta/notp/commit/58b28f9e2fac649b5b5eeba08c36a18cecd33f93))
*   satisfy clippy ([2216a8ae](https://github.com/kondanta/notp/commit/2216a8aef10c6d2b30b93141ab14899b142839f7))
*   decryption problem ([f3b9d11a](https://github.com/kondanta/notp/commit/f3b9d11ab01e66602c614b8dd9725539b60481ec))
*   otp generation issue ([673f4e4b](https://github.com/kondanta/notp/commit/673f4e4b5305fb7307688c3cdbf22fe4521b9b31))
* **ops:**  use vector address instead of move operation on mapify ([f0b4fe25](https://github.com/kondanta/notp/commit/f0b4fe2545805e61b0ba6dad3889b6b9bf9ebcc2))
* **util:**
  *  add folder creation if not exists ([2631aa9a](https://github.com/kondanta/notp/commit/2631aa9a2454487d1613b780dc062ea472e4e771))
  *  replace unwrap ([9a70ae33](https://github.com/kondanta/notp/commit/9a70ae33db7a99b1e93414c5553fc760110ea9ed))
