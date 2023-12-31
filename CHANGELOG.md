# Changelog

## [0.8.0](https://github.com/nfejzic/alemat/compare/v0.7.0...v0.8.0) (2023-12-26)


### Features

* add constructor for the norm (`∥`) operator ([660a412](https://github.com/nfejzic/alemat/commit/660a4123d08ba6168dde3ec2bae6ecb3c407bda4))
* add more constructors for operator ([#33](https://github.com/nfejzic/alemat/issues/33)) ([71ab5ca](https://github.com/nfejzic/alemat/commit/71ab5cabb1b1866a140b591adc19ba8c417e3174))

## [0.7.0](https://github.com/nfejzic/alemat/compare/v0.6.1...v0.7.0) (2023-12-22)


### Features

* add possibility to map content of `MathMl` ([#29](https://github.com/nfejzic/alemat/issues/29)) ([91d35de](https://github.com/nfejzic/alemat/commit/91d35dec4e3adcd746875c27f826905859498230))

## [0.6.1](https://github.com/nfejzic/alemat/compare/v0.6.0...v0.6.1) (2023-12-14)


### Bug Fixes

* document the operator and ident constructors ([#24](https://github.com/nfejzic/alemat/issues/24)) ([329fa2c](https://github.com/nfejzic/alemat/commit/329fa2c94e30b918223418977f5fac151614d664))

## [0.6.0](https://github.com/nfejzic/alemat/compare/v0.5.1...v0.6.0) (2023-12-14)


### Features

* improve API of alemat ([#22](https://github.com/nfejzic/alemat/issues/22)) ([4e068a6](https://github.com/nfejzic/alemat/commit/4e068a6b96dfdae4ddcff34163abd068a3b4a7b3))

## [0.5.1](https://github.com/nfejzic/alemat/compare/v0.5.0...v0.5.1) (2023-12-12)


### Bug Fixes

* wrap elements of underover and subsup in row when many used ([#19](https://github.com/nfejzic/alemat/issues/19)) ([8355f4d](https://github.com/nfejzic/alemat/commit/8355f4d17172d54ab5f8276233686884e4aa56e2))

## [0.5.0](https://github.com/nfejzic/alemat/compare/v0.4.1...v0.5.0) (2023-12-10)


### Features

* implement `IntoElements` for `Elements` ([#17](https://github.com/nfejzic/alemat/issues/17)) ([9a61b9b](https://github.com/nfejzic/alemat/commit/9a61b9b8642b4dcd281e27452f978dfcd695fbcf))

## [0.4.1](https://github.com/nfejzic/alemat/compare/v0.4.0...v0.4.1) (2023-12-10)


### Bug Fixes

* constrain `Error` associated type with `std::error::Error` ([#15](https://github.com/nfejzic/alemat/issues/15)) ([2177ea8](https://github.com/nfejzic/alemat/commit/2177ea8da922371088156496f1cfe1e23a661ccd))

## [0.4.0](https://github.com/nfejzic/alemat/compare/v0.3.0...v0.4.0) (2023-12-08)


### Features

* improve write API ([#14](https://github.com/nfejzic/alemat/issues/14)) ([99eacaf](https://github.com/nfejzic/alemat/commit/99eacaf4ab51a19bdab7ba3b320b08039f5c5266))

## [0.3.0](https://github.com/nfejzic/alemat/compare/v0.2.0...v0.3.0) (2023-12-08)


### Features

* add `Tag` variants and helper macro for conversion ([6f215e6](https://github.com/nfejzic/alemat/commit/6f215e6e8bea22b5c7ba9406bd48086f21727166))
* add basic implementation for annotation element ([a1d7b46](https://github.com/nfejzic/alemat/commit/a1d7b4605ff526a0b59a67cfe91c35d12c480e25))
* add basic implementation for MathML global attributes ([727af4b](https://github.com/nfejzic/alemat/commit/727af4be1793089629d63a48e10a56c23424f814))
* add buffered writer rendering trait and impl ([2554668](https://github.com/nfejzic/alemat/commit/2554668abb8882e11e87032b583995eff747a916))
* add getter for content of MathMl ([d710039](https://github.com/nfejzic/alemat/commit/d710039c40f2fdcb098bbe6b23e5696e77a3ba2c))
* add helper function to add attributes to table ([d35ad91](https://github.com/nfejzic/alemat/commit/d35ad911764d185a625d6d8a1b7fd65b98434c2a))
* add helper Ident constructors ([f10b9c5](https://github.com/nfejzic/alemat/commit/f10b9c5dc00cca686dce085090e0f7a932fdf304))
* add operator helper constructors ([6108285](https://github.com/nfejzic/alemat/commit/61082856206d13d3cf98c99a017112a9b11b0d08))
* define `ToMathMl` trait ([2927ce5](https://github.com/nfejzic/alemat/commit/2927ce5560d8a617e283f596e6f8c6fb9a7f0a9f))
* derive useful traits for marker structs ([70477d8](https://github.com/nfejzic/alemat/commit/70477d81ac208b946ff72b40c426fb9f8c65bccd))
* further improve API of elements and builders ([a962235](https://github.com/nfejzic/alemat/commit/a962235986341fa577f1ee04b7ad489fb7ef7871))
* group elements as defined in MathMl spec ([1fcdf1e](https://github.com/nfejzic/alemat/commit/1fcdf1e4c66b8f0249bb063ca180d6e394436d94))
* impl `ToMathMl` for annotation and semantics ([b78fac4](https://github.com/nfejzic/alemat/commit/b78fac450bcea49118654194fee7cc135fd10fd8))
* impl builder for `msub`, `msup` and `msubsup` ([53729d2](https://github.com/nfejzic/alemat/commit/53729d279417477ae41c0b865ee2c954e3e18ec4))
* impl builder pattern for multiscripts ([5d04c0a](https://github.com/nfejzic/alemat/commit/5d04c0a187c8447326c7c0d121f1d4cc7f9672e5))
* impl from error for tag ([aa77c3c](https://github.com/nfejzic/alemat/commit/aa77c3cc44ddee04213e44bc02b89cfc38f5939b))
* impl from frac for tag ([339563e](https://github.com/nfejzic/alemat/commit/339563ebed9133358713fe949d837b09367e83ce))
* impl from ident for tag ([cb3ac83](https://github.com/nfejzic/alemat/commit/cb3ac83bedb7bd9b04f8b10f34db114113b38f75))
* impl from multiscripts for tag ([fa9d153](https://github.com/nfejzic/alemat/commit/fa9d1539a96a8ce448b654c3b5571f40f2c87a58))
* implement `Renderer` for `BufMathMlWriter` ([5188edb](https://github.com/nfejzic/alemat/commit/5188edb377ffa348dc96192336a7b6b847dae623))
* implement `ToMathMl` for `Attribute` ([0df3dfe](https://github.com/nfejzic/alemat/commit/0df3dfefc648c69a845921369fc6ea08b2a2fc8d))
* implement action to tag conversion ([7f2943f](https://github.com/nfejzic/alemat/commit/7f2943fa57d5d7a02013d1f10426326e8060a74d))
* implement annotation and semantics to tag conversion ([5c7f2d8](https://github.com/nfejzic/alemat/commit/5c7f2d844517b19ab1e7bde657b18bcb8c490880))
* implement attributes for `MathMl` ([c5a7865](https://github.com/nfejzic/alemat/commit/c5a7865cf3e73a379f91553d2e0fef42896ec202))
* implement basic types and their builders ([81bed89](https://github.com/nfejzic/alemat/commit/81bed89dd4fa4fb420ad1fb918305e90269afd70))
* implement basics of some elements ([9abf901](https://github.com/nfejzic/alemat/commit/9abf9017182648579e0a273315da84f706f0b13c))
* implement builder for `math` ([77d93b6](https://github.com/nfejzic/alemat/commit/77d93b6eed2148a07e056dea55731fb3368b541a))
* implement builder for `merror` ([bcdfdca](https://github.com/nfejzic/alemat/commit/bcdfdca156fb4ef5735170162083e690ff058a16))
* implement builder for `mi` element (`ident`) ([14c0ca6](https://github.com/nfejzic/alemat/commit/14c0ca6fb3fb7a7786e1fcdde289b738bf0d925e))
* implement builder for `munderover` element ([df564d0](https://github.com/nfejzic/alemat/commit/df564d0e2084146542a850d4ebaccdab517df989))
* implement builder for radicals ([c89aa73](https://github.com/nfejzic/alemat/commit/c89aa732775a2251b325caadb6536429679f5cad))
* implement builder pattern for `annotation` ([97a229f](https://github.com/nfejzic/alemat/commit/97a229fd9559fca836896974d74425337e037ffc))
* implement builder pattern for `mfrac` ([d17ea5f](https://github.com/nfejzic/alemat/commit/d17ea5f1b1d475c57675692ab620284da9d10069))
* implement builder pattern for `semantics` ([c0ce1b0](https://github.com/nfejzic/alemat/commit/c0ce1b09048ceed54a75e612ba0d0ba91be06f96))
* implement building of `padded` ([dfd02c0](https://github.com/nfejzic/alemat/commit/dfd02c03f41931c598c26f5548fde21b01223881))
* implement building of `phantom` ([81d3baa](https://github.com/nfejzic/alemat/commit/81d3baad4dca448027d1f97d41b4c9e992ec005c))
* implement building patterns for mn, mo ([fc35c0d](https://github.com/nfejzic/alemat/commit/fc35c0d4299591ffd44a009d1b165925cfed3513))
* implement more elements ([0569f4a](https://github.com/nfejzic/alemat/commit/0569f4a72b0a863e27e2fd26fe1548ab13b887b8))
* implement more elements ([ecfe9ec](https://github.com/nfejzic/alemat/commit/ecfe9eca5e4e454ec5cbb14171e240c11acc628d))
* implement rendering with visitor pattern ([5586407](https://github.com/nfejzic/alemat/commit/5586407e03bbd55adb6b19f80e91f14a0770f4e2))
* implement rest of the conversions to `Tag` ([d3d95e5](https://github.com/nfejzic/alemat/commit/d3d95e5ebda98149d0ff9e718987f2895b4cb2e2))
* implement the rest of elements ([305ad42](https://github.com/nfejzic/alemat/commit/305ad42a4e14c7aa377f936f972140f42451537b))
* improve API of `mrow` ([6cb213e](https://github.com/nfejzic/alemat/commit/6cb213e1fa015744c556065458426b7b66e6c4c8))
* improve API of `ms`, `mspace` and `mstyle` ([b717979](https://github.com/nfejzic/alemat/commit/b717979a89d07cca0b8700a5f3cf34012107870e))
* improve API of `Table`, `TableRow` and `TableCell` ([a1aaf83](https://github.com/nfejzic/alemat/commit/a1aaf83a43a5542ce85634c22a2b116238fc81fc))
* improve API of Text element ([f55998e](https://github.com/nfejzic/alemat/commit/f55998e515ec901240cba4272b16ca3a48fa34fe))
* improve docs and impl builder for `maction` ([6da8df0](https://github.com/nfejzic/alemat/commit/6da8df0febbdebe9103a9dbf87e2923e251d2b12))
* init repository ([23e08af](https://github.com/nfejzic/alemat/commit/23e08af49ea3b3c6cbb56464ef5bee74cb398b7a))
* make API more ergonomic ([46dcae6](https://github.com/nfejzic/alemat/commit/46dcae6f4a5b7814ff5a175bb7e7a04fb22a21cb))
* mutably borrow renderer in `MathMlRenderer` trait ([ee9d7d9](https://github.com/nfejzic/alemat/commit/ee9d7d948e0364e38ffa9f245e122d45eefa9a73))
* prepare rendering of `MathMl` ([605b0ad](https://github.com/nfejzic/alemat/commit/605b0ad3d5b4e012dd15d52798ec6b13f28d17b5))
* provide more helper macros and improve API ([7edd11f](https://github.com/nfejzic/alemat/commit/7edd11fa15cdcffc4491eb20789ca0bb69142274))


### Bug Fixes

* borrow immutable when mutable not needed ([9311883](https://github.com/nfejzic/alemat/commit/93118835c1c0d9c2366e89f2f9e42ab6e54d14e9))
* correctly set sub and sup in `SubSupBuilder` ([9c13518](https://github.com/nfejzic/alemat/commit/9c13518ed2d573d9d9593d9a0185856498a25343))
* generalize `from_types` macro ([e917f5d](https://github.com/nfejzic/alemat/commit/e917f5d1ab357e501d4472f095b966cdf4cd83b4))
* ignore html and javascript files used for testing ([3bb0605](https://github.com/nfejzic/alemat/commit/3bb0605500b7a5bcc8f9126b7c3a1f96f663dc58))
* improve API consistency ([39e305f](https://github.com/nfejzic/alemat/commit/39e305ff1699e5356b2a5765ab31ef8901efec9a))
* improve buffered rendering ([4dc8fbc](https://github.com/nfejzic/alemat/commit/4dc8fbc2f7f6be59ad1499e0bae14cc6120894fa))
* improve tag_from_type macro ([323be61](https://github.com/nfejzic/alemat/commit/323be61aa81931eec82417f2a1c2189814f9de06))
* indicate fallible rendering operations ([c27c8f1](https://github.com/nfejzic/alemat/commit/c27c8f13b1ae1ea4fb45b9ddd13149622e4ef226))
* lint clippy.nursery and fix lint warnings ([5c87016](https://github.com/nfejzic/alemat/commit/5c8701634a6131566583c254075c5b94b42abbbe))
* rename `MathMl::render` to `MathMl::render_with` ([32098ff](https://github.com/nfejzic/alemat/commit/32098ff81d46023c3708cebfe4c903e4d2228810))
* rename trait `MathMlRenderer` to `Render` ([0a9d6c4](https://github.com/nfejzic/alemat/commit/0a9d6c487e77f85d79d53911ba7313046326f874))
* use buffered rendering per default ([7c5b8fe](https://github.com/nfejzic/alemat/commit/7c5b8fe0aa67fe840c9bb844132d7195dd6f4d50))
