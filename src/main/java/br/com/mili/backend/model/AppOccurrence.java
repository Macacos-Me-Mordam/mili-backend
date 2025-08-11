package br.com.mili.backend.model;

import br.com.mili.backend.data.enums.FrequencyTypeEnum;
import jakarta.persistence.*;
import org.hibernate.annotations.JdbcType;
import org.hibernate.dialect.PostgreSQLEnumJdbcType;

import java.time.OffsetDateTime;
import java.util.List;
import java.util.Objects;
import java.util.UUID;

@Entity
@Table(name = "app_occurrence")
public class AppOccurrence {

    @Id
    @GeneratedValue
    private UUID id;

    @Column(name = "photo_url", nullable = false)
    private String photoUrl;

    @Column(nullable = false)
    private String description;

    @Column(nullable = false)
    private String address;

    @Enumerated(EnumType.STRING)
    @Column(nullable = false, columnDefinition = "frequency_type")
    @JdbcType(PostgreSQLEnumJdbcType.class)
    private FrequencyTypeEnum frequency;

    @Column(name = "created_at", nullable = false, updatable = false, insertable = false)
    private OffsetDateTime createdAt;

    @Column(name = "updated_at", nullable = false, insertable = false)
    private OffsetDateTime updatedAt;

    @Column(name = "finalized_at", nullable = false, insertable = false)
    private OffsetDateTime finalizedAt;

    public AppOccurrence() {
    }

    public UUID getId() {
        return id;
    }

    public void setId(UUID id) {
        this.id = id;
    }

    public String getPhotoUrl() {
        return photoUrl;
    }

    public void setPhotoUrl(String photoUrl) {
        this.photoUrl = photoUrl;
    }

    public String getDescription() {
        return description;
    }

    public void setDescription(String description) {
        this.description = description;
    }

    public String getAddress() {
        return address;
    }

    public void setAddress(String address) {
        this.address = address;
    }

    public FrequencyTypeEnum getFrequency() {
        return frequency;
    }

    public void setFrequency(FrequencyTypeEnum frequency) {
        this.frequency = frequency;
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
        AppOccurrence that = (AppOccurrence) o;
        return Objects.equals(getId(), that.getId()) && Objects.equals(getPhotoUrl(), that.getPhotoUrl()) && Objects.equals(getDescription(), that.getDescription()) && Objects.equals(getAddress(), that.getAddress()) && getFrequency() == that.getFrequency() && Objects.equals(getCreatedAt(), that.getCreatedAt()) && Objects.equals(getUpdatedAt(), that.getUpdatedAt()) && Objects.equals(getFinalizedAt(), that.getFinalizedAt());
    }

    @Override
    public int hashCode() {
        return Objects.hash(getId(), getPhotoUrl(), getDescription(), getAddress(), getFrequency(), getCreatedAt(), getUpdatedAt(), getFinalizedAt());
    }
}
