package br.com.mili.backend.model;

import jakarta.persistence.*;

import java.time.OffsetDateTime;
import java.util.Objects;
import java.util.UUID;

@Entity
@Table(name = "camera_evidences")
public class Evidence {

    @Id
    @GeneratedValue
    private UUID id;

    @Column(name = "file_path", nullable = false)
    private String filePath;

    @Column(name = "created_at", nullable = false, insertable = false, updatable = false)
    private OffsetDateTime createdAt;

    @Column(name = "camera_id", nullable = false)
    private UUID cameraId;

    @Column(name = "occurrence_id")
    private UUID occurrenceId;

    public Evidence() {
    }

    public UUID getId() {
        return id;
    }

    public void setId(UUID id) {
        this.id = id;
    }

    public String getFilePath() {
        return filePath;
    }

    public void setFilePath(String filePath) {
        this.filePath = filePath;
    }

    public OffsetDateTime getCreatedAt() {
        return createdAt;
    }

    public void setCreatedAt(OffsetDateTime createdAt) {
        this.createdAt = createdAt;
    }

    public UUID getCameraId() {
        return cameraId;
    }

    public void setCameraId(UUID cameraId) {
        this.cameraId = cameraId;
    }

    public UUID getOccurrenceId() {
        return occurrenceId;
    }

    public void setOccurrenceId(UUID occurrenceId) {
        this.occurrenceId = occurrenceId;
    }

    @Override
    public boolean equals(Object o) {
        if (o == null || getClass() != o.getClass()) return false;
        Evidence evidence = (Evidence) o;
        return Objects.equals(getId(), evidence.getId()) && Objects.equals(getFilePath(), evidence.getFilePath()) && Objects.equals(getCreatedAt(), evidence.getCreatedAt()) && Objects.equals(getCameraId(), evidence.getCameraId()) && Objects.equals(getOccurrenceId(), evidence.getOccurrenceId());
    }

    @Override
    public int hashCode() {
        return Objects.hash(getId(), getFilePath(), getCreatedAt(), getCameraId(), getOccurrenceId());
    }
}
