# Mailbox MultiversX | Degen Sentinels
Smart Contract for the MultiversX Blockchain with a Mailbox function to receive, storage, and display messages sent by MultiversX accounts.

## NearX Dojo (nxDojo)
This code is part of a weekly challenge to the NearX Dojo (nxDojo), developed by Vini Barbosa to the Degen Sentinels dojo team.

## Endpoints (or calls)
`sendMail` - Sends an email to the contract, which will be stored;
The data must include the parameters `title` and `message`.
Each parameter must come after the `sendMail` call, separated by a `@` and encoded in hexadecimals.

i.e.,
`sendMail@<hexdecimalTitle>@<hexdecimalMessage>`

e.g.,
`sendMail@54657374205469746C65@48656C6C6F20576F726C6421`

This will send the following email -
**Title:** Test Title
**Message:** Hello World!

*More information soon.*


