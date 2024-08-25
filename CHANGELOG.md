# Changelog

## [0.2.4](https://github.com/loiccoyle/strandify/compare/strandify-v0.2.3...strandify-v0.2.4) (2024-08-25)


### Bug Fixes

* error handling ([95b9ae9](https://github.com/loiccoyle/strandify/commit/95b9ae92139ea5de8160026dc684390c93ca22e1))

## [0.2.3](https://github.com/loiccoyle/strandify/compare/strandify-v0.2.2...strandify-v0.2.3) (2024-08-18)


### Bug Fixes

* improve `line_to` width and rm sorting ([737e345](https://github.com/loiccoyle/strandify/commit/737e345ee3353e67db808329f78740ed9dec4812))


### Performance Improvements

* **render_img:** render svg to image in parallel chunks ([714b204](https://github.com/loiccoyle/strandify/commit/714b2047d422aaf73778949778e334b89835a784))

## [0.2.2](https://github.com/loiccoyle/strandify/compare/strandify-v0.2.1...strandify-v0.2.2) (2024-08-17)


### Bug Fixes

* add spinner to render_img ([9fd4448](https://github.com/loiccoyle/strandify/commit/9fd444818010da0abbe403014e33328bd4420ff6))
* cleanup peg shapes by adding and using `line_coords` ([b551ee6](https://github.com/loiccoyle/strandify/commit/b551ee6d0ed1c4c3f23e3ee51c5ad8ac73e0e032))
* error handling ([067d651](https://github.com/loiccoyle/strandify/commit/067d651c2659472344e7bfdffd1f450b3abec29f))
* peg jitter not being applied ([f2d04aa](https://github.com/loiccoyle/strandify/commit/f2d04aa6bfeda765fd1a0a67c55ad7e7d62308ac))

## [0.2.1](https://github.com/loiccoyle/strandify/compare/strandify-v0.2.0...strandify-v0.2.1) (2024-08-16)


### Bug Fixes

* better handling of empty line cache ([c927fb5](https://github.com/loiccoyle/strandify/commit/c927fb5fde5373f672fc5f6a6031463a786f15f0))


### Performance Improvements

* beam search optimization ([43d3525](https://github.com/loiccoyle/strandify/commit/43d3525d208922c9a41b1057b207ba65abe31301))

## [0.2.0](https://github.com/loiccoyle/strandify/compare/strandify-v0.1.0...strandify-v0.2.0) (2024-08-15)


### Features

* add beam search algorithm ([f83e3e1](https://github.com/loiccoyle/strandify/commit/f83e3e1f4c34bb3cd1c0fae634ecbb94c7880283))
* add border pegs ([14eedf7](https://github.com/loiccoyle/strandify/commit/14eedf74e8d159034c66cb564cf90bf0c501161c))
* add early stopping ([6a6c6aa](https://github.com/loiccoyle/strandify/commit/6a6c6aa7cd0a5dca28f49d1b4ed0588f3c44e43e))
* add populating cache spinner ([7540b7e](https://github.com/loiccoyle/strandify/commit/7540b7efa82c57f3a527cf25368b3c81ec8ea8cd))
* add project_to_yarn_color flag ([af45a72](https://github.com/loiccoyle/strandify/commit/af45a7201b9e571c597be1f9bc1e68c423e9e059))
* add render scaling ([f4f08aa](https://github.com/loiccoyle/strandify/commit/f4f08aa58a398880001d0f0b4e8319e021e64538))
* look dor starting peg by min mean around pixel ([a160eb8](https://github.com/loiccoyle/strandify/commit/a160eb82a1ed3e7c4992ffeb773e6f41526abb6e))
* render imgs from svg, better line algorithm, yarn color projection ([a39f3d2](https://github.com/loiccoyle/strandify/commit/a39f3d286cd791dc2ae9fc664463ec8bc13961cf))
* use `tiny-skia`, add yarn color ([12abdaa](https://github.com/loiccoyle/strandify/commit/12abdaa58c1cb411274493ca633d1e6a5f00301f))
* use rayon to speed everything up ([2851a0e](https://github.com/loiccoyle/strandify/commit/2851a0e8208f91de0f98e1edff66471939b25fa2))


### Bug Fixes

* add default trait for Yarn ([f40c538](https://github.com/loiccoyle/strandify/commit/f40c53826f13d8e51697b6fd589a510e8408bc09))
* add render resolution log ([c8a92d5](https://github.com/loiccoyle/strandify/commit/c8a92d5c174c1754f7021f05eeb289a910784e2a))
* better docstring ([44dfc2d](https://github.com/loiccoyle/strandify/commit/44dfc2dbbf2da9c2812391fd7100c724360d2747))
* better jitter consistency ([6645b57](https://github.com/loiccoyle/strandify/commit/6645b578684d99dfd9b92f235234a84b0993e28a))
* better last peg assignments ([27cc459](https://github.com/loiccoyle/strandify/commit/27cc4597742353721a9e4cfab512e4bef5fda2f8))
* better pbar msg ([4b36c44](https://github.com/loiccoyle/strandify/commit/4b36c4469c94cd453a360107d26d7ef84946269f))
* better white image generation ([d1f1a45](https://github.com/loiccoyle/strandify/commit/d1f1a4551094e77564cf5026629800cb4423a5a2))
* cargo clippy ([ffd1e4f](https://github.com/loiccoyle/strandify/commit/ffd1e4f318a4595df8b3d4368d84a4fd156033a9))
* cleanup ([6c4083e](https://github.com/loiccoyle/strandify/commit/6c4083ee526a3d49fd969580499a1565dccfb309))
* cleanup and order line pixels for consistent tests ([5b9781b](https://github.com/loiccoyle/strandify/commit/5b9781b36afc52f178cc7ee79eae032dac239839))
* cleanup docstrings ([4c63ae9](https://github.com/loiccoyle/strandify/commit/4c63ae95e451a746356cb0b15587c763526b6ff1))
* duplicate names ([9e91dbb](https://github.com/loiccoyle/strandify/commit/9e91dbbdb62dec237639dbc7aaa738aedcfd7673))
* error handling and cleanup ([86dba9b](https://github.com/loiccoyle/strandify/commit/86dba9befa25a772bb76482ab593a30a642aeb0d))
* file cli argument checker failing on json files ([2900e9e](https://github.com/loiccoyle/strandify/commit/2900e9e676f055157b82b5ef8809d5b723801cfc))
* improve defaults ([f0954cc](https://github.com/loiccoyle/strandify/commit/f0954cccb71a1d3e6c396e30b73e0f606af83bed))
* knit method should take blueprint ([f1bf4d6](https://github.com/loiccoyle/strandify/commit/f1bf4d6601c47e28909ab0a1b3e971c833363d5d))
* minor optimization ([0716afa](https://github.com/loiccoyle/strandify/commit/0716afaaeaf0f274ba28ec5c260cc2707de75a0c))
* minor optimizations ([444dc3e](https://github.com/loiccoyle/strandify/commit/444dc3efd3a85cb39ab4dfc3d647c86e6fe27915))
* print jitter amount ([ea55444](https://github.com/loiccoyle/strandify/commit/ea554444734d5cc02bae81e285a346428586e3c3))
* refactor Peg.line_to method, other_peg -&gt; other ([95c5a62](https://github.com/loiccoyle/strandify/commit/95c5a629700d541cda340587a76d2397db6885d4))
* remove unecessary clone ([5d16331](https://github.com/loiccoyle/strandify/commit/5d163312586e8ac1083d668891b799d3de061062))
* remove unused arg ([5412e98](https://github.com/loiccoyle/strandify/commit/5412e98f29dd1ef62269ce566e32d062af279fbd))
* remove useless tee ([b431e9f](https://github.com/loiccoyle/strandify/commit/b431e9fdf8bc41a9e9b6665d75642525712bfc21))
* rename variable to blueprint ([845a8d2](https://github.com/loiccoyle/strandify/commit/845a8d26cc457598571f7ef4a9f01a73fd1177ec))
* rm double log ([d0b78c5](https://github.com/loiccoyle/strandify/commit/d0b78c5b6c0e0625581eeb40ee8f80f88d2d04bd))
* set transparent pixels to white when reading image ([0c86037](https://github.com/loiccoyle/strandify/commit/0c860377a1637022e23b9d23cc62174f4289d3ac))
* **tests:** add background ([ffb6232](https://github.com/loiccoyle/strandify/commit/ffb6232d77555fe761d2bdefcbac2e972c9006a3))
* tweak help msg ([04164a9](https://github.com/loiccoyle/strandify/commit/04164a95245e52c94b3692e18e1c86b0d2d59b70))
* use min and max to compute deltas ([a885df3](https://github.com/loiccoyle/strandify/commit/a885df32982538cb26a9566ed777a0de92c89c7a))
* use proper alpha compositing the lighen the work image ([0140387](https://github.com/loiccoyle/strandify/commit/0140387baa9543a49f5bb7985ac974d437ea1a47))
* yarn delta should never be 0 ([fec940d](https://github.com/loiccoyle/strandify/commit/fec940d07c55086df8965f38878d200275eea22a))
