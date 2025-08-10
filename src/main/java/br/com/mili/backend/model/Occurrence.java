package br.com.mili.backend.model;

import jakarta.persistence.*;

import java.util.Objects;
import java.util.UUID;
import java.time.OffsetDateTime;

@Entity
@Table(name = "occurrences")
public class Occurrence {

    @Id
    @GeneratedValue
    private UUID id;

    @Column(nullable = false)
    private String description;

    @Column(name = "created_at", nullable = false, updatable = false, insertable = false)
    private OffsetDateTime createdAt;

    @Column(name = "updated_at", nullable = false, insertable = false)
    private OffsetDateTime updatedAt;

    @Column(name = "finalized_at", nullable = false, insertable = false)
    private OffsetDateTime finalizedAt;

    public Occurrence() {
    }

    public UUID getId() {
        return id;
    }

    public void setId(UUID id) {
        this.id = id;
    }

    public String getDescription() {
        return description;
    }

    public void setDescription(String description) {
        this.description = description;
    }

    public OffsetDateTime getCreatedAt() {
        return createdAt;
    }

    public void setCreatedAt(OffsetDateTime createdAt) {
        this.createdAt = createdAt;
    }

    public OffsetDateTime getUpdatedAt() {
        return updatedAt;
    }

    public void setUpdatedAt(OffsetDateTime updatedAt) {
        this.updatedAt = updatedAt;
    }

    public OffsetDateTime getFinalizedAt() {
        return finalizedAt;
    }

    public void setFinalizedAt(OffsetDateTime finalizedAt) {
        this.finalizedAt = finalizedAt;
    }

    @Override
    public boolean equals(Object o) {
        if (o == null || getClass() != o.getClass()) return false;
        Occurrence that = (Occurrence) o;
        return Objects.equals(getId(), that.getId()) && Objects.equals(getDescription(), that.getDescription()) && Objects.equals(getCreatedAt(), that.getCreatedAt()) && Objects.equals(getUpdatedAt(), that.getUpdatedAt()) && Objects.equals(getFinalizedAt(), that.getFinalizedAt());
    }

    @Override
    public int hashCode() {
        return Objects.hash(getId(), getDescription(), getCreatedAt(), getUpdatedAt(), getFinalizedAt());
    }
}
