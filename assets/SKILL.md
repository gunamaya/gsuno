---
name: gsuno
description: Write, generate, resume, and download Suno music with the gsuno CLI. Use for songs, lyrics, covers, remasters, stems, voice personas, and MP3/WAV/M4A/MP4 downloads. Discover syntax with gsuno agent-info --command and read gsuno guide prompting before composing.
---

# GSuno

- Start with `gsuno agent-info --command prompt` and `gsuno guide prompting`. Read `gsuno guide songwriting` for lyric structure. Scope discovery to the command you need.
- `gsuno prompt` builds a musical brief offline: BPM, meter, beat unit, groove, voice, delivery, instruments, arrangement, and exclusions. It reports missing choices and returns generation/preview argv. Presets are examples; choose casting for the song.
- `gsuno write --out song.txt` creates editable lyric placeholders. Fill them before generating. `--out` is lyrics only; piped stdout is JSON.
- `gsuno generate --dry-run` validates offline. Use a stable `--request-id UUID` for paid submission and retries. A changed musical request needs a new UUID.
- Generate one pair, download, and audition voice, delivery, rhythm, words, and ending before a large batch. Do not claim an audio quality check from file metadata alone. Leave sliders unset unless intentionally choosing them; 8/92 is not a universal recipe.
- v6 is the default. Use `gsuno models` and `gsuno credits` for current account access. Standard v6 currently costs 10 credits per pair; Max Mode costs more.
- `--download DIR` implies waiting. Interrupted work resumes with `gsuno jobs`, then `gsuno status <ids> --wait --download DIR`. Never resubmit just because polling timed out. Unknown submissions need library inspection.
- Downloads support `--format mp3|wav|m4a|mp4` and `--source auto|studio|library`; `auto` selects the account-supported route. Successful clips include `local_path`.
- `--headless` permits invisible Chrome only. `--no-browser` permits HTTP only; captcha requirements are returned explicitly. Default mode can use an offscreen browser when invisible solving fails.
- Use `gsuno auth --cookie-stdin` or `--jwt-stdin` for supplied secrets. `--token` is a solved captcha, with `--token-provider 1|2`; it is not an auth JWT.
- Machine failures appear as one stderr JSON envelope. Its `details` may include clip IDs and recovery actions. Piped successes use stdout JSON; `agent-info` is a raw manifest.
- `--force` bypasses only the duplicate guard. Literal lyric placeholders use the separate `--allow-placeholders` override. Deletion uses `--confirm` (`--yes` remains an alias).
- Research/priming mode remains available through `gsuno write --mode priming`; read `gsuno guide priming` for its documented scope.
