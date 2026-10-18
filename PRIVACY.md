# Privacy Policy

**Application:** aaai — audit for asset integrity
**Publisher:** nabbisen
**Effective:** 2026-10-18

## 1. Summary

aaai runs entirely on your device. It has no network capability, collects
nothing, and sends nothing anywhere. The only data it keeps is the data you
create by using it, stored in files on your own computer.

## 2. No network access

**aaai does not contain any network capability.** It has no HTTP client, no
socket code, and no telemetry, analytics, update check, or crash reporting of
any kind. There is no setting that enables network access, because there is no
code to enable.

The SARIF report format includes two fixed URLs in its output — a schema
identifier and a link to the project's repository. These are written into the
report file as text. aaai never retrieves them.

## 3. What aaai stores on your device

aaai is a tool for recording decisions, so it necessarily writes files. All of
them stay on your device. Nothing is transmitted.

**In your operating system's configuration directory:**

| Platform | Location |
|---|---|
| Windows | `%APPDATA%\aaai\` |
| macOS | `~/Library/Application Support/aaai/` |
| Linux | `$XDG_CONFIG_HOME/aaai/` (default `~/.config/aaai/`) |

- `prefs.yaml` — your display preferences, such as the colour theme.
- `profiles.yaml` — named audit profiles you have saved. **Each profile
  contains the filesystem paths of the folders you compared**, which on most
  systems include your user account name, along with the time you last used it.

**Wherever you choose to put them:**

- Audit definition files and the reasons you write in them.
- Review reports you export, in Markdown, JSON, HTML or SARIF.

You can delete any of these files at any time. Deleting the configuration
directory returns aaai to its initial state.

## 4. What aaai reads

aaai reads the folders you select for comparison, and the configuration and
definition files described above. It reads nothing else, and it reads only when
you ask it to compare, open, or audit something.

## 5. No personal information is collected

aaai does not collect, request, or transmit personal information. There are no
accounts, no sign-in, no identifiers, and no usage tracking.

Note that file paths you save in a profile may contain your user account name.
That information is written only to your own device, in the file described in
§3, and is never sent anywhere.

## 6. System access

The Microsoft Store package declares a single capability, `runFullTrust`. A
desktop application needs it to read and write files outside its own package —
which is what aaai does when it compares the folders you select and saves the
files you ask it to save. It is not used for anything else, and it grants no
network access.

## 7. Distribution platforms

If you installed aaai from the Microsoft Store, Microsoft may collect its own
information about the installation under their privacy policy. That is
Microsoft's collection, not aaai's, and aaai neither sees nor receives it. The
same applies to any other platform you install from.

## 8. Children's privacy

aaai collects no information from anyone, including children.

## 9. Changes to this policy

Changes are published in this file in the project repository, and the effective
date above is updated. The file's revision history is public.

## 10. Contact

Open an issue at <https://github.com/nabbisen/aaai/issues>.
