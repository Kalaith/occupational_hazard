# Character artwork

Generated with the built-in image_gen tool on 2026-09-04. The selected outputs
are copied into the project and loaded by macroquad-toolkit's AssetManager.
They are included in asset_registry.json for Windows and WebGL packaging.

## Mira Ashford

File: `assets/portraits/mira.png`

Prompt summary:

> Use case: stylized-concept. Asset type: fantasy management game character portrait. Create a polished painterly storybook portrait of a young adult female human adventurer, Mira Ashford, practical chestnut short hair, warm tan skin, green travel cloak over worn iron breastplate, sword hilt and battered round shield visible, earnest capable expression. Waist-up centered, head fully visible, square composition. Warm guild lamplight, muted teal and ochre palette, textured brushwork, rich readable silhouette. Background simple dark teal guild interior, no text, no lettering, no watermark. This is game artwork for Occupational Hazard, cozy fantasy with real stakes.

## Elowen

File: `assets/portraits/elowen.png`

Prompt summary:

> Use case: stylized-concept. Asset type: fantasy management game receptionist portrait. Polished painterly storybook portrait of an adult female human guild receptionist named Elowen, dark curly hair pinned into a bun, round brass spectacles, warm brown skin, cream blouse and burgundy waistcoat, ink-stained fingers holding a quill over a ledger at a wooden desk. Friendly shrewd smile. Waist-up centered square composition, head fully visible. Warm guild lamplight, muted teal and ochre palette, textured brushwork, readable silhouette, simple dark teal shelves background. No text, lettering or watermark. Artwork for Occupational Hazard, cozy fantasy bureaucracy.

All three adventurers now have painted dossier portraits. The catalog thumbnail is a
capture of the actual title screen featuring Mira and Elowen.

## Tomas Reed

File: `assets/portraits/tomas.png`

Final prompt (built-in image_gen):

> Use case: stylized-concept. Asset type: fantasy management game character portrait. Create a polished painterly storybook portrait of Tomas Reed, an adult male human ranger with sandy brown hair tied loosely back, light weathered skin, short beard, observant hazel eyes and a cautious thoughtful expression. Practical moss-green hooded cloak with hood down, worn brown leather armour, bow and a small quiver visible, one hand holding a folded woodland map. Waist-up centered square composition, head fully visible. Warm guild lamplight, muted teal and ochre palette, textured brushwork, rich readable silhouette. Simple dark teal guild interior background. No text, lettering or watermark. Match the cozy fantasy, grounded painterly portrait aesthetic of Occupational Hazard's Mira Ashford and receptionist Elowen.

## Pip Fenwick

File: `assets/portraits/pip.png`

Final prompt (built-in image_gen):

> Use case: stylized-concept. Asset type: fantasy management game character portrait. Create a polished painterly storybook portrait of Pip Fenwick, a young adult male human healer with tousled copper curls, freckles, kind green eyes and a dependable reassuring smile. Cream linen tunic, muted blue-teal travelling vest and short wool shoulder cape, practical healer's satchel with clean rolled bandages and herbal sprigs. Hands gently holding a small ceramic medicine jar; a simple wooden sun-shaped pendant at his collar. Waist-up centered square composition, head fully visible. Warm guild lamplight, muted teal and ochre palette, textured brushwork, rich readable silhouette. Simple dark teal guild interior background. No text, lettering or watermark. Match the cozy fantasy, grounded painterly portrait aesthetic of Occupational Hazard's Mira Ashford and receptionist Elowen.

## Cutaway assets (2026-09-09)

All five new assets use the built-in imagegen tool, with project copies in
`assets/headquarters/`. No external image API or API key was used. Existing
portraits define identities; supplied mockups define architectural composition,
lighting and slate/timber/copper materials. The catalog now captures the building
on the actual title screen.

- `building.png` (1536×1024): empty two-storey cutaway, recovery/records upstairs,
  common/assignments downstairs, ground gate and bare courtyard. Prompt summary:
  "NO UI, NO text, NO people, NO animals. Warm amber lantern-lit timber and stone
  interiors, cool blue dusk mountains and distant castle. Clear full room
  interiors, front-facing side elevation; empty foreground for separate sprites."
- `people-keyed.png` (1536×1024): Mira, Tomas, Pip and Elowen standing in the first
  row; seated recovery poses and Elowen's ledger pose in the second. Final edit
  prompt: "Change ONLY background to perfectly uniform flat pure saturated magenta
  #FF00FF everywhere between and behind these 8 isolated character sprites.
  Preserve character appearances, poses and items. No checkerboard, texture,
  gradient, halo or shadows."
- `activity-keyed.png` (1536×1024): three walking figures with packs, then sword,
  bow and bandage-practice poses. Prompt summary: "SAME first three characters,
  preserving faces, genders, hair and costume. THREE equal columns TWO rows.
  Top row Mira, Tomas and Pip walking right with packs; bottom row practising
  sword guard, drawing bow, and bandaging forearm. Flat magenta #FF00FF."
- `facilities-keyed.png` (1536×1024): dummies, medical trolley, targets/rack and
  medical cabinet. Prompt summary: "FOUR separate objects in 2×2 grid, generous
  margins, front-facing side elevation, painterly realistic aged timber and warm
  amber lighting. No people, text, floor or shadows. Flat magenta #FF00FF."
- `route.png` (1536×1024): a generic route vignette, not a literal map of every
  job. Prompt summary: "Medieval woodland path past three mossy boundary stones
  towards a distant stone bridge, village and hilltop castle at blue dusk. Warm
  lantern, slate blue mountains and copper sunset. Compose for a horizontal crop."

The initial transparency attempts contained painted backgrounds, not alpha.
Final source atlases retain magenta. Toolkit `AssetManager::load_texture_keyed`
removes it once at loading and preserves genuine existing alpha. Source art is
never decoded or reloaded during interaction. `scene.rs` records exact atlas
crops, feet anchors and display scales. Figures precede labels; upgrade props
appear only after purchase. All paths are in the texture manifest and registry.


## Owner-requested artwork replacement (2026-09-09)

The owner rejected the earlier room/figure presentation as placeholder quality.
Three replacement images were generated with the built-in imagegen tool and
copied into the registered project paths. Original generations remain under
C:/Users/Kalai/.codex/generated_images/01a08353-17cb-77f3-ba07-5ae283708fd4/.

- building.png: exec-200d9a9b-c797-4f02-ba48-4f8939f3edc9.png. Edit retained the
  exact architecture and floor geometry. Prompt specified three distinct recovery
  beds with linen and bandages, a working records library, furnished common room,
  posted commissions and maps, travel supplies at the gate and an empty courtyard
  until equipment is purchased. No people or interface were baked into the art.
- people-keyed.png: exec-02c159f4-814c-4c22-8697-4344400a2340.png. Four columns,
  two rows. Upper row: Mira checks her satchel, Tomas adjusts travel equipment,
  Pip checks medicine and Elowen works seated at her ledger. Lower row: the three
  adventurers rest on stools; Elowen reads a scroll. Prompt requested natural
  adult proportions, matte worn clothing and warm room lighting rather than
  frontal hero poses. Full figures are preserved in explicit atlas crops.
- activity-keyed.png: exec-f2b692fd-2d06-405a-90a5-10a4bb815782.png. Three columns,
  two rows. Side-facing travel and sword/bow/bandage practice match the replacement
  people atlas. Prompt retained all weapons and feet inside their cells, natural
  adult proportions, worn clothing and a flat magenta key with no floor.

Room figures now occupy about 19% of the viewport height when standing and 15%
when resting, with floor contact shadows. Elowen occupies 18%. Travel figures use
19%. Original portraits, route vignette and purchased equipment remain authored
assets; no geometric stand-in figures are used. The earlier atlas pose descriptions
above refer to the superseded versions preserved in git history.

The replacements ship as building-v2.png, people-v2-keyed.png and activity-v2-keyed.png so browser caches cannot retain the superseded art. The texture manifest and asset registry use these versioned paths.

## Feedback-driven destination and furniture art

Built-in imagegen produced two 1536x1024 atlases. No external API was used.
Source originals remain in the same generated_images thread directory as above.

- destinations.png: exec-1fd06c50-f7d4-4032-aa9f-00cdf6a27394.png. Three columns,
  four rows of distinct 2:1 destination paintings in contract order (cellar,
  medicine marsh, beekeeper apiary, well, lantern trial, bridge, shutters, quarry,
  boundary stones, shepherd, millhouse fever, watchtower bandages). Prompt required
  matching matte dusk environments, no text, independent compositions in each cell.
  The renderer maps stable contract IDs to cells and excludes divider pixels.
- rest-beds-keyed.png: exec-cdfdd26c-4b7c-4df5-8e6b-4e406ffcbefb.png. Three columns,
  two rows. Upper row: Mira, Tomas and Pip reclining inside occupied oak beds with
  pillows and green blankets, including naturally overlapping footboards. Lower
  row contains matching bedside table, desk and archery target props (reserved,
  not presented as new gameplay). Prompt used the building and people atlases as
  references and explicitly forbade perching beside beds, armor, stools and weapons.
  Only the three occupied beds are integrated; existing authored foreground surfaces
  provide additional occlusion in the scene.

- people-v3-keyed.png: exec-0549ab41-c51b-4f40-8680-26fa86cd8fa6.png.
- activity-v3-keyed.png: exec-4d6095ce-429e-4dcd-9a3d-ae3ac397af5e.png.

Both final edits retained atlas dimensions, silhouettes and poses, but requested
soft blended shading, broad matte brushwork, reduced microcontrast and saturation,
removal of etched clothing/hair details and studio highlights. The room painting
provided the lighting reference. Flat magenta keys remain untouched. Versioned
paths ensure the softened figures replace cached assets in existing browsers.
