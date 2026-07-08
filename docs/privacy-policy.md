# Privacy Policy

**Effective date:** 2026-07-08

This Privacy Policy explains what data Bot Monitor ("the Bot", "we", "us") collects when you use it on Discord, how that data is used, and how you can have it removed.

## 1. Data We Collect

When you use the `/add` command to register a bot for monitoring, we store:

- **Your Discord user ID** — so we know who to notify.
- **The Discord ID of the bot you registered** — so we know which bot to monitor.
- **The Discord ID of the server (guild) the command was run in** — so we can tell whether Bot Monitor, you, and the monitored bot still share a server together (used for the automatic cleanup described in [§4](#4-data-storage--retention)).

We do not collect your username, avatar, email address, message content, or any other personal information. We do not use the `MESSAGE_CONTENT` intent and cannot read the contents of your messages.

## 2. How We Use Presence Data

Bot Monitor uses Discord's privileged **Presence Intent** (`GUILD_PRESENCES`) to receive real-time online/offline status updates for bots present in servers we share. This presence data is processed transiently, in memory, to detect when a monitored bot's status changes — it is **not stored**.

## 3. How We Use Your Data

The Discord user ID, bot ID, and guild ID stored via `/add` are used solely to:

- Determine which users should be notified when a monitored bot goes offline or comes back online.
- Send you a Direct Message (via the `DIRECT_MESSAGES` intent) containing that alert.
- Check whether Bot Monitor, you, and the monitored bot still share a server, so stale entries can be automatically cleaned up (see [§4](#4-data-storage--retention)).

## 4. Data Storage & Retention

Registered entries (bot ID + your user ID) are stored in our database until one of the following happens:

- You remove the entry yourself using the `/remove` command, or
- Bot Monitor is removed from the server the entry was created in, **and** no other server it remains in has both you and the monitored bot present — in that case the entry is automatically deleted. If another shared server does still have both of you present, the entry is kept and silently re-associated with that server instead of being deleted.

Note that this automatic cleanup only runs when Bot Monitor itself is removed from a server. It does not detect you or the monitored bot leaving a shared server independently while Bot Monitor remains — those cases require manual removal via `/remove`.

## 5. Data Sharing

We do not sell, rent, or share your data with any third party. Data is used exclusively to operate the monitoring and alerting features described above.

## 6. Your Rights

You can view what you have registered at any time with `/list`, and delete an entry at any time with `/remove`. If you would like all data associated with your Discord user ID deleted and cannot do so yourself (e.g. you no longer share a server with the Bot), you can request deletion — see [Contact](#8-contact).

## 7. Children's Privacy

Bot Monitor is intended for use in accordance with [Discord's Terms of Service](https://discord.com/terms), which requires users to be at least 13 years old (or the minimum age required in your jurisdiction). We do not knowingly collect data from users who do not meet this requirement.

## 8. Contact

Questions about this policy or requests regarding your data can be raised via [GitHub Issues](https://github.com/i-h-bar/bot-monitor/issues).

## 9. Changes to This Policy

We may update this Privacy Policy from time to time. Material changes will be reflected by updating the effective date above. Continued use of the Bot after changes take effect constitutes acceptance of the revised policy.