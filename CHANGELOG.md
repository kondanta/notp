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
