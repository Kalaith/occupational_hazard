# Occupational Hazard
## Game Design Document

### High Concept

**Occupational Hazard** is a fantasy management simulation where the player works behind the front desk of an Adventurers' Guild.

Heroes may fight the monsters, explore forgotten ruins, and rescue villagers, but somebody has to decide **which heroes get sent to which quests**.

The player receives contracts, evaluates their danger, assigns adventurers or parties, issues rewards, approves promotions, and keeps the guild branch productive.

Adventurers grow from inexperienced recruits into valuable veterans. Sending rookies on dangerous assignments may accelerate their careers, but injuries and deaths can leave the guild unable to complete difficult contracts or meet its quotas.

The player never directly controls an expedition. They make the decision and then live with the result.

---

# Design Pillars

## 1. People, Not Units

Adventurers should gradually become recognisable individuals.

The player should remember:

- who they recruited
- their first successful quest
- who they usually adventure with
- injuries and failures
- promotions they earned
- times the player took a chance on them
- veterans they trained under
- friends or party members they lost

Losing an experienced adventurer should hurt because the guild has lost both a valuable resource and somebody the player knows.

---

## 2. Risk Without Perfect Information

The player should rarely know exactly what will happen.

Avoid presenting decisions as:

> Success Chance: 74%

Instead provide information such as:

**Quest Danger:** Moderate  
**Recommended Rank:** D  
**Known Threat:** Goblins  
**Location:** Old Mill  
**Previous Reports:** None

An experienced ranger might recognise evidence suggesting something worse is happening.

Information becomes better as the guild and its adventurers gain experience.

---

## 3. Today's Safe Decision Can Create Tomorrow's Problem

Veterans are reliable.

Sending them everywhere is therefore tempting.

But constantly relying on veterans means rookies don't develop.

Eventually the guild might have:

- two excellent veterans
- twelve inexperienced adventurers
- several injured members
- multiple high-rank contracts

The player must continually balance **current success against developing future capability**.

---

## 4. The Player Stays Behind the Desk

The player does not control combat.

The interesting moment is:

> "Do I send these people?"

Once an adventurer leaves, the decision has been made.

The player eventually learns what happened through returning adventurers, reports, injuries, rewards, rumours, or someone simply failing to return.

---

# Core Gameplay Loop

Each game day follows approximately:

### 1. Open the Guild

New quests arrive.

Adventurers become available.

Existing expeditions may return.

Events and messages may appear.

---

### 2. Process Returning Adventurers

Resolve completed expeditions.

Possible outcomes include:

- overwhelming success
- success
- partial success
- retreat
- failure
- injury
- serious injury
- missing adventurer
- death
- unexpected discovery

Rewards and experience are distributed.

The player may learn new information about regions, enemies or quest types.

---

### 3. Review New Contracts

Clients submit requests.

Example:

**Something in the Well**

Client: Marlow Village  
Requested Rank: E  
Reward: 38 gold  
Deadline: 3 days

> Villagers report noises coming from the old well and several missing chickens.

The information supplied by the client is not necessarily accurate.

---

### 4. Assign Adventurers

The player chooses an individual or party.

Factors include:

- rank
- experience
- class
- skills
- equipment
- personality
- fatigue
- injuries
- party relationships
- knowledge of the enemy
- knowledge of the region

The player may decide not to accept a quest.

---

### 5. Dispatch

The adventurers leave.

The quest becomes an active expedition.

Different quests take different amounts of time.

A local rat problem may resolve that afternoon.

An expedition into ancient ruins could take several days.

---

### 6. Administration

The player handles:

- promotions
- recruitment
- rewards
- guild finances
- adventurer records
- equipment requests
- complaints
- disciplinary issues
- head-office requirements

Then the guild closes and another day begins.

---

# Adventurers

Every adventurer is a persistent character.

## Basic Attributes

Each adventurer has:

- Name
- Portrait
- Age
- Rank
- Class
- Experience
- Equipment
- Health
- Fatigue
- Traits
- Skills
- Quest history
- Relationships
- Reputation

Example:

### Mira Ashford

**Rank:** D  
**Class:** Fighter  
**Experience:** Experienced

Traits:

- Cautious
- Dependable
- Protective

Skills:

- Swordsmanship III
- Shield II
- Goblin Lore I

Record:

- 27 successful quests
- 3 failed quests
- 2 injuries
- 0 abandoned quests

---

# Adventurer Ranks

Initial ranks could be:

**F Ã¢â€ â€™ E Ã¢â€ â€™ D Ã¢â€ â€™ C Ã¢â€ â€™ B Ã¢â€ â€™ A Ã¢â€ â€™ S**

Rank represents professional certification rather than simply character level.

A Rank C adventurer is someone the guild considers capable of accepting Rank C contracts.

---

# Promotions

One of the player's major responsibilities is deciding when adventurers should be promoted.

Promotion is **not automatic**.

An adventurer becomes eligible after meeting requirements such as:

- experience
- completed quests
- recommendations
- required skills
- successful difficult assignments

The player then approves or denies the promotion.

### Promoting Early

Benefits:

- access to harder quests
- increased adventurer confidence
- faster career development
- greater guild capacity

Risks:

- adventurer may be underprepared
- increased injury/death risk
- reputation damage after failures

### Holding Someone Back

Benefits:

- additional experience
- safer development
- stronger preparation

Risks:

- frustration
- morale loss
- adventurer may leave
- harder guild quotas become difficult to fulfil

An adventurer might confront the player:

> "I've completed seventeen E-rank assignments. How many more do I need before you believe I'm ready?"

The player may genuinely have good reasons for refusing.

---

# Quest System

Quests are generated from structured components rather than requiring every quest to be handcrafted.

A quest contains:

- requester
- location
- objective
- estimated danger
- actual danger
- enemies
- duration
- deadline
- reward
- special conditions
- hidden complications

Example:

### Missing Livestock

**Displayed information**

Rank E  
Reward: 25g  
Expected Duration: Half Day

Several sheep disappeared overnight.

**Hidden simulation**

Cause: Wolves  
Number: 4  
Actual difficulty: E

Another generated version might secretly contain:

Cause: Werewolf  
Actual difficulty: C

The receptionist only knows what has been reported.

---

# Quest Categories

Examples include:

- monster extermination
- escort
- rescue
- investigation
- gathering
- exploration
- delivery
- bounty
- missing person
- dungeon expedition
- defence
- recovery of artifacts

Different classes and traits perform better in different situations.

A fighter may be excellent at killing monsters but poor at locating a missing traveller.

---

# Quest Assessment

The guild assigns an estimated rank to quests.

Early estimates can be unreliable.

As the guild develops:

- better scouts become available
- adventurers provide intelligence
- monster knowledge improves
- regional knowledge increases
- previous quests reveal patterns

A suspicious Rank E contract might therefore display:

> **Guild Assessment: E**  
> Ranger Assessment: Possibly D  
> Known Information: Incomplete

The final decision belongs to the player.

---

# Expedition Resolution

Expeditions are simulated rather than played.

The simulation considers:

- adventurer capabilities
- party composition
- quest requirements
- enemy strength
- equipment
- injuries
- fatigue
- personality
- relationships
- random events

The simulation produces a narrative result.

Example:

### Old Mill Investigation

**Result: SUCCESS**

Mira discovered that the reported goblins were occupying tunnels beneath the mill.

The party attempted to clear the tunnels.

Tomas was injured protecting Pip.

The goblins retreated.

**Rewards**

Guild: +48g  
Reputation: +3

**Adventurers**

Mira: +Experience  
Tomas: +Experience, Injured  
Pip: +Experience

**New Information**

Old Mill Tunnels discovered.

---

# Death

Death should be possible but relatively uncommon when adventurers are assigned appropriately.

Danger increases dramatically when:

- under-ranked
- badly injured
- exhausted
- poorly equipped
- badly matched to the mission
- sent without sufficient party support

Death is permanent.

There is no direct game-over because an important adventurer dies.

Instead, death damages the organisation.

If the guild's only B-rank adventurer dies, B-rank quests become extremely difficult to fulfil.

This creates cascading consequences rather than an artificial instant failure.

---

# Injuries

Most bad expeditions should cause injuries rather than deaths.

Examples:

- bruised
- sprained ankle
- broken arm
- concussion
- poisoned
- magical burns
- permanent scar

Injuries have recovery times.

The player may face:

> **7 days remaining in quota period**

while their best fighter has:

> **Broken Arm: 9 days recovery**

The player must adapt.

---

# Guild Quotas

The guild is part of a larger organisation.

Head office establishes periodic expectations.

Example monthly requirements:

- Complete 30 quests
- Earn 1,200 Guild Marks
- Complete 4 C-rank or higher contracts
- Maintain acceptable reputation
- Maintain at least 6 active adventurers

Quotas should create pressure without dictating exactly how the player operates.

Repeated failure can result in:

- warnings
- reduced funding
- fewer prestigious contracts
- management intervention
- eventual branch closure

Branch closure is the primary loss condition.

---

# Recruitment

New adventurers regularly apply.

The player decides who to register.

Applicants may range from obvious talent to questionable choices.

Example:

### Finn

Age: 18

Desired Class: Fighter

Experience:

> "Helped my uncle fight a boar once."

Traits:

- Brave
- Reckless
- Friendly

The player can accept or reject him.

A seemingly terrible applicant might eventually become one of the guild's greatest heroes.

---

# Relationships

Adventurers develop relationships through shared quests.

Examples:

- friendship
- rivalry
- mentorship
- distrust
- loyalty
- romantic relationship

Relationships affect expedition behaviour.

A veteran may protect a rookie.

Friends may fight better together.

Someone may refuse to adventure with a person they distrust.

A veteran could request:

> "Put Lysa on my next assignment. She's inexperienced, but I'll teach her."

This creates an organic way for players to develop rookies.

---

# Adventurer Personality

Traits should influence decisions rather than simply provide numerical modifiers.

### Reckless

More likely to continue fighting when retreat would be safer.

### Cautious

More likely to retreat from unexpected danger.

### Protective

More likely to risk injury protecting another party member.

### Ambitious

Wants difficult quests and rapid promotion.

### Loyal

Less likely to leave the guild.

### Greedy

Prefers high-paying assignments.

Different personalities can therefore turn identical parties into very different risks.

---

# Guild Reputation

The branch develops a reputation.

Possible characteristics include:

**Reliable**

Clients trust the guild with important work.

**Prestigious**

Powerful adventurers want to join.

**Dangerous**

The guild has unusually high casualties.

**Rookie Friendly**

Young adventurers seek training here.

**Elite**

The branch attracts dangerous contracts.

Reputation should emerge primarily from player behaviour rather than being selected directly.

---

# Progression

Progression should primarily expand the player's **options and information**, not merely increase numbers.

Potential unlocks include:

### Better Records

See more detailed adventurer histories.

### Monster Archive

Identify likely threats from quest descriptions.

### Regional Maps

Improve danger estimates.

### Medical Services

Reduce injury recovery.

### Training

Develop inexperienced adventurers between quests.

### Equipment Stores

Allow adventurers to prepare for specialised missions.

### Scouts

Investigate suspicious contracts before assigning a party.

### Larger Guild Licence

Increase roster capacity and unlock higher-rank contracts.

---

# The Reception Desk

The interface should reinforce the fantasy that the player works at the guild.

Rather than presenting a conventional strategy-game dashboard, the primary interface should resemble a functional fantasy workplace.

Possible elements:

- quest notices
- adventurer dossiers
- guild ledger
- promotion forms
- letters from headquarters
- map
- reward envelopes
- stamps
- adventurer portraits

However, usability takes priority over decorative clutter.

The player should never need to hunt around the screen for information.

---

# Tone

The world should balance cozy fantasy bureaucracy with genuine consequences.

One moment:

> Applicant occupation: Wizard  
>   
> Certification?  
>   
> "Technically I'm self-taught."

Later:

> **Expedition overdue: 2 days**

The game should not constantly attempt comedy.

Humour comes naturally from bureaucracy colliding with fantasy adventuring.

Likewise, deaths become meaningful because the world is usually warm rather than relentlessly grim.

---

# Emergent Stories

The game should favour systems capable of producing stories without requiring scripted narratives.

For example:

A nervous F-rank healer joins the guild.

The player pairs her repeatedly with an experienced fighter.

They become friends.

The fighter mentors her.

She eventually reaches C-rank.

During a dangerous expedition, the fighter is killed protecting her.

She survives.

Several years later she becomes an A-rank adventurer and gains the **Protective** trait.

The game never explicitly wrote that story.

The systems produced it.

This is a major long-term design goal.

---

# Prototype Scope

The first playable version should deliberately remain small.

## Include

- One guild branch
- Day progression
- 10Ã¢â‚¬â€œ15 persistent adventurers
- F through C ranks
- Several classes
- Adventurer traits
- Procedurally generated quests
- Quest difficulty
- Party assignment
- Expedition simulation
- Injuries
- Death
- Experience
- Promotion decisions
- Rewards
- Guild reputation
- Monthly quotas
- Basic recruitment
- Save/load

## Do Not Include Initially

- Player-controlled combat
- World exploration
- Detailed town simulation
- Crafting
- Large equipment system
- Complex economy
- S-rank/endgame content
- Hundreds of traits
- elaborate relationship simulation
- story campaign
- multiplayer

The prototype should first prove one thing:

> **Is deciding who to send on a quest interesting?**

---

# Prototype Gameplay Example

The player starts with:

**1 D-rank veteran**  
**2 E-rank adventurers**  
**4 F-rank rookies**

The day's contracts include:

### Slimes in Orchard
Rank F

### Merchant Escort
Rank E

### Missing Hunters
Rank D

### Ruins Investigation
Rank D

The obvious safe approach is sending the veteran on a D-rank quest.

But there are **two** D-rank contracts.

The player could send:

Veteran Ã¢â€ â€™ Missing Hunters

Two E-ranks + talented F-rank Ã¢â€ â€™ Ruins

The second party is underqualified.

However, success could provide enough experience for one E-rank adventurer to become eligible for promotion.

The monthly requirement is:

> **Complete 6 D-rank contracts**

There are nine days remaining.

Suddenly a seemingly simple roster assignment becomes an interesting strategic decision.

That is the heart of **Occupational Hazard**.

---

# Definition of Prototype Success

The prototype is successful when the player:

1. Develops favourite adventurers.
2. Hesitates before assigning someone to a dangerous quest.
3. Sometimes deliberately risks rookies because they need experience.
4. Feels the consequences of injuries and deaths in future scheduling.
5. Has meaningful reasons to promote someone early or hold them back.
6. Cannot solve every day simply by sending the highest-ranked adventurer.
7. Can tell memorable stories about what happened to their guild without those stories being explicitly scripted.

If those behaviours emerge with a small roster and a simple quest simulator, **Occupational Hazard has its core game**.
---

# Implemented Vertical Slice: The First Bronze Licence

For this milestone, the rank vocabulary is **Iron -> Bronze**, superseding the
letter-rank examples above within the playable slice. Three persistent recruits
share a desk with receptionist Elowen. The player reviews twelve authored contracts,
selects a party, dispatches expeditions, advances days, and reads return reports.

Bronze eligibility requires 60 experience and three successful contracts. The
candidate must then complete the two-day Lantern Road Trial alone. Certification
requires the player's explicit APPROVE BRONZE signature in the adventurer dossier.
It unlocks a Bronze commission, increases capability and celebrates an intermediate achievement.

The implemented first-month review closes on day 30 after returning expeditions,
XP and rewards resolve. Its targets are one Bronze certification and one successful
Bronze commission. The saved review records both targets, each recruit's rank, XP
and successes, and closing treasury with net change from the opening 80g.
Unsuccessful reviews explain the missed targets. Visible controls continue into
a labelled sandbox or restart through replacement confirmation. The objective
control reopens the frozen review in sandbox. Late dispatches show the cutoff.

Saved contextual tutorial acknowledgements and skip state introduce selection,
dispatch, day progression, reports, recovery, the trial and explicit approval.
MENU offers HELP to revisit every lesson. Short guidance stays in a desk panel
with outlined next controls; normal play advances steps without closing a modal.

Class suitability, experience, rank, party support and fatigue drive deterministic
outcomes. Healers reduce injury risk. Retreats grant limited experience but no gold;
medical leave and fatigue recover on days spent at the guild. Failed jobs can be
retried, while successful requests leave the board. Distinct class-suitable jobs
provide the experience and successes needed for promotion. Autosaves preserve
the roster, expeditions, reports, completed contracts and certifications.

This milestone deliberately excludes the larger prototype's recruitment, permanent
death, monthly closure quotas, relationship simulation and procedural contracts.
Personality labels currently establish identity; their behavioural simulation remains
future work. The working slice proves assignment, consequences and certification first.

## Prototype Extension: Guild Services

Contract earnings now fund a permanent infirmary (100g), a permanent training
yard (140g), or route preparation (20g for the next dispatch of that contract).
The infirmary doubles medical-leave recovery at home. The yard grants 5 XP per
day to uninjured, fatigue-free Iron adventurers who remain at home, capped at
60 XP; it never grants successful contracts or replaces the field trial.
Scouting contributes a small deterministic preparation advantage and is consumed
on dispatch, including on an expedition that later retreats. Promotion trials
cannot be scouted. Services and unconsumed preparation are saved; older ledgers
start with no purchased services.


## Phase 1 observation follow-up

The first unfamiliar player stopped before the review: the tutorial took focus,
multiple mission returns were hard to find, and completed quests remained on the
board. Guidance now sits alongside usable controls with the next target outlined.
Every report has an inbox entry and saved unread state. Completed ordinary jobs
leave the board; failures remain retryable. Six additional authored Iron jobs
preserve a distinct three-job promotion route for every class without repeat
rewards. Original six-slot saves are extended without changing existing slots.
The trial is a per-candidate appointment and remains for candidates who need it.

The player saw no need for the 100g infirmary in the interrupted session and
estimated the small mission set would take about five minutes. This is evidence
against claiming a 30-minute session, not a measured full playthrough. Facility
prices remain provisional; no facility is mandatory. Further pacing and service
value decisions require a repeat session after the blockers are fixed. Phase 2
has not been broadly implemented: dated arrivals, expiry and stable instance
identities remain on the roadmap.
