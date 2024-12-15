import { Client } from "https://deno.land/x/postgres@v0.17.0/mod.ts";

interface Verse {
    language: string;
    version?: string | undefined;
    testament: string;
    book: number;
    chapter: number;
    verse: number;
    text: string;
}

const dbClient = new Client({
    hostname: "terraform-20241206002335676200000001.cru2wuokeaez.us-east-1.rds.amazonaws.com",
    database: "postgres",
    user: "postgres",
    password: "bibledbpassword",
    port: 5432,
    tls: {
        enabled: true,
        enforce: true, // ensure this is true once you have the correct CA
        caCertificates: [
          await Deno.readTextFile("./rds-combined-ca-bundle.pem")
        ]
      },
});

async function insertVerse(verse: Verse): Promise<void> {
    const query = `
        INSERT INTO verses (language, version, testament, book, chapter, verse, text)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
    `;
    const values = [
        verse.language,
        verse.version || null,
        verse.testament,
        verse.book,
        verse.chapter,
        verse.verse,
        verse.text,
    ];
    console.log(values);

    try {
        // Pass the query and the array of values as the second argument
        await dbClient.queryObject(query, values);
        console.log(`Inserted verse: ${verse.language} ${verse.testament} ${verse.book}:${verse.chapter}:${verse.verse}`);
    } catch (error) {
        console.error("Error inserting verse:", error);
    }
}


async function processFile(filePath: string): Promise<void> {
    try {
        const fileContent = await Deno.readTextFile(filePath);
        const verses: Verse[] = JSON.parse(fileContent);

        for (const verse of verses) {
            await insertVerse(verse);
        }
    } catch (error) {
        console.error(`Error processing file ${filePath}:`, error);
    }
}

async function main(): Promise<void> {
    await dbClient.connect();

    const processedFolder = "./processed";

    try {
        const folderInfo = await Deno.stat(processedFolder);
        if (!folderInfo.isDirectory) {
            console.error("The /processed path exists but is not a folder.");
            return;
        }
    } catch {
        console.error("The /processed folder does not exist.");
        return;
    }

    for await (const entry of Deno.readDir(processedFolder)) {
        const filePath = `${processedFolder}/${entry.name}`;

        if (entry.isFile && entry.name.endsWith(".json")) {
            console.log(`Processing file: ${entry.name}`);
            await processFile(filePath);
        } else {
            console.warn(`Skipping non-JSON file: ${entry.name}`);
        }
    }

    await dbClient.end();
}

main().catch((error) => console.error(error));