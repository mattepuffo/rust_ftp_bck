# RUST FTP BCK

Programma scritto in RUST per la creazione di backup di cartelle su server FTP.

Internamente uso DuckDB per la configurazione e il salvataggio di log in cui memorizzo cosa è stato salvato e quando.

## TODO
- creazione cartelle bck e zip in inizio programma se nn esistono
- db e bck db in sotto cartella
- file zip in sottocartella con nome e data
- pulizia sottocartella zip
- in fase di creazione ftp indicare quale config
- svuotare log
- zip
- upload FTP
- delete server
- delete sync
- delete file zip