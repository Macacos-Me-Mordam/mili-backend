package br.com.mili.backend.data.dto;

import com.fasterxml.jackson.annotation.JsonIgnore;
import com.fasterxml.jackson.annotation.JsonProperty;

import java.time.OffsetDateTime;
import java.util.Objects;
import java.util.UUID;

public class UserDTO  {

    private static final long serialVersionUID = 1L;
    private UUID id;
    private String name;
    private String email;

    @JsonProperty(access = JsonProperty.Access.WRITE_ONLY)
    private String password;
    private OffsetDateTime createdAt;
    private OffsetDateTime updateAt;

    public UserDTO() {
    }

    public UUID getId() {
        return id;
    }

    public void setId(UUID id) {
        this.id = id;
    }

    public String getName() {
        return name;
    }

    public void setName(String name) {
        this.name = name;
    }

    public String getEmail() {
        return email;
    }

    public void setEmail(String email) {
        this.email = email;
    }

    public OffsetDateTime getCreatedAt() {
        return createdAt;
    }

    public void setCreatedAt(OffsetDateTime createdAt) {
        this.createdAt = createdAt;
    }

    public OffsetDateTime getUpdateAt() {
        return updateAt;
    }

    public void setUpdateAt(OffsetDateTime updateAt) {
        this.updateAt = updateAt;
    }

    public String getPassword() {
        return password;
    }

    public void setPassword(String password) {
        this.password = password;
    }

    @Override
    public boolean equals(Object o) {
        if (o == null || getClass() != o.getClass()) return false;
        UserDTO userDTO = (UserDTO) o;
        return Objects.equals(getId(), userDTO.getId()) && Objects.equals(getName(), userDTO.getName()) && Objects.equals(getEmail(), userDTO.getEmail()) && Objects.equals(getPassword(), userDTO.getPassword()) && Objects.equals(getCreatedAt(), userDTO.getCreatedAt()) && Objects.equals(getUpdateAt(), userDTO.getUpdateAt());
    }

    @Override
    public int hashCode() {
        return Objects.hash(getId(), getName(), getEmail(), getPassword(), getCreatedAt(), getUpdateAt());
    }
}
