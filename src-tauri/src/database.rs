// Quantum Energy Mobile - Offline-First Database Module
// SQLite-based local storage for quantum energy data

use rusqlite::{Connection, Result, params};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumEnergyRecord {
    pub id: Option<i64>,
    pub frequency: f64,
    pub amplitude: f64,
    pub phase: f64,
    pub coherence: f64,
    pub entanglement_factor: f64,
    pub timestamp: i64,
    pub synced: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorDataRecord {
    pub id: Option<i64>,
    pub battery_level: f64,
    pub temperature: f64,
    pub magnetic_x: f64,
    pub magnetic_y: f64,
    pub magnetic_z: f64,
    pub acceleration_x: f64,
    pub acceleration_y: f64,
    pub acceleration_z: f64,
    pub light_intensity: f64,
    pub timestamp: i64,
    pub synced: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolarStormRecord {
    pub id: Option<i64>,
    pub severity: String,
    pub impact_level: f64,
    pub estimated_arrival: i64,
    pub protective_measures: String,
    pub timestamp: i64,
    pub acknowledged: bool,
}

pub struct QuantumDatabase {
    conn: Arc<Mutex<Connection>>,
}

impl QuantumDatabase {
    pub fn new(db_path: PathBuf) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        
        // Create tables if they don't exist
        conn.execute(
            "CREATE TABLE IF NOT EXISTS quantum_energy (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                frequency REAL NOT NULL,
                amplitude REAL NOT NULL,
                phase REAL NOT NULL,
                coherence REAL NOT NULL,
                entanglement_factor REAL NOT NULL,
                timestamp INTEGER NOT NULL,
                synced INTEGER DEFAULT 0
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS sensor_data (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                battery_level REAL NOT NULL,
                temperature REAL NOT NULL,
                magnetic_x REAL NOT NULL,
                magnetic_y REAL NOT NULL,
                magnetic_z REAL NOT NULL,
                acceleration_x REAL NOT NULL,
                acceleration_y REAL NOT NULL,
                acceleration_z REAL NOT NULL,
                light_intensity REAL NOT NULL,
                timestamp INTEGER NOT NULL,
                synced INTEGER DEFAULT 0
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS solar_storms (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                severity TEXT NOT NULL,
                impact_level REAL NOT NULL,
                estimated_arrival INTEGER NOT NULL,
                protective_measures TEXT NOT NULL,
                timestamp INTEGER NOT NULL,
                acknowledged INTEGER DEFAULT 0
            )",
            [],
        )?;

        // Create indexes for better query performance
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_quantum_timestamp ON quantum_energy(timestamp)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_sensor_timestamp ON sensor_data(timestamp)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_storm_timestamp ON solar_storms(timestamp)",
            [],
        )?;

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    // Quantum Energy Operations
    pub fn save_quantum_state(&self, record: &QuantumEnergyRecord) -> Result<i64> {
        let conn = self.conn.lock().map_err(|_| rusqlite::Error::InvalidQuery)?;
        conn.execute(
            "INSERT INTO quantum_energy (frequency, amplitude, phase, coherence, entanglement_factor, timestamp, synced) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                record.frequency,
                record.amplitude,
                record.phase,
                record.coherence,
                record.entanglement_factor,
                record.timestamp,
                record.synced as i32
            ],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn get_quantum_history(&self, limit: i64) -> Result<Vec<QuantumEnergyRecord>> {
        let conn = self.conn.lock().map_err(|_| rusqlite::Error::InvalidQuery)?;
        let mut stmt = conn.prepare(
            "SELECT id, frequency, amplitude, phase, coherence, entanglement_factor, timestamp, synced FROM quantum_energy ORDER BY timestamp DESC LIMIT ?1"
        )?;

        let records = stmt.query_map(params![limit], |row| {
            Ok(QuantumEnergyRecord {
                id: Some(row.get(0)?),
                frequency: row.get(1)?,
                amplitude: row.get(2)?,
                phase: row.get(3)?,
                coherence: row.get(4)?,
                entanglement_factor: row.get(5)?,
                timestamp: row.get(6)?,
                synced: row.get::<_, i32>(7)? != 0,
            })
        })?;

        let mut result = Vec::new();
        for record in records {
            result.push(record?);
        }
        Ok(result)
    }

    pub fn get_unsynced_quantum_states(&self) -> Result<Vec<QuantumEnergyRecord>> {
        let conn = self.conn.lock().map_err(|_| rusqlite::Error::InvalidQuery)?;
        let mut stmt = conn.prepare(
            "SELECT id, frequency, amplitude, phase, coherence, entanglement_factor, timestamp, synced FROM quantum_energy WHERE synced = 0 ORDER BY timestamp ASC"
        )?;

        let records = stmt.query_map([], |row| {
            Ok(QuantumEnergyRecord {
                id: Some(row.get(0)?),
                frequency: row.get(1)?,
                amplitude: row.get(2)?,
                phase: row.get(3)?,
                coherence: row.get(4)?,
                entanglement_factor: row.get(5)?,
                timestamp: row.get(6)?,
                synced: row.get::<_, i32>(7)? != 0,
            })
        })?;

        let mut result = Vec::new();
        for record in records {
            result.push(record?);
        }
        Ok(result)
    }

    pub fn mark_quantum_synced(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().map_err(|_| rusqlite::Error::InvalidQuery)?;
        conn.execute(
            "UPDATE quantum_energy SET synced = 1 WHERE id = ?1",
            params![id],
        )?;
        Ok(())
    }

    // Sensor Data Operations
    pub fn save_sensor_data(&self, record: &SensorDataRecord) -> Result<i64> {
        let conn = self.conn.lock().map_err(|_| rusqlite::Error::InvalidQuery)?;
        conn.execute(
            "INSERT INTO sensor_data (battery_level, temperature, magnetic_x, magnetic_y, magnetic_z, acceleration_x, acceleration_y, acceleration_z, light_intensity, timestamp, synced) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                record.battery_level,
                record.temperature,
                record.magnetic_x,
                record.magnetic_y,
                record.magnetic_z,
                record.acceleration_x,
                record.acceleration_y,
                record.acceleration_z,
                record.light_intensity,
                record.timestamp,
                record.synced as i32
            ],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn get_sensor_history(&self, limit: i64) -> Result<Vec<SensorDataRecord>> {
        let conn = self.conn.lock().map_err(|_| rusqlite::Error::InvalidQuery)?;
        let mut stmt = conn.prepare(
            "SELECT id, battery_level, temperature, magnetic_x, magnetic_y, magnetic_z, acceleration_x, acceleration_y, acceleration_z, light_intensity, timestamp, synced FROM sensor_data ORDER BY timestamp DESC LIMIT ?1"
        )?;

        let records = stmt.query_map(params![limit], |row| {
            Ok(SensorDataRecord {
                id: Some(row.get(0)?),
                battery_level: row.get(1)?,
                temperature: row.get(2)?,
                magnetic_x: row.get(3)?,
                magnetic_y: row.get(4)?,
                magnetic_z: row.get(5)?,
                acceleration_x: row.get(6)?,
                acceleration_y: row.get(7)?,
                acceleration_z: row.get(8)?,
                light_intensity: row.get(9)?,
                timestamp: row.get(10)?,
                synced: row.get::<_, i32>(11)? != 0,
            })
        })?;

        let mut result = Vec::new();
        for record in records {
            result.push(record?);
        }
        Ok(result)
    }

    pub fn get_unsynced_sensor_data(&self) -> Result<Vec<SensorDataRecord>> {
        let conn = self.conn.lock().map_err(|_| rusqlite::Error::InvalidQuery)?;
        let mut stmt = conn.prepare(
            "SELECT id, battery_level, temperature, magnetic_x, magnetic_y, magnetic_z, acceleration_x, acceleration_y, acceleration_z, light_intensity, timestamp, synced FROM sensor_data WHERE synced = 0 ORDER BY timestamp ASC"
        )?;

        let records = stmt.query_map([], |row| {
            Ok(SensorDataRecord {
                id: Some(row.get(0)?),
                battery_level: row.get(1)?,
                temperature: row.get(2)?,
                magnetic_x: row.get(3)?,
                magnetic_y: row.get(4)?,
                magnetic_z: row.get(5)?,
                acceleration_x: row.get(6)?,
                acceleration_y: row.get(7)?,
                acceleration_z: row.get(8)?,
                light_intensity: row.get(9)?,
                timestamp: row.get(10)?,
                synced: row.get::<_, i32>(11)? != 0,
            })
        })?;

        let mut result = Vec::new();
        for record in records {
            result.push(record?);
        }
        Ok(result)
    }

    pub fn mark_sensor_synced(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().map_err(|_| rusqlite::Error::InvalidQuery)?;
        conn.execute(
            "UPDATE sensor_data SET synced = 1 WHERE id = ?1",
            params![id],
        )?;
        Ok(())
    }

    // Solar Storm Operations
    pub fn save_solar_storm(&self, record: &SolarStormRecord) -> Result<i64> {
        let conn = self.conn.lock().map_err(|_| rusqlite::Error::InvalidQuery)?;
        conn.execute(
            "INSERT INTO solar_storms (severity, impact_level, estimated_arrival, protective_measures, timestamp, acknowledged) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                record.severity,
                record.impact_level,
                record.estimated_arrival,
                record.protective_measures,
                record.timestamp,
                record.acknowledged as i32
            ],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn get_solar_storms(&self, limit: i64) -> Result<Vec<SolarStormRecord>> {
        let conn = self.conn.lock().map_err(|_| rusqlite::Error::InvalidQuery)?;
        let mut stmt = conn.prepare(
            "SELECT id, severity, impact_level, estimated_arrival, protective_measures, timestamp, acknowledged FROM solar_storms ORDER BY timestamp DESC LIMIT ?1"
        )?;

        let records = stmt.query_map(params![limit], |row| {
            Ok(SolarStormRecord {
                id: Some(row.get(0)?),
                severity: row.get(1)?,
                impact_level: row.get(2)?,
                estimated_arrival: row.get(3)?,
                protective_measures: row.get(4)?,
                timestamp: row.get(5)?,
                acknowledged: row.get::<_, i32>(6)? != 0,
            })
        })?;

        let mut result = Vec::new();
        for record in records {
            result.push(record?);
        }
        Ok(result)
    }

    pub fn acknowledge_solar_storm(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().map_err(|_| rusqlite::Error::InvalidQuery)?;
        conn.execute(
            "UPDATE solar_storms SET acknowledged = 1 WHERE id = ?1",
            params![id],
        )?;
        Ok(())
    }

    // Statistics and Analytics
    pub fn get_average_coherence(&self, hours: i64) -> Result<f64> {
        let conn = self.conn.lock().map_err(|_| rusqlite::Error::InvalidQuery)?;
        let cutoff = chrono::Utc::now().timestamp() - (hours * 3600);
        
        let avg: f64 = conn.query_row(
            "SELECT AVG(coherence) FROM quantum_energy WHERE timestamp > ?1",
            params![cutoff],
            |row| row.get(0),
        )?;
        
        Ok(avg)
    }

    pub fn get_sensor_statistics(&self, hours: i64) -> Result<(f64, f64, f64)> {
        let conn = self.conn.lock().map_err(|_| rusqlite::Error::InvalidQuery)?;
        let cutoff = chrono::Utc::now().timestamp() - (hours * 3600);
        
        let (avg_temp, avg_battery, avg_light): (f64, f64, f64) = conn.query_row(
            "SELECT AVG(temperature), AVG(battery_level), AVG(light_intensity) FROM sensor_data WHERE timestamp > ?1",
            params![cutoff],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )?;
        
        Ok((avg_temp, avg_battery, avg_light))
    }

    // Cleanup old data
    pub fn cleanup_old_data(&self, days: i64) -> Result<(usize, usize, usize)> {
        let conn = self.conn.lock().map_err(|_| rusqlite::Error::InvalidQuery)?;
        let cutoff = chrono::Utc::now().timestamp() - (days * 86400);
        
        let quantum_deleted = conn.execute(
            "DELETE FROM quantum_energy WHERE timestamp < ?1 AND synced = 1",
            params![cutoff],
        )?;
        
        let sensor_deleted = conn.execute(
            "DELETE FROM sensor_data WHERE timestamp < ?1 AND synced = 1",
            params![cutoff],
        )?;
        
        let storm_deleted = conn.execute(
            "DELETE FROM solar_storms WHERE timestamp < ?1 AND acknowledged = 1",
            params![cutoff],
        )?;
        
        Ok((quantum_deleted, sensor_deleted, storm_deleted))
    }

    // Database maintenance
    pub fn vacuum(&self) -> Result<()> {
        let conn = self.conn.lock().map_err(|_| rusqlite::Error::InvalidQuery)?;
        conn.execute("VACUUM", [])?;
        Ok(())
    }

    pub fn get_database_size(&self) -> Result<i64> {
        let conn = self.conn.lock().map_err(|_| rusqlite::Error::InvalidQuery)?;
        let size: i64 = conn.query_row(
            "SELECT page_count * page_size FROM pragma_page_count(), pragma_page_size()",
            [],
            |row| Ok(row.get::<_, i64>(0)? * row.get::<_, i64>(1)?),
        )?;
        Ok(size)
    }
}
