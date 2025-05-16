let art = ` ████████   █████████  █████████  ████      ██
██      ██  ██     ██  ██         ██ ██     ██
██      ██  ██     ██  ██         ██  ██    ██
██      ██  █████████  █████████  ██   ██   ██
██      ██  ██         ██         ██    ██  ██
██      ██  ██         ██         ██     ██ ██
 ████████   ██         █████████  ██      ████
                             
                        R  E  S  O  L  V  I  U  M
`
let welcomemsg = `Yo! It seems like you are using the DevTools console.\nBut, be careful here! Most attackers use it to do Self-XSS (if u dont know whats that, https://www.cobalt.io/vulnerability-wiki/v5-validation-sanitization/self-reflected-xss).\nSelf-XSS can make so the attacker can act like its you doing it. Be careful, and dont copy paste stuff unless the script is sent by the owner or you know what does the code do!`
console.log(art + "\n" + welcomemsg);