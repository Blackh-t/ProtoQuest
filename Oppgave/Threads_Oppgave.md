### Oppgave: Delt Minne vs Isolerte Minne i Flertrådede Systemer

#### Beskrivelse:
I flertrådede systemer er det vanlig å velge mellom å bruke **delt minne** eller **isolerte minne** for å håndtere data mellom tråder. Delt minne gir trådene tilgang til felles data, mens isolerte minne (typisk brukt i prosesser) sikrer at hver tråd har sitt eget minneområde, og de må bruke spesifikke mekanismer for kommunikasjon (som meldinger eller felles områder).

I denne oppgaven skal du utforske hvordan disse to tilnærmingene påvirker ytelsen i et system som kjører parallelt med flere tråder som utfører forskjellige oppgaver.

#### Del 1: Teoretisk Bakgrunn
1. Beskriv forskjellen mellom delt minne og isolert minne.
2. Forklar hvordan **cache eviction** kan påvirkes av delt minne og isolert minne.
3. Diskuter fordeler og ulemper ved delt minne i flertrådede systemer, spesielt med tanke på **cache-konsistens** og **ytelse**.
4. Drøft hvordan isolerte minne (som i prosesser) kan bidra til bedre minnehierarkihåndtering og redusert risiko for **cache evictions**.

#### Del 2: Eksperimentell Undersøkelse
1. Implementer et program i et flerkjernersystem som simulerer **delt minne** og **isolert minne**.
2. Utfør ytelsestester for begge tilnærmingene og sammenlign:
   - Kjør flere tråder som utfører uavhengige beregninger (orthogonale oppgaver) som ikke deler data.
   - Kjør flere tråder som deler data for å se hvordan delt minne påvirker ytelsen.
   - Mål tiden for oppgaver og observer cache-hits og cache-misses.
3. Evaluer resultatene: Når gir delt minne bedre ytelse, og når gir isolert minne bedre ytelse?

#### Del 3: Konklusjon
1. Basert på dine eksperimenter, oppsummer når du vil bruke delt minne vs isolert minne i et flerkjernersystem.
2. Diskuter hvordan valg av minnearkitektur påvirker både ytelse og strømsparing i applikasjoner som krever høy parallellisering.

---

Dette gir en god balanse mellom teoretisk forståelse og praktisk anvendelse, samtidig som du får muligheten til å teste forskjellige tilnærminger og forstå hvordan de påvirker ytelsen i ulike scenarier.