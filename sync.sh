rsync -arvuzc garage_pi.local:/home/pi/garage_rfid/migrations/ ./migrations/
rsync -arvuzc garage_pi.local:/home/pi/garage_rfid/src/schema.rs ./src/schema.rs
rsync -arvuzc garage_pi.local:/home/pi/garage_rfid/Cargo.lock ./Cargo.lock
rsync -arvuzc --delete garage_pi.local:/home/pi/garage_rfid/logs/ ./logs/
rsync -arvzc --delete --exclude garage_rfid.sqlite3 --exclude target/  --exclude .git/ ./ garage_pi.local:/home/pi/garage_rfid/
