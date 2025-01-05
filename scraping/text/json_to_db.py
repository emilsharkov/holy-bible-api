import json
from dotenv import load_dotenv
import os
import psycopg2
from psycopg2.extras import execute_values

# Load environment variables
load_dotenv('C:/Users/emosh/vscode/bible-api/db/postgres/.env')

# Database configuration
hostname = os.getenv("HOST")
user = os.getenv("USER")
password = os.getenv("PASSWORD")
database = os.getenv("DATABASE")
port = os.getenv("PORT")

# Establish database connection
connection = psycopg2.connect(
    host=hostname,
    user=user,
    password=password,
    dbname=database,
    port=port
)
cursor = connection.cursor()

# Directory containing JSON files
input_directory = "json"
for filename in os.listdir(input_directory):
    if filename.endswith(".json"):
        try:
            # Parse filename for language and version
            parts = filename[:-5].split("_")
            language = parts[0]
            version = parts[1] if len(parts) > 1 else None

            # Check if language and version exist in the database
            if version is None:
                cursor.execute(
                    "SELECT bible_id FROM bibles WHERE language = %s AND version IS NULL;",
                    (language,)
                )
            else:
                cursor.execute(
                    "SELECT bible_id FROM bibles WHERE language = %s AND version = %s;",
                    (language, version)
                )

            result = cursor.fetchone()

            if result:
                bible_id = result[0]
                print(f"Bible already exists with ID: {bible_id} for file {filename}")
                continue
            else:
                # Insert new Bible record if not found
                cursor.execute(
                    "INSERT INTO bibles (language, version) VALUES (%s, %s) RETURNING bible_id;",
                    (language, version)
                )
                bible_id = cursor.fetchone()[0]
                print(f"Inserted Bible with ID: {bible_id} for file {filename}")

            # Load JSON data
            input_file = os.path.join(input_directory, filename)
            with open(input_file, "r", encoding="utf-8") as f:
                verses_data = json.load(f)

            # Prepare verses for bulk insertion
            verses = [
                (bible_id, item["book"], item["chapter"], item["verse"], item["text"])
                for item in verses_data
            ]

            # Bulk insert verses
            execute_values(
                cursor,
                "INSERT INTO verses (bible_id, book, chapter, verse, text) VALUES %s",
                verses
            )

            connection.commit()
            print(f"Inserted {len(verses)} verses from {filename} successfully!")

        except Exception as e:
            print(f"Error processing file {filename}: {e}")
            connection.rollback()

# Close the cursor and connection
cursor.close()
connection.close()

print("All files processed successfully!")
