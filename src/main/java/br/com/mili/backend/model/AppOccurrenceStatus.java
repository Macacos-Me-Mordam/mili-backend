package br.com.mili.backend.model;

import br.com.mili.backend.data.enums.OccurrenceStatusEnum;
import jakarta.persistence.*;
import org.hibernate.annotations.JdbcType;
import org.hibernate.dialect.PostgreSQLEnumJdbcType;

import java.time.OffsetDateTime;
import java.util.Objects;
import java.util.UUID;

@Entity
@Table(name = "app_occurrence_statuses")
public class AppOccurrenceStatus {

    @Id
    @GeneratedValue
    private UUID id;

    @Column(name = "app_occurrence_id", nullable = false)
    private UUID appOccurrenceId;

    @Enumerated(EnumType.STRING)
    @Column(columnDefinition = "occurrence_status")
    @JdbcType(PostgreSQLEnumJdbcType.class)
    private OccurrenceStatusEnum status;

    @Column(name = "status_date", nullable = false)
    private OffsetDateTime statusDate;

    public AppOccurrenceStatus() {
    }

    public UUID getId() {
        return id;
    }

    public void setId(UUID id) {
        this.id = id;
    }

    public UUID getAppOccurrenceId() {
        return appOccurrenceId;
    }

    public void setAppOccurrenceId(UUID appOccurrenceId) {
        this.appOccurrenceId = appOccurrenceId;
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
        if (o == null || getClass() != o.getClass()) return false;
        AppOccurrenceStatus that = (AppOccurrenceStatus) o;
        return Objects.equals(getId(), that.getId()) && Objects.equals(getAppOccurrenceId(), that.getAppOccurrenceId()) && getStatus() == that.getStatus() && Objects.equals(getStatusDate(), that.getStatusDate());
    }

    @Override
    public int hashCode() {
        return Objects.hash(getId(), getAppOccurrenceId(), getStatus(), getStatusDate());
    }
}