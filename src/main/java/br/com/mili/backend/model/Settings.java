package br.com.mili.backend.model;

import jakarta.persistence.Column;
import jakarta.persistence.Entity;
import jakarta.persistence.Id;
import jakarta.persistence.Table;

import java.time.OffsetDateTime;
import java.util.Objects;

@Entity
@Table(name = "app_settings")
public class Settings {

    @Id
    @Column(name = "key")
    private String key;

    @Column(name = "value", nullable = false)
    private String value;

    @Column(name = "updated_at", nullable = false, insertable = false, updatable = false)
    private OffsetDateTime updatedAt;

    protected Settings() {}

    public Settings(String key, String value) {
        this.key = key;
        this.value = value;
    }

    public String getKey() {
        return key;
    }

    public void setKey(String key) {
        this.key = key;
    }

    public String getValue() {
        return value;
    }

    public void setValue(String value) {
        this.value = value;
    }

    public OffsetDateTime getUpdatedAt() {
        return updatedAt;
    }

    public void setUpdatedAt(OffsetDateTime updatedAt) {
        this.updatedAt = updatedAt;
    }

    @Override
    public boolean equals(Object o) {
        if (o == null || getClass() != o.getClass()) return false;
        Settings settings = (Settings) o;
        return Objects.equals(getKey(), settings.getKey()) && Objects.equals(getValue(), settings.getValue()) && Objects.equals(getUpdatedAt(), settings.getUpdatedAt());
    }

    @Override
    public int hashCode() {
        return Objects.hash(getKey(), getValue(), getUpdatedAt());
    }
}
