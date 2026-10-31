@@lesson xp-password_tool Password Generator and Strength Meter
@@text
A console tool for the help desk: generate random passwords or Diceware-style passphrases with a cryptographically secure generator, and rate a password's strength by estimated entropy and simple rules (length, character classes, common words).

**You'll practise:** System.Security.Cryptography.RandomNumberGenerator · string building · command-line arguments · classes and static methods

## Milestones

1. **Generate**: `gen 16` makes a 16-character password from upper, lower, digits and symbols.
2. **Passphrase**: `phrase 5` picks 5 words from words.txt, joined by dashes.
3. **Entropy**: estimate bits as length × log2(pool size), and for phrases words × log2(word count).
4. **Rate**: Weak / OK / Strong with reasons (too short, only one character class, a common password).
5. **Never use `System.Random` for passwords**: explain why in a comment.

## You're done when

- The generator only uses RandomNumberGenerator, and ratings match your expectations on a few test passwords.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"Why is System.Random not safe for passwords?"*
- *"How do I pick a random index without modulo bias?"*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project password_tool Password Generator and Strength Meter
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# Password Generator and Strength Meter

A console tool for the help desk: generate random passwords or Diceware-style passphrases with a cryptographically secure generator, and rate a password's strength by estimated entropy and simple rules (length, character classes, common words).

## Milestones

- [ ] 1. Generate: `gen 16` makes a 16-character password from upper, lower, digits and symbols.
- [ ] 2. Passphrase: `phrase 5` picks 5 words from words.txt, joined by dashes.
- [ ] 3. Entropy: estimate bits as length × log2(pool size), and for phrases words × log2(word count).
- [ ] 4. Rate: Weak / OK / Strong with reasons (too short, only one character class, a common password).
- [ ] 5. Never use `System.Random` for passwords: explain why in a comment.

## Done when

- [ ] The generator only uses RandomNumberGenerator, and ratings match your expectations on a few test passwords.
@@file PasswordTool.csproj
<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <OutputType>Exe</OutputType>
    <TargetFramework>net8.0</TargetFramework>
    <ImplicitUsings>enable</ImplicitUsings>
    <Nullable>enable</Nullable>
    <InvariantGlobalization>true</InvariantGlobalization>
    <RollForward>Major</RollForward>
  </PropertyGroup>
</Project>
@@file Program.cs
using System.Security.Cryptography;

// password tool: dotnet run -- gen 16 | phrase 5 | rate "P@ssw0rd"
if (args.Length == 0)
{
    Console.WriteLine("usage: gen N | phrase N | rate PASSWORD");
    return;
}

switch (args[0])
{
    case "gen":
        Console.WriteLine(Passwords.Generate(int.Parse(args[1])));
        break;
    default:
        Console.WriteLine("TODO: phrase and rate");
        break;
}

static class Passwords
{
    const string Pool = "ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz23456789!@#$%&*?";

    public static string Generate(int length)
    {
        // TODO: use RandomNumberGenerator.GetInt32(Pool.Length) for each character
        return new string('x', length);
    }
}
@@file words.txt
apple
river
stone
cloud
maple
tiger
lantern
silver
orbit
kettle
meadow
harbor
pencil
summit
velvet
willow
