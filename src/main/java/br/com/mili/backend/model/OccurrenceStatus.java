package br.com.mili.backend.model;

import br.com.mili.backend.data.enums.OccurrenceStatusEnum;
import jakarta.persistence.*;
import org.hibernate.annotations.JdbcType;
import org.hibernate.dialect.PostgreSQLEnumJdbcType;

import java.time.OffsetDateTime;
import java.util.Objects;
import java.util.UUID;

@Entity
@Table(name = "occurrence_statuses")
public class OccurrenceStatus {

    @Id
    @GeneratedValue
    private UUID id;

    @Column(name = "occurrence_id", nullable = false)
    private UUID occurrenceId;

    @Enumerated(EnumType.STRING)
    @Column(columnDefinition = "occurrence_status")
    @JdbcType(PostgreSQLEnumJdbcType.class)
    private OccurrenceStatusEnum status;

    @Column(name = "status_date", nullable = false)
    private OffsetDateTime statusDate;

    public OccurrenceStatus() {
    }

    public UUID getId() {
        return id;
    }

    public void setId(UUID id) {
        this.id = id;
    }

    public UUID getOccurrenceId() {
        return occurrenceId;
    }

    public void setOccurrenceId(UUID occurrenceId) {
        this.occurrenceId = occurrenceId;
    }

    public OccurrenceStatusEnum getStatus() {
        return status;
    }

    public void setStatus(OccurrenceStatusEnum status) {
        this.status = status;
    }

    public OffsetDateTime getStatusDate() {
        return statusDate;
    }

    public void setStatusDate(OffsetDateTime statusDate) {
        this.statusDate = statusDate;
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        OccurrenceStatus that = (OccurrenceStatus) o;
        return Objects.equals(id, that.id);
    }

    @Override
    public int hashCode() {
        return Objects.hash(id);
    }
}